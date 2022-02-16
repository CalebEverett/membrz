use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::state::{Collection, Creator, DataV2, UseMethod, Uses};

declare_id!("GMpDFEBcv4M6hhNrKq9FcVF5NGtvvedjKZKG38T7opdH");

#[program]
pub mod nftfactory {
    use super::*;

    pub fn create_user(_ctx: Context<CreateUser>) -> ProgramResult {
        msg!("Create user");

        Ok(())
    }

    pub fn create_group(ctx: Context<CreateGroup>, _group_seed: Pubkey) -> ProgramResult {
        msg!("Create group");

        ctx.accounts.group.owner = ctx.accounts.payer.key();
        ctx.accounts.group.users.push(ctx.accounts.payer.key());
        ctx.accounts.user.groups.push(ctx.accounts.group.key());
        Ok(())
    }

    pub fn create_master_edition(
        ctx: Context<CreateMasterEdition>,
        data: AnchorDataV2,
        is_mutable: bool,
        auth_bump: u8,
    ) -> ProgramResult {
        msg!("Create master edition");

        let mint_to_ctx = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let auth_seeds = [
            "pda".as_bytes(),
            &program::Nftfactory::id().to_bytes(),
            &[auth_bump],
        ];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                mint_to_ctx,
                &[&auth_seeds],
            ),
            1,
        )?;

        msg!(&ctx
            .accounts
            .metadata_program
            .to_account_info()
            .key
            .to_string());

        let create_metadata_ctx = CreateMetaDataAccountsV2 {
            payer: ctx.accounts.payer.clone(),
            metadata_account: ctx.accounts.metadata_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        create_metadata_accounts_v2(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                create_metadata_ctx,
                &[&auth_seeds],
            ),
            false,
            is_mutable,
            data.into(),
        )?;
        msg!("finito");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(init, payer = payer, seeds = [payer.key.as_ref()], bump, space = User::LEN)]
    account: Account<'info, User>,
    system_program: Program<'info, System>,
}

#[account]
pub struct User {
    groups: Vec<Pubkey>,
}

impl User {
    const LEN: usize = 8 + 4 + 5 * 32;
}

#[derive(Accounts)]
#[instruction(_group_seed: Pubkey)]
pub struct CreateGroup<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [payer.key.as_ref()], bump)]
    pub user: Account<'info, User>,
    #[account(init, payer = payer, seeds = [_group_seed.as_ref()], bump, space = Group::LEN)]
    pub group: Account<'info, Group>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Group {
    owner: Pubkey,
    users: Vec<Pubkey>,
}

impl Group {
    const LEN: usize = 8 + 32 + 4 + 5 * 32;
}

#[derive(Accounts)]
pub struct CreateMasterEdition<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: AccountInfo<'info>,
    #[account(init, payer = payer, mint::decimals = 0, mint::authority =  authority, mint::freeze_authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = payer, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,
    pub metadata_program: Program<'info, TokenMetadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct AnchorDataV2 {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<AnchorCreator>>,
    /// Collection
    pub collection: Option<AnchorCollection>,
    /// Uses
    pub uses: Option<AnchorUses>,
}

impl From<AnchorDataV2> for DataV2 {
    fn from(item: AnchorDataV2) -> Self {
        DataV2 {
            name: item.name,
            symbol: item.symbol,
            uri: item.uri,
            seller_fee_basis_points: item.seller_fee_basis_points,
            creators: item
                .creators
                .map(|a| a.into_iter().map(|v| v.into()).collect()),
            collection: item.collection.map(|v| v.into()),
            uses: item.uses.map(|v| v.into()),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct AnchorCreator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

impl From<AnchorCreator> for Creator {
    fn from(item: AnchorCreator) -> Self {
        Creator {
            address: item.address,
            verified: item.verified,
            share: item.share,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum AnchorUseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct AnchorUses {
    pub use_method: AnchorUseMethod,
    pub remaining: u64,
    pub total: u64,
}

impl From<AnchorUses> for Uses {
    fn from(item: AnchorUses) -> Self {
        Uses {
            use_method: item.use_method.into(),
            remaining: item.remaining,
            total: item.total,
        }
    }
}

impl From<AnchorUseMethod> for UseMethod {
    fn from(item: AnchorUseMethod) -> Self {
        match item {
            AnchorUseMethod::Burn => UseMethod::Burn,
            AnchorUseMethod::Multiple => UseMethod::Burn,
            AnchorUseMethod::Single => UseMethod::Burn,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct AnchorCollection {
    pub verified: bool,
    pub key: Pubkey,
}

impl From<AnchorCollection> for Collection {
    fn from(item: AnchorCollection) -> Self {
        Collection {
            verified: item.verified,
            key: item.key,
        }
    }
}

pub fn create_metadata_accounts_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetaDataAccountsV2<'info>>,
    update_authority_is_signer: bool,
    is_mutable: bool,
    data: DataV2,
) -> ProgramResult {
    let ix = mpl_token_metadata::instruction::create_metadata_accounts_v2(
        mpl_token_metadata::ID.clone(),
        ctx.accounts.metadata_account.key.clone(),
        ctx.accounts.mint.key.clone(),
        ctx.accounts.authority.key.clone(),
        ctx.accounts.payer.key.clone(),
        ctx.accounts.authority.key.clone(),
        data.name,
        data.symbol,
        data.uri,
        data.creators,
        data.seller_fee_basis_points,
        update_authority_is_signer,
        is_mutable,
        data.collection,
        data.uses,
    );
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata_account,
            ctx.accounts.mint,
            ctx.accounts.authority.clone(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.authority,
            ctx.accounts.rent,
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct CreateMetaDataAccountsV2<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub metadata_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}

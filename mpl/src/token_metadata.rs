use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::borsh::try_from_slice_unchecked;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::pubkey::Pubkey;
pub use mpl_token_metadata::state::{Collection, Creator, DataV2, UseMethod, Uses};
use std::ops::Deref;

pub use mpl_token_metadata::ID;

#[allow(clippy::too_many_arguments)]
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
        ctx.accounts.mint_authority.key.clone(),
        ctx.accounts.payer.key.clone(),
        ctx.accounts.update_authority.key.clone(),
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
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata_account.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.update_authority.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct CreateMetaDataAccountsV2<'info> {
    pub payer: AccountInfo<'info>,
    pub metadata_account: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct MetadataAccount(mpl_token_metadata::state::Metadata);

impl MetadataAccount {
    pub const LEN: usize = mpl_token_metadata::state::MAX_METADATA_LEN;
}

impl anchor_lang::AccountDeserialize for MetadataAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        try_from_slice_unchecked(buf)
            .map(MetadataAccount)
            .map_err(|_| {
                ProgramError::BorshIoError("Failed to deserialize metadata account".to_string())
            })
    }
}

impl anchor_lang::AccountSerialize for MetadataAccount {}

#[derive(Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        ID
    }
}

impl anchor_lang::Owner for MetadataAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for MetadataAccount {
    type Target = mpl_token_metadata::state::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

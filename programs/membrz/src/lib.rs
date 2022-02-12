//! The typescript example serves to show how one would setup an Anchor
//! workspace with TypeScript tests and migrations.

use anchor_lang::prelude::*;
use anchor_spl::{
    self,
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};

declare_id!("GMpDFEBcv4M6hhNrKq9FcVF5NGtvvedjKZKG38T7opdH");

#[program]
pub mod membrz {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>) -> ProgramResult {
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

    pub fn create_nft(ctx: Context<CreateNft>, bump_seed: u8) -> ProgramResult {
        msg!("Create NFT");
        let mint_to_ctx = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.pda.to_account_info(),
        };
        let signer_seeds = [
            "pda".as_bytes(),
            &program::Membrz::id().to_bytes(),
            &[bump_seed],
        ];
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                mint_to_ctx,
                &[&signer_seeds],
            ),
            1,
        )
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
pub struct CreateNft<'info> {
    pub authority: Signer<'info>,

    #[account(init, payer = authority, mint::decimals = 0, mint::authority = pda, mint::freeze_authority = pda)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = pda)]
    pub token_account: Account<'info, TokenAccount>,

    pub pda: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

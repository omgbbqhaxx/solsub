use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9abegPWEJVgaJ5kmNiDCfh93iMCovugQY4ToznvXDkWV");

#[error_code]
pub enum MyError {
    #[msg("your receiver address is not correct")]
    InvalidReceiverAddress,
}

impl From<MyError> for ProgramError {
    fn from(e: MyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl Subdetalis {
    pub const MAX_SIZE: usize = 2 + (1 + 32) + (4 + 10 * 32);
}

#[program]
mod hello_anchor {
    use super::*;

    pub fn subscribe(ctx: Context<Subscribe>, cprice: u64) -> Result<()> {
        if cprice != 1000000000 {
            msg!("only 1 solana accepted");
            return Err(MyError::InvalidReceiverAddress.into());
        }

        let raddr: Pubkey = Pubkey::from_str("HdwzqmZbNVanLU56sKmgwdWjgZReHf9ESuD7GfGUkErc")
            .map_err(|_| MyError::InvalidReceiverAddress)?;

        let buyer = &mut ctx.accounts.buyer;
        let current_ts = ctx.accounts.clock.unix_timestamp;
        buyer.subtime = current_ts;

        if &ctx.accounts.receiver.key() != &raddr {
            msg!("You only can send money to admin of course");
            return Err(MyError::InvalidReceiverAddress.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.receiver.key(),
            cprice,
        );

        msg!("course_price {:?}", cprice);

        msg!(" key here, {:?}", &ctx.accounts.user.key());

        msg!(" admin here, {:?}", &ctx.accounts.receiver);

        msg!(" admin here, {:?}", &ctx.accounts.receiver.key());

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        );

        Ok(())
    }

    pub fn renew(ctx: Context<Renew>, cprice: u64) -> Result<()> {
        if cprice != 1000000000 {
            msg!("only 1 solana accepted");
            return Err(MyError::InvalidReceiverAddress.into());
        }

        let raddr: Pubkey = Pubkey::from_str("HdwzqmZbNVanLU56sKmgwdWjgZReHf9ESuD7GfGUkErc")
            .map_err(|_| MyError::InvalidReceiverAddress)?;

        let buyer = &mut ctx.accounts.buyer;
        let current_ts = ctx.accounts.clock.unix_timestamp;
        buyer.subtime = current_ts;

        if &ctx.accounts.receiver.key() != &raddr {
            msg!("You only can send money to admin of course");
            return Err(MyError::InvalidReceiverAddress.into());
        }

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.receiver.key(),
            cprice,
        );

        msg!("course_price {:?}", cprice);

        msg!(" key here, {:?}", &ctx.accounts.user.key());

        msg!(" admin here, {:?}", &ctx.accounts.receiver);

        msg!(" admin here, {:?}", &ctx.accounts.receiver.key());

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(init, payer = user, space = 8 + Subdetalis::MAX_SIZE, seeds=[b"solsub", user.key().as_ref()], bump)]
    pub buyer: Account<'info, Subdetalis>,
    /// CHECK: doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct Renew<'info> {
    #[account(mut)]
    pub buyer: Account<'info, Subdetalis>,
    /// CHECK: doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}

#[account]
pub struct Subdetalis {
    pub subtime: i64,
}

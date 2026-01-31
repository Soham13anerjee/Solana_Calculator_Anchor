use anchor_lang::prelude::*;

declare_id!("EUpLvkTnwGwihbExdw3khstYmtmnfim6Fnyh8iD8HgvE");

#[program]
pub mod calci {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.calci_acc.calci_result = 0;
        ctx.accounts.calci_acc.payer = ctx.accounts.fee_payer.key();

        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.calci_acc.calci_result = a + b;
        msg!(
            "Addition Result : {:?}",
            ctx.accounts.calci_acc.calci_result
        );
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, a: i64, b: i64) -> Result<()> {
        ctx.accounts.calci_acc.calci_result = a - b;
        msg!(
            "Addition Result : {:?}",
            ctx.accounts.calci_acc.calci_result
        );
        Ok(())
    }
    pub fn div(ctx: Context<Div>, a: i64, b: i64) -> Result<()> {
        
        require!(b!=0,Errorcode::DivisionByZero);
        ctx.accounts.calci_acc.calci_result = a / b;
        msg!(
            "Addition Result : {:?}",
            ctx.accounts.calci_acc.calci_result
        );
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct CalciResult {
    calci_result: i64,
    payer: Pubkey,
}

#[error_code]
pub enum Errorcode{
     #[msg("Division by zero is not allowed")]
     DivisionByZero
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    fee_payer: Signer<'info>,

    #[account(init,space=8+CalciResult::INIT_SPACE,payer=fee_payer,seeds=[b"calci",fee_payer.key().as_ref()],bump)]
    calci_acc: Account<'info, CalciResult>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    calci_acc: Account<'info, CalciResult>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    calci_acc: Account<'info, CalciResult>,
}

#[derive(Accounts)]
pub struct Div<'info> {
    #[account(mut)]
    calci_acc: Account<'info, CalciResult>,
}

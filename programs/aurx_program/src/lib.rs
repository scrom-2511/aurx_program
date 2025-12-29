use anchor_lang::prelude::*;

declare_id!("5piAYhnjtWG6bnpkX3q18rNzKNgt3536xq47w3VS5uuV");

#[program]
pub mod aurx_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

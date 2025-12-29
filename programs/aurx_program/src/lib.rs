use anchor_lang::prelude::*;

declare_id!("5piAYhnjtWG6bnpkX3q18rNzKNgt3536xq47w3VS5uuV");

#[program]
pub mod aurx_program {
    use super::*;

    pub fn store_hash(ctx: Context<StoreHash>, user_email: String, file_id: String, hash: String) -> Result<()> {
        require!(hash.as_bytes().len() <= 64, AxumError::HashTooLong);
        ctx.accounts.storage_pda.hash = hash;
        Ok(())
    }
}

#[account]
pub struct HashStorage {
    hash: String,
}

#[derive(Accounts)]
#[instruction(user_email:String, file_id: String)]
pub struct StoreHash<'info> {
    #[account(init, payer = signer, space = 8+4+50, seeds=[signer.key().as_ref(), user_email.as_bytes(), file_id.as_bytes()], bump)]
    pub storage_pda: Account<'info, HashStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AxumError {
    #[msg("Hash is too long.")]
    HashTooLong
}
use anchor_lang::prelude::*;

declare_id!("GQhQ5sYhuHiZgM6ocAaMrTT6sTsXMBF2gtTic8vZKWsT");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a mutable reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize otal_gifs counter
        base_account.total_gifs = 0;
        Ok(())
    }

    // Accept gif_link param from the user
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the git_list vector
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff <'info> {
    // Create an account own by this program
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account <'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// AddGif data specification
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer <'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // attach a vector to the account
    pub gif_list: Vec<ItemStruct>,
}
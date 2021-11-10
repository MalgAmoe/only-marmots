use anchor_lang::prelude::*;

declare_id!("J5mre6rguL9nvR6kXjtjHhEfQZMAUSEethGMTmPZ7TUc");

#[program]
pub mod marmot {
    use super::*;
    pub fn start_marmot_central(ctx: Context<StartMarmotCentral>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            marmotness: [].to_vec(),
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote_marmotness(ctx: Context<VoteMarmotness>, id: u64, is_marmot: bool) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif = match base_account.gif_list.get_mut(id as usize) {
            Some(gif) => gif,
            None => return Err(ErrorCode::UnexistantMarmot.into()),
        };

        for vote in &gif.marmotness {
            if vote.user_address == *user.to_account_info().key {
                return Err(ErrorCode::DoubleVote.into());
            }
        }

        gif.marmotness.push(VoteStruct {
            user_address: *user.to_account_info().key,
            is_marmot,
        });
        Ok(())
    }
}

#[error]
pub enum ErrorCode {
    #[msg("Id of gif does not exist")]
    UnexistantMarmot,
    #[msg("You cannot vote twice for marmotness")]
    DoubleVote,
}

#[derive(Accounts)]
pub struct StartMarmotCentral<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteMarmotness<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct VoteStruct {
    pub user_address: Pubkey,
    pub is_marmot: bool,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub marmotness: Vec<VoteStruct>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

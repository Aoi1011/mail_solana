use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod crowd_funding {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _new_project_id: u64) -> ProgramResult {
        let old_project_ids = &mut ctx.accounts.state;
        // let copy_project_ids = old_project_ids.project_ids.clone();
        old_project_ids.project_ids.push(_new_project_id);
        // old_project_ids.account.project_ids
        Ok(())
    }

    pub fn createProject(ctx: Context<CreateProject>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority)]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub authority: Pubkey,
    pub project_ids: Vec<u64>,
}

// #[account]
// pub struct Project {
//     pub authority: Pubkey,
//     pub project_ids: Vec<u64>,
// }

// #[derive(Clone)]
// pub struct IProject<'info> {
//     representative: Signer<'info>,
// }

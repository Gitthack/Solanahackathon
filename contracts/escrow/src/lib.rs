use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

// Escrow Contract with x402 Payment Support
// Handles task payments between employers and workers

declare_id!("ESCROW1111111111111111111111111111111111111");

#[program]
pub mod escrow {
    use super::*;

    // Initialize escrow for a task
    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        task_id: String,
        amount: u64,
        deadline: i64,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        escrow_account.employer = ctx.accounts.employer.key();
        escrow_account.task_id = task_id;
        escrow_account.amount = amount;
        escrow_account.deadline = deadline;
        escrow_account.status = EscrowStatus::Pending;
        escrow_account.worker = None;
        
        // Transfer tokens from employer to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.employer_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.employer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        msg!("Escrow initialized: {} USDC locked for task {}", amount, task_id);
        Ok(())
    }

    // Worker accepts the task
    pub fn accept_task(
        ctx: Context<AcceptTask>,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        require!(
            escrow_account.status == EscrowStatus::Pending,
            ErrorCode::InvalidStatus
        );
        
        escrow_account.worker = Some(ctx.accounts.worker.key());
        escrow_account.status = EscrowStatus::Active;
        escrow_account.accepted_at = Clock::get()?.unix_timestamp;
        
        msg!("Task accepted by worker: {}", ctx.accounts.worker.key());
        Ok(())
    }

    // Complete task and release payment (employer confirms)
    pub fn complete_task(
        ctx: Context<CompleteTask>,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        
        require!(
            escrow_account.employer == ctx.accounts.employer.key(),
            ErrorCode::Unauthorized
        );
        require!(
            escrow_account.status == EscrowStatus::Active,
            ErrorCode::InvalidStatus
        );
        
        // Transfer to worker
        let worker = escrow_account.worker.unwrap();
        let amount = escrow_account.amount;
        
        // CPI transfer from escrow to worker
        let seeds = &[
            b"escrow",
            escrow_account.task_id.as_bytes(),
            &[*ctx.bumps.get("escrow_account").unwrap()],
        ];
        let signer = &[&seeds[..],
        ];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.worker_token_account.to_account_info(),
            authority: ctx.accounts.escrow_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        escrow_account.status = EscrowStatus::Completed;
        
        msg!("Payment released: {} USDC to worker {}", amount, worker);
        Ok(())
    }

    // Refund employer if task expired
    pub fn refund_expired(
        ctx: Context<RefundExpired>,
    ) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow_account;
        let current_time = Clock::get()?.unix_timestamp;
        
        require!(
            current_time > escrow_account.deadline,
            ErrorCode::NotExpired
        );
        require!(
            escrow_account.status != EscrowStatus::Completed,
            ErrorCode::AlreadyCompleted
        );
        
        // Refund employer
        let amount = escrow_account.amount;
        
        // CPI transfer from escrow back to employer
        let seeds = &[
            b"escrow",
            escrow_account.task_id.as_bytes(),
            &[*ctx.bumps.get("escrow_account").unwrap()],
        ];
        let signer = &[&seeds[..],
        ];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.employer_token_account.to_account_info(),
            authority: ctx.accounts.escrow_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;
        
        escrow_account.status = EscrowStatus::Refunded;
        
        msg!("Refund issued: {} USDC", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = employer,
        space = 8 + 300,
        seeds = [b"escrow", task_id.as_bytes()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(mut)]
    pub employer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptTask<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub worker: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub employer: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub worker_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RefundExpired<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub employer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct EscrowAccount {
    pub employer: Pubkey,
    pub worker: Option<Pubkey>,
    pub task_id: String,
    pub amount: u64,
    pub deadline: i64,
    pub accepted_at: i64,
    pub status: EscrowStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EscrowStatus {
    Pending,    // Waiting for worker
    Active,     // Worker assigned, in progress
    Completed,  // Payment released
    Refunded,   // Expired, refunded to employer
    Disputed,   // Under arbitration
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid escrow status")]
    InvalidStatus,
    #[msg("Task not expired yet")]
    NotExpired,
    #[msg("Task already completed")]
    AlreadyCompleted,
}

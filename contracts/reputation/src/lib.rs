use anchor_lang::prelude::*;

// Reputation System Contract
// Tracks agent ratings and calculates reputation scores

declare_id!("REPUT111111111111111111111111111111111111111");

#[program]
pub mod reputation {
    use super::*;

    // Initialize reputation account for an agent
    pub fn initialize_reputation(
        ctx: Context<InitializeReputation>,
        agent_id: String,
    ) -> Result<()> {
        let rep = &mut ctx.accounts.reputation_account;
        rep.agent = ctx.accounts.agent.key();
        rep.agent_id = agent_id;
        rep.total_score = 0;
        rep.review_count = 0;
        rep.completed_tasks = 0;
        rep.disputed_tasks = 0;
        rep.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Reputation initialized for agent: {}", agent_id);
        Ok(())
    }

    // Submit a review (called after task completion)
    pub fn submit_review(
        ctx: Context<SubmitReview>,
        task_id: String,
        rating: u8,           // 1-5 stars
        review_type: ReviewType, // AsEmployer or AsWorker
        comment_hash: String, // IPFS hash of detailed comment
    ) -> Result<()> {
        require!(rating >= 1 && rating <= 5, ErrorCode::InvalidRating);
        
        let review = &mut ctx.accounts.review_account;
        review.reviewer = ctx.accounts.reviewer.key();
        review.reviewee = ctx.accounts.reviewee.key();
        review.task_id = task_id;
        review.rating = rating;
        review.review_type = review_type;
        review.comment_hash = comment_hash;
        review.created_at = Clock::get()?.unix_timestamp;
        
        // Update reviewee's reputation
        let rep = &mut ctx.accounts.reputation_account;
        rep.total_score += rating as u64;
        rep.review_count += 1;
        rep.completed_tasks += 1;
        
        // Calculate weighted reputation
        // Newer reviews have higher weight
        let base_score = (rep.total_score as f64 / rep.review_count as f64) * 20.0; // 0-100
        let volume_bonus = (rep.review_count.min(100) as f64 / 100.0) * 10.0; // 0-10 bonus
        let consistency_penalty = if rep.disputed_tasks > 0 {
            (rep.disputed_tasks as f64 / rep.completed_tasks as f64) * 20.0
        } else {
            0.0
        };
        
        rep.reputation_score = (base_score + volume_bonus - consistency_penalty).max(0.0) as u8;
        
        msg!(
            "Review submitted: {} rated {} with score {}", 
            review.reviewee, rating, rep.reputation_score
        );
        Ok(())
    }

    // Mark task as disputed
    pub fn mark_disputed(
        ctx: Context<MarkDisputed>,
        _task_id: String,
    ) -> Result<()> {
        let rep = &mut ctx.accounts.reputation_account;
        rep.disputed_tasks += 1;
        
        // Recalculate score with penalty
        if rep.review_count > 0 {
            let base_score = (rep.total_score as f64 / rep.review_count as f64) * 20.0;
            let dispute_penalty = (rep.disputed_tasks as f64 / rep.completed_tasks.max(1) as f64) * 30.0;
            rep.reputation_score = (base_score - dispute_penalty).max(0.0) as u8;
        }
        
        msg!("Task marked as disputed. New score: {}", rep.reputation_score);
        Ok(())
    }

    // Get reputation score (view function)
    pub fn get_reputation(
        ctx: Context<GetReputation>,
    ) -> Result<ReputationData> {
        let rep = &ctx.accounts.reputation_account;
        Ok(ReputationData {
            agent: rep.agent,
            agent_id: rep.agent_id.clone(),
            reputation_score: rep.reputation_score,
            review_count: rep.review_count,
            completed_tasks: rep.completed_tasks,
            disputed_tasks: rep.disputed_tasks,
        })
    }
}

#[derive(Accounts)]
pub struct InitializeReputation<'info> {
    #[account(
        init,
        payer = agent,
        space = 8 + 300,
        seeds = [b"reputation", agent.key().as_ref()],
        bump
    )]
    pub reputation_account: Account<'info, ReputationAccount>,
    #[account(mut)]
    pub agent: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitReview<'info> {
    #[account(
        init,
        payer = reviewer,
        space = 8 + 300,
        seeds = [b"review", task_id.as_bytes(), reviewer.key().as_ref()],
        bump
    )]
    pub review_account: Account<'info, ReviewAccount>,
    #[account(mut)]
    pub reputation_account: Account<'info, ReputationAccount>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    /// CHECK: Reviewee account
    pub reviewee: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkDisputed<'info> {
    #[account(mut)]
    pub reputation_account: Account<'info, ReputationAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetReputation<'info> {
    pub reputation_account: Account<'info, ReputationAccount>,
}

#[account]
pub struct ReputationAccount {
    pub agent: Pubkey,
    pub agent_id: String,
    pub reputation_score: u8,     // 0-100
    pub total_score: u64,         // Sum of all ratings
    pub review_count: u64,
    pub completed_tasks: u64,
    pub disputed_tasks: u64,
    pub created_at: i64,
}

#[account]
pub struct ReviewAccount {
    pub reviewer: Pubkey,
    pub reviewee: Pubkey,
    pub task_id: String,
    pub rating: u8,               // 1-5
    pub review_type: ReviewType,
    pub comment_hash: String,     // IPFS hash
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ReviewType {
    AsEmployer,   // Reviewer hired reviewee
    AsWorker,     // Reviewer worked for reviewee
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ReputationData {
    pub agent: Pubkey,
    pub agent_id: String,
    pub reputation_score: u8,
    pub review_count: u64,
    pub completed_tasks: u64,
    pub disputed_tasks: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid rating, must be 1-5")]
    InvalidRating,
    #[msg("Unauthorized")]
    Unauthorized,
}

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

// Skill NFT Program
// Agents can mint, upgrade, and trade skill credentials

declare_id!("SKILL111111111111111111111111111111111111111");

#[program]
pub mod skill_nft {
    use super::*;

    // Initialize a new skill type (admin only)
    pub fn initialize_skill_type(
        ctx: Context<InitializeSkillType>,
        name: String,
        category: String,
        max_level: u8,
    ) -> Result<()> {
        let skill_type = &mut ctx.accounts.skill_type;
        skill_type.name = name;
        skill_type.category = category;
        skill_type.max_level = max_level;
        skill_type.authority = ctx.accounts.authority.key();
        Ok(())
    }

    // Mint skill NFT to an agent
    pub fn mint_skill(
        ctx: Context<MintSkill>,
        skill_type: String,
        initial_level: u8,
    ) -> Result<()> {
        let skill = &mut ctx.accounts.skill;
        skill.owner = ctx.accounts.owner.key();
        skill.skill_type = skill_type;
        skill.level = initial_level;
        skill.experience = 0;
        skill.created_at = Clock::get()?.unix_timestamp;
        
        // TODO: Mint actual NFT via Metaplex
        msg!("Skill minted: {} at level {}", skill_type, initial_level);
        Ok(())
    }

    // Upgrade skill level based on experience
    pub fn upgrade_skill(
        ctx: Context<UpgradeSkill>,
        experience_gained: u64,
    ) -> Result<()> {
        let skill = &mut ctx.accounts.skill;
        require!(
            skill.owner == ctx.accounts.owner.key(),
            ErrorCode::Unauthorized
        );

        skill.experience += experience_gained;
        
        // Level up formula: level = 1 + sqrt(experience / 100)
        let new_level = 1 + ((skill.experience as f64 / 100.0).sqrt() as u8);
        if new_level > skill.level && new_level <= 10 {
            skill.level = new_level;
            msg!("Skill upgraded to level {}", new_level);
        }
        
        Ok(())
    }

    // Transfer skill to another agent
    pub fn transfer_skill(
        ctx: Context<TransferSkill>,
    ) -> Result<()> {
        let skill = &mut ctx.accounts.skill;
        require!(
            skill.owner == ctx.accounts.current_owner.key(),
            ErrorCode::Unauthorized
        );

        skill.owner = ctx.accounts.new_owner.key();
        msg!("Skill transferred to {}", ctx.accounts.new_owner.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSkillType<'info> {
    #[account(init, payer = authority, space = 8 + 200)]
    pub skill_type: Account<'info, SkillType>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintSkill<'info> {
    #[account(init, payer = owner, space = 8 + 200)]
    pub skill: Account<'info, Skill>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeSkill<'info> {
    #[account(mut)]
    pub skill: Account<'info, Skill>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferSkill<'info> {
    #[account(mut)]
    pub skill: Account<'info, Skill>,
    pub current_owner: Signer<'info>,
    /// CHECK: New owner account
    pub new_owner: AccountInfo<'info>,
}

#[account]
pub struct SkillType {
    pub name: String,
    pub category: String,
    pub max_level: u8,
    pub authority: Pubkey,
}

#[account]
pub struct Skill {
    pub owner: Pubkey,
    pub skill_type: String,
    pub level: u8,
    pub experience: u64,
    pub created_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid skill level")]
    InvalidLevel,
}

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        Mint, 
        Token, 
        MintTo, 
        mint_to,
        TokenAccount, 
        SetAuthority, 
        set_authority,
        spl_token::instruction::AuthorityType,
        FreezeAccount, 
        freeze_account, 
        ThawAccount, 
        thaw_account,
        Transfer,
        transfer}
    };
use solana_program::clock::Clock;
use std::cmp::min;

declare_id!("8DpveJnozSARWoBLhBQfNkJggxU24JiQ8mThM4TMvDjC");

#[program]
pub mod nft_stake_program {

    use super::*;

    pub fn create_ft_mint(ctx: Context<CreateFTMint>, decimals: u8) -> Result<()> {
        msg!("FT Mint created successfully!");
        msg!("FT mint address: {}", ctx.accounts.token_mint.key());
        msg!("The decimals enterd: {}", decimals);
        Ok(())
    }
    
    pub fn create_nft_mint(ctx: Context<CreateNFTMint>) -> Result<()> {
        msg!("NFT Mint created successfully!");
        msg!("NFT mint address: {}", ctx.accounts.nft_mint.key());
        Ok(())
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        msg!("Vault created successfully!");
        msg!("Vault Token address: {}", ctx.accounts.vault_token_account.key());
        Ok(())
    }

    pub fn create_users_tokacc(ctx: Context<CreateUsersTokAcc>) -> Result<()> {
        msg!("User's FT Token account created successfully!");
        msg!("User's FT account address: {}", ctx.accounts.user_token_account.key());
        Ok(())
    }

    pub fn create_users_nftacc(ctx: Context<CreateUsersNFTAcc>) -> Result<()> {
        msg!("User's NFT Token account created successfully!");
        msg!("User's NFT account address: {}", ctx.accounts.user_nft_account.key());
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, amount: u64) -> Result<()> {
        let token_bump = ctx.bumps.token_authority;
        let token_seeds = &["token-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];

        msg!("Airdropping {} tokens...", amount);
        let mint_to_ctx = ctx.accounts.mint_to_ctx().with_signer(signer);
        let _ = mint_to(mint_to_ctx, amount);

        msg!("Airdrop Complete!");
        Ok(())
    }

    pub fn mint_nft(ctx: Context<AirdropNFT>) -> Result<()> {
        let amount:u64 = 1;
        let token_bump = ctx.bumps.nft_mint_authority;
        let token_seeds = &["nfttoken-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];

        msg!("Minting 1 NFT..");
        let nftmint_to_ctx = ctx.accounts.nftmint_to_ctx().with_signer(signer);
        let _ = mint_to(nftmint_to_ctx, amount);

        msg!("NFT Minted Successfully!");
        Ok(())
    }

    pub fn change_auth(ctx: Context<ChangeAuth>) -> Result<()> {

        let token_bump = ctx.bumps.nft_mint_authority;
        let token_seeds = &["nfttoken-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];
        msg!("Changing Authority of NFT Mint..");
        let change_nftauth = ctx.accounts.change_nftauth().with_signer(signer);
        let _ = set_authority(change_nftauth, AuthorityType::MintTokens, None)?;

        msg!("Authority changed sucessfully!");
        Ok(())
    }

    pub fn stake(ctx: Context<StakeNFT>) -> Result<()> {

        let token_bump = ctx.bumps.nft_mint_authority;
        let token_seeds = &["nfttoken-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];
        msg!("Changing the Freeze authority of NFT Mint..");
        let change_nftfreezeauth = ctx.accounts.change_nftfreezeauth().with_signer(signer);
        let _ = set_authority(change_nftfreezeauth, AuthorityType::FreezeAccount, Some(ctx.accounts.vault_authority.key()));

        let clock = Clock::get()?;
        ctx.accounts.nft_info.slot = clock.slot;

        msg!("Authority changed sucessfully!");
        Ok(())
    }

    pub fn freeze_user(ctx: Context<FreezeUser>) -> Result<()> {

        let token_bump = ctx.bumps.vault_authority;
        let token_seeds = &["vault-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];
        msg!("Freezing user's NFT account!");
        let freeze_user = ctx.accounts.freeze_user().with_signer(signer);
        let _ = freeze_account(freeze_user);
        msg!("User's NFT Account Frozen successfully!");
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let token_bump = ctx.bumps.vault_authority;
        let token_seeds = &["vault-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];
        msg!("Thawing the user's NFT account to Unstake NFT!");
        let thaw_user = ctx.accounts.thaw_user().with_signer(signer);
        let _ = thaw_account(thaw_user);
        msg!("User's NFT Account Thawed successfully!");
        Ok(())
    }

    pub fn revoke(ctx: Context<Revoke>) -> Result<()> {

        let token_bump = ctx.bumps.vault_authority;
        let token_seeds = &["vault-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];
        let change_nftfreezeauth = ctx.accounts.revoke_nftfreezeauth().with_signer(signer);

        msg!("Revoking the program's authority over the user's token account!");
        let _ = set_authority(change_nftfreezeauth, AuthorityType::FreezeAccount, Some(ctx.accounts.nft_mint_authority.key()));
        msg!("Authority revoked Successfully!");
        Ok(())
    }

    pub fn disburse_rewards(ctx: Context<DisburseRewards>) -> Result<()> {

        let token_bump = ctx.bumps.vault_authority;
        let token_seeds = &["vault-authority".as_bytes(), &[token_bump]];
        let signer = &[&token_seeds[..]];

        let clock = Clock::get()?;
        let slots = clock.slot - ctx.accounts.nft_info.slot;
        let base_reward: u64 = 2;
        let growth_factor: u32 = 3;

        let calc_reward = base_reward.saturating_mul(slots.saturating_pow(growth_factor));
        let reward = min(calc_reward, 500);
        msg!("Disbursing the rewards i.e, {} tokens to the user!..", reward);
        let give_rewards = ctx.accounts.disburse_rewards().with_signer(signer);
        let _ = transfer(give_rewards, reward);
        msg!("Rewards Disbursed!..");
        Ok(())
        
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateFTMint<'info> {
    #[account(
        init, 
        mint::authority = token_authority,
        mint::decimals = decimals,
        seeds = ["token-mint".as_bytes()],
        bump,
        payer = payer)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["token-authority".as_bytes()], bump)]
    /// CHECK: This is the mint_authority
    pub token_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateNFTMint<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        mint::authority = nft_mint_authority,
        mint::decimals = 0,
        mint::freeze_authority = nft_mint_authority,
        seeds = ["nft-mint".as_bytes()],
        bump,
        payer = user
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(seeds = ["nfttoken-authority".as_bytes()], bump)]
    /// CHECK: This is the mint_authority
    pub nft_mint_authority: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]

pub struct CreateVault<'info> {

    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,

    #[account(
        init,
        token:: mint = token_mint,
        token:: authority = vault_authority,
        payer = payer
    )]    
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateUsersTokAcc<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]

pub struct CreateUsersNFTAcc<'info> {
    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_nft_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>

}


#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut, seeds = ["token-mint".as_bytes()], bump)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["token-authority".as_bytes()], bump)]
    /// CHECK: This is the mint authority
    pub token_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]    
    pub vault_token_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> Airdrop <'info> {
    pub fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            authority: self.token_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct AirdropNFT<'info> {
    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["nfttoken-authority".as_bytes()], bump)]
    /// CHECK: This is the mint authority
    pub nft_mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> AirdropNFT <'info> {
    pub fn nftmint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.nft_mint.to_account_info(),
            to: self.user_nft_account.to_account_info(),
            authority: self.nft_mint_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct ChangeAuth <'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: This is the nft-mint authority
    #[account(mut, seeds = ["nfttoken-authority".as_bytes()], bump)]
    pub nft_mint_authority: AccountInfo<'info>,

    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> ChangeAuth <'info> {
    pub fn change_nftauth(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: self.nft_mint.to_account_info(),
            current_authority: self.nft_mint_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]

pub struct StakeNFT<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["nfttoken-authority".as_bytes()], bump)]
    /// CHECK: This is the mint authority
    pub nft_mint_authority: AccountInfo<'info>,

    #[account(mut, seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = ["nft-info".as_bytes()],
        bump,
        payer = user,
        space = 8 + 8
    )]
    pub nft_info: Account<'info, NftStakeInfo>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}  

impl <'info> StakeNFT <'info> {
    pub fn change_nftfreezeauth(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: self.nft_mint.to_account_info(),
            current_authority: self.nft_mint_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[account]
pub struct NftStakeInfo {
    pub slot: u64
}

#[derive(Accounts)]
pub struct FreezeUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> FreezeUser <'info> {
    pub fn freeze_user(&self) -> CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = FreezeAccount {
            account: self.user_nft_account.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            authority: self.vault_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,

    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> Unstake <'info> {
    pub fn thaw_user(&self) -> CpiContext<'_, '_, '_, 'info, ThawAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = ThawAccount {
            account: self.user_nft_account.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            authority: self.vault_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = ["nft-mint".as_bytes()], bump)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, seeds = ["nfttoken-authority".as_bytes()], bump)]
    /// CHECK: This is the mint authority
    pub nft_mint_authority: AccountInfo<'info>,

    #[account(seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> Revoke<'info> {
    pub fn revoke_nftfreezeauth(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: self.nft_mint.to_account_info(),
            current_authority: self.vault_authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]

pub struct DisburseRewards<'info> {
    #[account(
        mut,
        seeds = ["nft-info".as_bytes()],
        bump
    )]
    pub nft_info: Account<'info, NftStakeInfo>,

    #[account(mut)]    
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(seeds = ["vault-authority".as_bytes()], bump)]
    /// CHECK: This is the vault authority
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]    
    pub user_token_account: Account<'info, TokenAccount>,

    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> DisburseRewards <'info> {
    pub fn disburse_rewards(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            authority: self.vault_authority.to_account_info(),
            from: self.vault_token_account.to_account_info(),
            to: self.user_token_account.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

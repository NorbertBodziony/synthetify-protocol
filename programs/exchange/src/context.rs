use anchor_lang::prelude::*;
use crate::account::*;
use anchor_spl::token::{self, Burn, MintTo, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateSwapline<'info> {
    #[account(seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(init,seeds = [b"swaplinev1", synthetic.key.as_ref(),collateral.key.as_ref()], bump=bump, payer=admin )]
    pub swapline: Loader<'info, Swapline>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(
        constraint = &collateral_reserve.mint == collateral.key,
        constraint = collateral_reserve.owner == state.load()?.exchange_authority
    )]
    pub collateral_reserve: CpiAccount<'info, TokenAccount>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct UseSwapline<'info> {
    #[account(seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,seeds = [b"swaplinev1", synthetic.key.as_ref(),collateral.key.as_ref()], bump = swapline.load()?.bump )]
    pub swapline: Loader<'info, Swapline>,
    #[account(mut)]
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(
        mut,
        constraint = &user_collateral_account.mint == collateral.key,
        constraint = &user_collateral_account.owner == signer.key
    )]
    pub user_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = &user_synthetic_account.mint == synthetic.key,
        constraint = &user_synthetic_account.owner == signer.key
    )]
    pub user_synthetic_account: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(
        mut,
        constraint = &collateral_reserve.mint == collateral.key,
        constraint = collateral_reserve.to_account_info().key == &swapline.load()?.collateral_reserve,
        constraint = collateral_reserve.owner == state.load()?.exchange_authority
    )]
    pub collateral_reserve: CpiAccount<'info, TokenAccount>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&UseSwapline<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(accounts: &UseSwapline<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.synthetic.to_account_info(),
            to: accounts.user_synthetic_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'a, 'b, 'c, 'info> From<&UseSwapline<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &UseSwapline<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.synthetic.to_account_info(),
            to: accounts.user_synthetic_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct WithdrawSwaplineFee<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,seeds = [b"swaplinev1", synthetic.key.as_ref(),collateral.key.as_ref()],bump = swapline.load()?.bump)]
    pub swapline: Loader<'info, Swapline>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = collateral_reserve.owner == state.load()?.exchange_authority,
        constraint = collateral_reserve.to_account_info().key == &swapline.load()?.collateral_reserve,
        constraint = &collateral_reserve.mint == collateral.key
    )]
    pub collateral_reserve: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = &to.mint == collateral.key,
    )]
    pub to: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&WithdrawSwaplineFee<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(
        accounts: &WithdrawSwaplineFee<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.collateral_reserve.to_account_info(),
            to: accounts.to.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct SetHaltedSwapline<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
     #[account(mut,seeds = [b"swaplinev1", synthetic.key.as_ref(),collateral.key.as_ref()],bump = swapline.load()?.bump)]
    pub swapline: Loader<'info, Swapline>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct InitializeAssetsList<'info> {
    #[account(init)]
    pub assets_list: Loader<'info, AssetsList>,
    pub sny_reserve: AccountInfo<'info>,
    pub sny_liquidation_fund: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(Accounts)]
pub struct SetAssetsPrices<'info> {
    #[account(mut)]
    pub assets_list: Loader<'info, AssetsList>,
}
#[derive(Accounts)]
pub struct AddNewAsset<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
}
#[derive(Accounts)]
pub struct AdminWithdraw<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(mut)]
    pub to: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&AdminWithdraw<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &AdminWithdraw<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.usd_token.to_account_info(),
            to: accounts.to.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct WithdrawAccumulatedDebtInterest<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(mut,
        constraint = &to.mint == usd_token.key
    )]
    pub to: CpiAccount<'info, TokenAccount>,
    #[account("token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&WithdrawAccumulatedDebtInterest<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(
        accounts: &WithdrawAccumulatedDebtInterest<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.usd_token.to_account_info(),
            to: accounts.to.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct SetMaxSupply<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
}
#[derive(Accounts)]
pub struct SetPriceFeed<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub price_feed: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct AddCollateral<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub asset_address: AccountInfo<'info>,
    #[account(constraint = liquidation_fund.owner == state.load()?.exchange_authority)]
    pub liquidation_fund: CpiAccount<'info,TokenAccount>,
    #[account(constraint = liquidation_fund.owner == state.load()?.exchange_authority)]
    pub reserve_account: CpiAccount<'info,TokenAccount>,
    pub feed_address: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetCollateralRatio<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub collateral_address: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetMaxCollateral<'info> {
    #[account(mut, seeds = [b"statev1".as_ref(), &[state.load()?.bump]])]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub collateral_address: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    pub new_admin: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct SetSettlementSlot<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub synthetic_address: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct AddSynthetic<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub asset_address: CpiAccount<'info, anchor_spl::token::Mint>,
    pub feed_address: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateExchangeAccount<'info> {
    #[account(init,seeds = [b"accountv1", admin.key.as_ref()], bump=bump, payer=payer )]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    pub admin: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut, has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&Withdraw<'info>> for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    fn from(accounts: &Withdraw<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.reserve_account.to_account_info(),
            to: accounts.user_collateral_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(mut,
        constraint = &to.mint == usd_token.key
    )]
    pub to: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut, has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&Mint<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(accounts: &Mint<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.usd_token.to_account_info(),
            to: accounts.to.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(mut,
        constraint = &reserve_address.owner == exchange_authority.key
    )]
    pub reserve_address: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &user_collateral_account.owner == owner.key,
    )]
    pub user_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    // owner can deposit to any exchange_account
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&Deposit<'info>> for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    fn from(accounts: &Deposit<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.user_collateral_account.to_account_info(),
            to: accounts.reserve_address.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account("exchange_authority.key == &state.load()?.exchange_authority")]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(mut,
        constraint = &liquidator_usd_account.mint == usd_token.key,
        constraint = &liquidator_usd_account.owner == signer.key
    )]
    pub liquidator_usd_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub liquidator_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub signer: AccountInfo<'info>,
    #[account(mut,
        constraint = liquidation_fund.mint == liquidator_collateral_account.mint
    )]
    pub liquidation_fund: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_account: CpiAccount<'info, TokenAccount>,
}
#[derive(Accounts)]
pub struct BurnToken<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(constraint= exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(mut,
        constraint = &user_token_account_burn.mint == usd_token.key,
        constraint = &user_token_account_burn.owner == owner.key
    )]
    pub user_token_account_burn: CpiAccount<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&BurnToken<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &BurnToken<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.usd_token.to_account_info(),
            to: accounts.user_token_account_burn.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account("exchange_authority.key == &state.load()?.exchange_authority")]
    pub exchange_authority: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub token_in: AccountInfo<'info>,
    #[account(mut)]
    pub token_for: AccountInfo<'info>,
    #[account(mut,
        constraint = &user_token_account_in.mint == token_in.key
    )]
    pub user_token_account_in: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &user_token_account_for.mint == token_for.key
    )]
    pub user_token_account_for: CpiAccount<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&Swap<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &Swap<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.token_in.to_account_info(),
            to: accounts.user_token_account_in.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'a, 'b, 'c, 'info> From<&Swap<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(accounts: &Swap<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.token_for.to_account_info(),
            to: accounts.user_token_account_for.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct CheckCollateralization<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
}
#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
}
#[derive(Accounts)]
pub struct WithdrawRewards<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut, has_one = owner)]
    pub exchange_account: Loader<'info, ExchangeAccount>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = user_token_account.mint == staking_fund_account.mint,
        constraint = &staking_fund_account.owner == exchange_authority.key,
        constraint = staking_fund_account.to_account_info().key == &state.load()?.staking.fund_account
    )]
    pub staking_fund_account: CpiAccount<'info, TokenAccount>,
}
#[derive(Accounts)]
pub struct WithdrawLiquidationPenalty<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub to: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = liquidation_fund.mint == to.mint,
        constraint = &liquidation_fund.owner == exchange_authority.key
    )]
    pub liquidation_fund: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
}
#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut, seeds = [b"statev1".as_ref()],bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Init<'info> {
    #[account(init, seeds = [b"statev1".as_ref()], bump = bump, payer = payer)]
    pub state: Loader<'info, State>,
    pub payer: AccountInfo<'info>,
    pub admin: AccountInfo<'info>,
    pub assets_list: AccountInfo<'info>,
    pub exchange_authority: AccountInfo<'info>,
    pub staking_fund_account: CpiAccount<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct SettleSynthetic<'info> {
    #[account(init, seeds = [b"settlement".as_ref(), token_to_settle.key.as_ref()], bump=bump, payer = payer)]
    pub settlement: Loader<'info, Settlement>,
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    pub payer: AccountInfo<'info>,
    pub token_to_settle: AccountInfo<'info>,
    #[account(
        mut,
        constraint = &settlement_reserve.owner == exchange_authority.key,
        constraint = &settlement_reserve.mint == usd_token.key
    )]
    pub settlement_reserve: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = usd_token.key == &assets_list.load()?.synthetics[0].asset_address
    )]
    pub usd_token: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&SettleSynthetic<'info>>
    for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
{
    fn from(accounts: &SettleSynthetic<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.usd_token.to_account_info(),
            to: accounts.settlement_reserve.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct SwapSettledSynthetic<'info> {
    #[account(seeds = [b"settlement".as_ref(), token_to_settle.key.as_ref()],bump = settlement.load()?.bump)]
    pub settlement: Loader<'info, Settlement>,
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,
        constraint = token_to_settle.key == &settlement.load()?.token_in_address
    )]
    pub token_to_settle: AccountInfo<'info>,
    #[account(mut)]
    pub user_settled_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = usd_token.key == &user_usd_account.mint
    )]
    pub user_usd_account: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = settlement_reserve.to_account_info().key == &settlement.load()?.reserve_address
    )]
    pub settlement_reserve: CpiAccount<'info, TokenAccount>,
    #[account(
        constraint = usd_token.key == &settlement.load()?.token_out_address
    )]
    pub usd_token: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(signer,
        constraint = &user_settled_token_account.owner == signer.key
    )]
    pub signer: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&SwapSettledSynthetic<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(
        accounts: &SwapSettledSynthetic<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.settlement_reserve.to_account_info(),
            to: accounts.user_usd_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
impl<'a, 'b, 'c, 'info> From<&SwapSettledSynthetic<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Burn<'info>>
{
    fn from(accounts: &SwapSettledSynthetic<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.token_to_settle.to_account_info(),
            to: accounts.user_settled_token_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(init, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()], bump=bump, payer=admin )]
    pub vault: Loader<'info, Vault>,
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    #[account(constraint = assets_list.to_account_info().key == &state.load()?.assets_list)]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(
        constraint = &collateral_reserve.mint == collateral.key,
        constraint = collateral_reserve.owner == state.load()?.exchange_authority
    )]
    pub collateral_reserve: CpiAccount<'info, TokenAccount>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateVaultEntry<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(init, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()], bump=bump, payer=owner)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump )]
    pub vault: Loader<'info, Vault>,
    #[account(constraint = assets_list.to_account_info().key == &state.load()?.assets_list)]
    pub assets_list: Loader<'info, AssetsList>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct DepositVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()],bump=vault_entry.load()?.bump)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump)]
    pub vault: Loader<'info, Vault>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut,
        constraint = &vault.load()?.collateral_reserve == reserve_address.to_account_info().key,
        constraint = &reserve_address.mint == collateral.key,
        constraint = &reserve_address.owner == exchange_authority.key
    )]
    pub reserve_address: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &user_collateral_account.mint == collateral.key,
        constraint = &user_collateral_account.owner == owner.key
    )]
    pub user_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&DepositVault<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &DepositVault<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.user_collateral_account.to_account_info(),
            to: accounts.reserve_address.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct BorrowVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()],bump=vault_entry.load()?.bump)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump)]
    pub vault: Loader<'info, Vault>,
    #[account(mut)]
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut,
        constraint = &to.mint == synthetic.key,
    )]
    pub to: CpiAccount<'info, TokenAccount>, // not must be owner
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&BorrowVault<'info>> for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
    fn from(accounts: &BorrowVault<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: accounts.synthetic.to_account_info(),
            to: accounts.to.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()],bump=vault_entry.load()?.bump)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump)]
    pub vault: Loader<'info, Vault>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut, 
        constraint = &vault.load()?.collateral_reserve == reserve_address.to_account_info().key,
        constraint = &reserve_address.owner == exchange_authority.key,
        constraint = &reserve_address.mint == collateral.key,
    )]
    pub reserve_address: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &user_collateral_account.mint == collateral.key,
        constraint = &user_collateral_account.owner == owner.key
    )]
    pub user_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,
    pub exchange_authority: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&WithdrawVault<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &WithdrawVault<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.reserve_address.to_account_info(),
            to: accounts.user_collateral_account.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct RepayVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()],bump=vault_entry.load()?.bump)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump)]
    pub vault: Loader<'info, Vault>,
    #[account(mut)]
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut, 
        constraint = &user_token_account_repay.owner == owner.key,
        constraint = &user_token_account_repay.mint == synthetic.key,
    )]
    pub user_token_account_repay: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}
impl<'a, 'b, 'c, 'info> From<&RepayVault<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
    fn from(accounts: &RepayVault<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
        let cpi_accounts = Burn {
            mint: accounts.synthetic.to_account_info(),
            to: accounts.user_token_account_repay.to_account_info(),
            authority: accounts.exchange_authority.to_account_info(),
        };
        let cpi_program = accounts.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct LiquidateVault<'info> {
    #[account(seeds = [b"statev1".as_ref()], bump = state.load()?.bump)]
    pub state: Loader<'info, State>,
    #[account(mut,has_one = owner, seeds = [b"vault_entryv1", owner.key.as_ref(), vault.to_account_info().key.as_ref()],bump=vault_entry.load()?.bump)]
    pub vault_entry: Loader<'info, VaultEntry>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref()],bump=vault.load()?.bump )]
    pub vault: Loader<'info, Vault>,
    #[account(mut)]
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(mut,
        constraint = assets_list.to_account_info().key == &state.load()?.assets_list
    )]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(mut,
        constraint = &vault.load()?.collateral_reserve == collateral_reserve.to_account_info().key,
        constraint = &collateral_reserve.mint == collateral.key
    )]
    pub collateral_reserve: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &liquidator_synthetic_account.mint == synthetic.key,
        constraint = &liquidator_synthetic_account.owner == liquidator.key
    )]
    pub liquidator_synthetic_account: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &liquidator_collateral_account.owner == liquidator.key,
        constraint = &liquidator_collateral_account.mint == collateral.key
    )]
    pub liquidator_collateral_account: CpiAccount<'info, TokenAccount>,
    #[account(mut,
        constraint = &liquidation_fund.owner == &state.load()?.exchange_authority,
        constraint = &liquidation_fund.mint == collateral.key
    )]
    pub liquidation_fund: CpiAccount<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    #[account(signer)]
    pub liquidator: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetVaultHalted<'info> {
    #[account(seeds = [b"statev1".as_ref(), &[state.load()?.bump]])]
    pub state: Loader<'info, State>,
    #[account(signer)]
    pub admin: AccountInfo<'info>,
    #[account(mut, seeds = [b"vaultv1", synthetic.key.as_ref(), collateral.key.as_ref(), &[vault.load()?.bump]])]
    pub vault: Loader<'info, Vault>,
    pub synthetic: AccountInfo<'info>,
    pub collateral: AccountInfo<'info>,
    #[account(constraint = assets_list.to_account_info().key == &state.load()?.assets_list)]
    pub assets_list: Loader<'info, AssetsList>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(constraint = exchange_authority.key == &state.load()?.exchange_authority)]
    pub exchange_authority: AccountInfo<'info>,
}
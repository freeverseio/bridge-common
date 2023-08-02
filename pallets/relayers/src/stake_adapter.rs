// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Code that allows `NamedReservableCurrency` to be used as a `StakeAndSlash`
//! mechanism of the relayers pallet.

use bp_relayers::{PayRewardFromAccount, RewardsAccountParams, StakeAndSlash};
use frame_support::traits::{tokens::BalanceStatus, NamedReservableCurrency};
use parity_scale_codec::Codec;
use sp_runtime::{traits::Get, DispatchError, DispatchResult};
use sp_std::{fmt::Debug, marker::PhantomData};

/// `StakeAndSlash` that works with `NamedReservableCurrency` and uses named
/// reservations.
///
/// **WARNING**: this implementation assumes that the relayers pallet is configured to
/// use the [`bp_relayers::PayRewardFromAccount`] as its relayers payment scheme.
pub struct StakeAndSlashNamed<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>(
    PhantomData<(AccountId, BlockNumber, Currency, ReserveId, Stake, Lease)>,
);

impl<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>
    StakeAndSlash<AccountId, BlockNumber, Currency::Balance>
    for StakeAndSlashNamed<AccountId, BlockNumber, Currency, ReserveId, Stake, Lease>
where
    AccountId: Codec + Debug,
    Currency: NamedReservableCurrency<AccountId>,
    ReserveId: Get<Currency::ReserveIdentifier>,
    Stake: Get<Currency::Balance>,
    Lease: Get<BlockNumber>,
{
    type RequiredStake = Stake;
    type RequiredRegistrationLease = Lease;

    fn reserve(relayer: &AccountId, amount: Currency::Balance) -> DispatchResult {
        Currency::reserve_named(&ReserveId::get(), relayer, amount)
    }

    fn unreserve(relayer: &AccountId, amount: Currency::Balance) -> Currency::Balance {
        Currency::unreserve_named(&ReserveId::get(), relayer, amount)
    }

    fn repatriate_reserved(
        relayer: &AccountId,
        beneficiary: RewardsAccountParams,
        amount: Currency::Balance,
    ) -> Result<Currency::Balance, DispatchError> {
        let beneficiary_account =
            PayRewardFromAccount::<(), AccountId>::rewards_account(beneficiary);
        Currency::repatriate_reserved_named(
            &ReserveId::get(),
            relayer,
            &beneficiary_account,
            amount,
            BalanceStatus::Free,
        )
    }
}

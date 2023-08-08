#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use frame_support::traits::{Currency, ExistenceRequirement};
use sp_runtime::traits::{StaticLookup, Zero};
pub use weights::*;

// helpful for looking up other accounts:
pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
// helpful for representing an amount of Currency to be minted, transferred, etc.
pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;

        // Currency Trait will be implemented by the Balances Pallet
        // the Balances Pallet is wired up in the mock.rs.
        // To add this pallet to the Node runtime, we'd have to wire up the Balances Pallet
        // in the runtime/lib.rs.
        type Balances: Currency<Self::AccountId>;
    }

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Approved {
            owner: T::AccountId,
            spender: T::AccountId,
            amount: BalanceOf<T>,
        },
        Transferred {
            sender: T::AccountId,
            receiver: T::AccountId,
            amount: BalanceOf<T>,
        },
        TransferredFrom {
            sender: T::AccountId,
            operator: T::AccountId,
            receiver: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        NotApproved,
    }

    #[pallet::storage]
    #[pallet::getter(fn allowances)]
    pub type Allowances<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // owner
        Blake2_128Concat,
        T::AccountId, // sender
        BalanceOf<T>,
        ValueQuery,
    >;

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            _sender: OriginFor<T>,
            _receiver: AccountIdLookupOf<T>,
            _amount: BalanceOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(_sender)?;
            let receiver = T::Lookup::lookup(_receiver)?;

            Self::_transfer(&sender, &receiver, _amount)?;

            Self::deposit_event(Event::<T>::Transferred {
                sender,
                receiver,
                amount: _amount,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::approve())]
        pub fn approve(
            _owner: OriginFor<T>,
            _spender: AccountIdLookupOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let owner = ensure_signed(_owner)?;
            let spender = T::Lookup::lookup(_spender)?;

            Allowances::<T>::insert(owner.clone(), spender.clone(), amount);

            Self::deposit_event(Event::<T>::Approved {
                owner,
                spender,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::transfer_from())]
        pub fn transfer_from(
            _spender: OriginFor<T>,
            _owner: AccountIdLookupOf<T>,
            _recipient: AccountIdLookupOf<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let spender = ensure_signed(_spender)?;

            let owner = T::Lookup::lookup(_owner)?;
            let recipient = T::Lookup::lookup(_recipient)?;

            Allowances::<T>::try_mutate(
                owner.clone(),
                spender.clone(),
                |allowance| -> DispatchResult {
                    ensure!(*allowance > Zero::zero(), Error::<T>::NotApproved);
                    ensure!(*allowance >= amount, Error::<T>::InsufficientBalance);

                    Self::_transfer(&owner, &recipient, amount)?;

                    *allowance -= amount;

                    Ok(())
                },
            )?;

            Self::deposit_event(Event::<T>::TransferredFrom {
                sender: owner,
                operator: spender,
                receiver: recipient,
                amount,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn _transfer(
            from: &T::AccountId,
            to: &T::AccountId,
            amount: BalanceOf<T>,
        ) -> Result<(), DispatchError> {
            let balance_sender = T::Balances::free_balance(from);
            ensure!(balance_sender >= amount, Error::<T>::InsufficientBalance);

            T::Balances::transfer(&from, &to, amount, ExistenceRequirement::KeepAlive)
                .map_err(|_| Error::<T>::InsufficientBalance)?;

            Ok(())
        }
    }
}

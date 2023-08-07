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
use sp_runtime::traits::StaticLookup;
pub use weights::*;

// helpful for looking up other accounts:
pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;
// helpful for representing an amount of Currency to be minted, transferred, etc.
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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
        type Currency: Currency<Self::AccountId>;
    }

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Transferred {
            sender: T::AccountId,
            receiver: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
    }

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

            let balance_sender = T::Currency::free_balance(&sender);

            ensure!(balance_sender >= _amount, Error::<T>::InsufficientBalance);

            T::Currency::transfer(&sender, &receiver, _amount, ExistenceRequirement::KeepAlive)?;

            Self::deposit_event(Event::<T>::Transferred {
                sender,
                receiver,
                amount: _amount,
            });

            Ok(())
        }
    }
}

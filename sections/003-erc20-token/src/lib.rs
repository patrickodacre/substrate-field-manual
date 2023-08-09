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
pub use types::*;
pub mod types;
use sp_runtime::traits::{CheckedAdd, CheckedSub, One, StaticLookup, Zero};
pub use weights::*;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

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
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;

        type MaxTokenNameLen: Get<u32>;
        type MaxTokenSymbolLen: Get<u32>;
        type MaxLength: Get<u32>;
        type TokenId: Copy
            + Default
            + Member
            + Parameter
            + Encode
            + Decode
            + CheckedAdd
            + MaxEncodedLen
            + One
            + Zero;
        type TokenBalance: Copy
            + Default
            + PartialOrd
            + Member
            + Parameter
            + Encode
            + Decode
            + CheckedAdd
            + CheckedSub
            + MaxEncodedLen
            + Zero;
    }

    #[pallet::storage]
    #[pallet::getter(fn last_token_id)]
    pub type LastTokenId<T: Config> = StorageValue<_, T::TokenId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tokens)]
    pub type Tokens<T: Config> =
        StorageMap<_, Twox64Concat, T::TokenId, TokenDetails<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn balance_of)]
    pub type BalanceOf<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::TokenId,
        Twox64Concat,
        T::AccountId,
        T::TokenBalance,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn allowance)]
    pub type Allowance<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Twox64Concat, T::TokenId>,
            NMapKey<Twox64Concat, T::AccountId>, // owner
            NMapKey<Twox64Concat, T::AccountId>, // spender
        ),
        T::TokenBalance,
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        TokenMinted {
            token_id: T::TokenId,
            who: T::AccountId,
        },
        Transferred {
            from: T::AccountId,
            to: T::AccountId,
            amount: T::TokenBalance,
        },
        Approved {
            token_id: T::TokenId,
            owner: T::AccountId,
            spender: T::AccountId,
            amount: T::TokenBalance,
        },
        TransferredFrom {
            spender: T::AccountId,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::TokenBalance,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        NoneToken,
        TokenExists,
        TokenIdOverflow,
        InsufficientBalance,
        TokenBalanceOverflow,
        NotApproved,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn mint(
            _minter: OriginFor<T>,
            name: BoundedVec<u8, T::MaxTokenNameLen>,
            symbol: BoundedVec<u8, T::MaxTokenSymbolLen>,
            supply: T::TokenBalance,
        ) -> DispatchResult {
            ensure!(supply > Zero::zero(), Error::<T>::NoneValue);

            let minter = ensure_signed(_minter)?;

            let last_id = LastTokenId::<T>::get();
            let token_id = last_id
                .checked_add(&One::one())
                .ok_or(Error::<T>::TokenIdOverflow)?;

            let details = TokenDetails {
                name,
                symbol,
                supply,
            };

            Tokens::<T>::insert(token_id, details);
            BalanceOf::<T>::insert(token_id, minter.clone(), supply);

            Self::deposit_event(Event::TokenMinted {
                token_id,
                who: minter,
            });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn transfer(
            _from: OriginFor<T>,
            _to: AccountIdLookupOf<T>,
            token_id: T::TokenId,
            amount: T::TokenBalance,
        ) -> DispatchResult {
            let from = ensure_signed(_from)?;
            let to = T::Lookup::lookup(_to)?;

            Self::_transfer(token_id, &from, &to, amount)?;

            Self::deposit_event(Event::Transferred {
                from,
                to,
                amount: amount,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn approve(
            _owner: OriginFor<T>,
            _spender: AccountIdLookupOf<T>,
            token_id: T::TokenId,
            amount: T::TokenBalance,
        ) -> DispatchResult {
            let owner = ensure_signed(_owner)?;
            let spender = T::Lookup::lookup(_spender)?;

            let _token = Tokens::<T>::get(token_id).ok_or(Error::<T>::NoneToken)?;

            Allowance::<T>::insert((token_id, owner.clone(), spender.clone()), amount);

            Self::deposit_event(Event::Approved {
                token_id,
                owner,
                spender,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn transfer_from(
            _spender: OriginFor<T>,
            token_id: T::TokenId,
            _owner: AccountIdLookupOf<T>,
            _recipient: AccountIdLookupOf<T>,
            amount: T::TokenBalance,
        ) -> DispatchResult {
            let spender = ensure_signed(_spender)?;

            let owner = T::Lookup::lookup(_owner)?;
            let recipient = T::Lookup::lookup(_recipient)?;

            Allowance::<T>::try_mutate(
                (token_id, owner.clone(), spender.clone()),
                |allowance| -> DispatchResult {
                    ensure!(*allowance > Zero::zero(), Error::<T>::NotApproved);
                    ensure!(*allowance >= amount, Error::<T>::InsufficientBalance);

                    Self::_transfer(token_id, &owner, &recipient, amount)?;

                    *allowance = allowance
                        .checked_sub(&amount)
                        .ok_or(Error::<T>::InsufficientBalance)?;

                    Ok(())
                },
            )?;

            Self::deposit_event(Event::TransferredFrom {
                spender,
                from: owner,
                to: recipient,
                amount,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn _transfer(
            token_id: T::TokenId,
            from: &T::AccountId,
            to: &T::AccountId,
            amount: T::TokenBalance,
        ) -> Result<(), DispatchError> {
            BalanceOf::<T>::try_mutate(token_id, &from, |balance| -> DispatchResult {
                *balance = balance
                    .checked_sub(&amount)
                    .ok_or(Error::<T>::InsufficientBalance)?;

                Ok(())
            })?;

            BalanceOf::<T>::try_mutate(token_id, &to, |balance| -> DispatchResult {
                *balance = balance
                    .checked_add(&amount)
                    .ok_or(Error::<T>::TokenBalanceOverflow)?;

                Ok(())
            })?;

            Ok(())
        }
    }
}

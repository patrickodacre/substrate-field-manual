# Currency and Balances

The Balances Pallet is what implements the Currency Trait. Balances is meant to manage currency native to your blockchain.

"The balances pallet is specially optimized to minimize its computational and storage footprint. This means that when using the balances pallet, users will be paying the lowest possible fees and your chain will be using the least amount of weight per transaction.

Additionally, the Balances Pallet implements a number of core traits needed for a native token like ReservableCurrency, LockableCurrency, and supports things like vesting via the Vesting Pallet."
- <a href="https://substrate.stackexchange.com/a/50/4494" target="_blank">Shawn Tabrizi</a>

These are rough notes and questions to be clarified and answered at a later date:

- what is the significance of the following:

```rust
// mock.rs
pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

```

## TIL

- to use `Balances::make_free_balance_be()` function, you must import the Currency trait from `frame_support`.

```rust
// tests.rs

use frame_support::traits::Currency;

```

- a useful helper to create accounts:

```rust
// tests.rs

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;

fn account(id: u8) -> AccountIdOf<Test> {
    [id; 32].into()
}

```

You could use the `AccountIdOf<Test>` type here as a shorthand for the `T::AccountId` used in the lib.rs.

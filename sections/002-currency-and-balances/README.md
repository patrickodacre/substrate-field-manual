# Currency and Balances

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

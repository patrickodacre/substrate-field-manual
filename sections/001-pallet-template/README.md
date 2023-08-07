# Pallet Template

Swiped from the Substate Node Template, this pallet gives you a minimal working example how a pallet is put together.

## lib.rs - The Pallet

The pallet module itself and the child sections of that pallet are created using Substrate macros.

- Module -> #[frame_support::pallet]
-- #[pallet::config]

-- #[pallet::error]

-- #[pallet::event]
--- #[pallet::generate_deposit(pub(super) fn deposit_event)]

-- #[pallet::storage]
--- #[pallet::getter]

-- #[pallet::call]
--- #[pallet::call_index]
--- #[pallet::weight]


## mock.rs + tests.rs

mock.rs sets up the runtime environment for the pallet. The pallet has configurable types found in the Config (#[pallet::config]) trait. The mock.rs file is where you define the concrete values for those configurable types.

tests.rs is where you define your tests. You can separate tests in `mod` blocks for organization. A good/fast way to practice and test your pallet idea is to write tests -- it's much faster than doing a full build to manually test your extrinsics.
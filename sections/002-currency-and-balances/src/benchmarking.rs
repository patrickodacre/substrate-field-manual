//! Benchmarking setup for pallet-currencydemo
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

// TODO!
#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn transfer() {
        let value = 100u32.into();
        let caller: T::AccountId = whitelisted_caller();
        // #[extrinsic_call]
        // do_something(RawOrigin::Signed(caller), value);

        // assert_eq!(Something::<T>::get(), Some(value));
    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}

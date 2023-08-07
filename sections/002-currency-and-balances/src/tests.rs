use std::time::SystemTime;

use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
// import traits::Currency so our ::make_free_balance_be() function will be available
use frame_support::traits::Currency;

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;

fn account(id: u8) -> AccountIdOf<Test> {
    [id; 32].into()
}

#[test]
fn can_transfer_currency() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Setup
        let alice = account(1);
        let bob = account(2);
        let starting_balance = 100;
        let transfer_amount = 50;

        System::set_block_number(1);
        Balances::make_free_balance_be(&alice, starting_balance);

        // before
        let balance_alice = Balances::free_balance(&alice);
        let balance_bob = Balances::free_balance(&bob);
        assert_eq!(balance_alice, starting_balance);
        assert_eq!(balance_bob, 0);

        // transfer
        assert_ok!(TemplateModule::transfer(
            RuntimeOrigin::signed(alice.clone()),
            bob.clone(),
            transfer_amount
        ));

        // after
        let balance_alice = Balances::free_balance(&alice);
        let balance_bob = Balances::free_balance(&bob);
        assert_eq!(balance_alice, starting_balance - transfer_amount);
        assert_eq!(balance_bob, transfer_amount);

        System::assert_last_event(
            Event::Transferred {
                sender: alice,
                receiver: bob,
                amount: transfer_amount,
            }
            .into(),
        );
    });
}

use std::time::SystemTime;

use crate::{mock::*, Error, Event};
use frame_support::{assert_err, assert_noop, assert_ok};
// import traits::Currency so our ::make_free_balance_be() function will be available
use frame_support::traits::Currency;

type AccountIdOf<Test> = <Test as frame_system::Config>::AccountId;

fn account(id: u8) -> AccountIdOf<Test> {
    [id; 32].into()
}

pub mod approvals {
    use super::*;

    #[test]
    fn can_approve_spender() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let alice = account(1);
            let bob = account(2);
            let amount_approved = 100;

            let approved_amount = TemplateModule::allowances(alice.clone(), bob.clone());
            assert_eq!(approved_amount, 0);

            assert_ok!(TemplateModule::approve(
                RuntimeOrigin::signed(alice.clone()),
                bob.clone(),
                amount_approved
            ));

            System::assert_last_event(
                Event::Approved {
                    owner: alice.clone(),
                    spender: bob.clone(),
                    amount: amount_approved,
                }
                .into(),
            );

            let approved_amount = TemplateModule::allowances(alice.clone(), bob.clone());
            assert_eq!(approved_amount, amount_approved);
        });
    }

    #[test]
    fn can_transfer_from() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let alice = account(1);
            let bob = account(2);
            let amount_total = 100;
            let amount_transfer = 80;
            Balances::make_free_balance_be(&alice, amount_total);
            assert_eq!(Balances::free_balance(&alice), amount_total);

            assert_ok!(TemplateModule::approve(
                RuntimeOrigin::signed(alice.clone()),
                bob.clone(),
                amount_transfer
            ));

            assert_ok!(TemplateModule::transfer_from(
                RuntimeOrigin::signed(bob.clone()),
                alice.clone(),
                bob.clone(),
                amount_transfer,
            ));

            System::assert_last_event(
                Event::TransferredFrom {
                    sender: alice.clone(),
                    operator: bob.clone(),
                    receiver: bob.clone(),
                    amount: amount_transfer,
                }
                .into(),
            );

            assert_eq!(
                Balances::free_balance(&alice),
                amount_total - amount_transfer
            );

            assert_eq!(Balances::free_balance(&bob), amount_transfer);
        });
    }

    #[test]
    fn cannot_transfer_from_when_not_approved() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let alice = account(1);
            let bob = account(2);
            let amount_total = 100;
            let amount_transfer = 80;
            Balances::make_free_balance_be(&alice, amount_total);
            assert_eq!(Balances::free_balance(&alice), amount_total);

            // NOT Approved
            // assert_ok!(TemplateModule::approve(
            // RuntimeOrigin::signed(alice.clone()),
            // bob.clone(),
            // amount_transfer
            // ));

            assert_err!(
                TemplateModule::transfer_from(
                    RuntimeOrigin::signed(bob.clone()),
                    alice.clone(),
                    bob.clone(),
                    amount_transfer,
                ),
                Error::<Test>::NotApproved
            );

            // Still full balance
            assert_eq!(Balances::free_balance(&alice), amount_total);
            assert_eq!(Balances::free_balance(&bob), 0);
        });
    }

    #[test]
    fn approved_amount_is_reduced_when_transferred() {
        new_test_ext().execute_with(|| {
            System::set_block_number(1);

            let alice = account(1);
            let bob = account(2);
            let amount_total = 100;
            let amount_transfer = 80;
            Balances::make_free_balance_be(&alice, amount_total);
            assert_eq!(Balances::free_balance(&alice), amount_total);

            assert_ok!(TemplateModule::approve(
                RuntimeOrigin::signed(alice.clone()),
                bob.clone(),
                amount_transfer
            ));

            // before
            let approved_amount = TemplateModule::allowances(alice.clone(), bob.clone());
            assert_eq!(approved_amount, amount_transfer);

            assert_ok!(TemplateModule::transfer_from(
                RuntimeOrigin::signed(bob.clone()),
                alice.clone(),
                bob.clone(),
                amount_transfer,
            ));

            // after
            let approved_amount = TemplateModule::allowances(alice.clone(), bob.clone());
            assert_eq!(approved_amount, 0);
        });
    }
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

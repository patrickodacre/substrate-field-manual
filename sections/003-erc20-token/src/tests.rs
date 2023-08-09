use crate::types::*;
use crate::{mock::*, Error, Event};
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use frame_support::{assert_err, assert_noop, assert_ok, BoundedVec};

type MaxNameLen<T> = <T as pallet_erc20::Config>::MaxTokenNameLen;
type MaxSymbolLen<T> = <T as pallet_erc20::Config>::MaxTokenSymbolLen;
type TokenBalance<T> = <T as pallet_erc20::Config>::TokenBalance;
const ALICE: u64 = 1;
const BOB: u64 = 2;
const DAVE: u64 = 3;

fn create_token(supply: TokenBalance<Test>) -> TokenDetails<Test> {
    let name =
        BoundedVec::<u8, MaxNameLen<Test>>::try_from("MY_TOKEN".as_bytes().to_vec()).unwrap();
    let symbol =
        BoundedVec::<u8, MaxSymbolLen<Test>>::try_from("MTKN".as_bytes().to_vec()).unwrap();

    TokenDetails::new(name, symbol, supply)
}

#[test]
fn can_mint_token() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let token = create_token(1000);
        let token_id = 1;

        assert_ok!(TemplateModule::mint(
            RuntimeOrigin::signed(ALICE),
            token.name.clone(),
            token.symbol.clone(),
            token.supply
        ));

        System::assert_last_event(
            Event::TokenMinted {
                token_id,
                who: ALICE,
            }
            .into(),
        );

        let details = TemplateModule::tokens(1).unwrap();

        assert_eq!(details.supply, token.supply);
        assert_eq!(details.name, token.name);
        assert_eq!(details.symbol, token.symbol);

        let balance = TemplateModule::balance_of(token_id, ALICE);

        assert_eq!(balance, token.supply);
    });
}

#[test]
fn test_can_transfer_tokens() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let supply: TokenBalance<Test> = 1000;
        let xfer_amount: TokenBalance<Test> = 500;
        let token = create_token(supply);
        let token_id = 1;

        assert_ok!(TemplateModule::mint(
            RuntimeOrigin::signed(ALICE),
            token.name.clone(),
            token.symbol.clone(),
            token.supply
        ));

        assert_ok!(TemplateModule::transfer(
            RuntimeOrigin::signed(ALICE),
            BOB,
            token_id,
            xfer_amount
        ));

        let balance_alice = TemplateModule::balance_of(token_id, ALICE);
        let balance_bob = TemplateModule::balance_of(token_id, BOB);

        assert_eq!(balance_alice, supply - xfer_amount);
        assert_eq!(balance_bob, xfer_amount);

        System::assert_last_event(
            Event::Transferred {
                from: ALICE,
                to: BOB,
                amount: xfer_amount,
            }
            .into(),
        );
    });
}

#[test]
fn cannot_approve_nonexistent_tokens_for_transfer() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let supply: TokenBalance<Test> = 1000;
        let approved_amount: TokenBalance<Test> = 100;
        let _token = create_token(supply);

        let res = TemplateModule::approve(RuntimeOrigin::signed(ALICE), BOB, 1, approved_amount);

        assert_err!(res, Error::<Test>::NoneToken);

        RuntimeOrigin::signed(ALICE);
    });
}

#[test]
fn can_approve_tokens_for_transfer() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let supply: TokenBalance<Test> = 1000;
        let approved_amount: TokenBalance<Test> = 100;
        let token = create_token(supply);
        let token_id = 1;

        assert_ok!(TemplateModule::mint(
            RuntimeOrigin::signed(ALICE),
            token.name.clone(),
            token.symbol.clone(),
            token.supply
        ));

        assert_ok!(TemplateModule::approve(
            RuntimeOrigin::signed(ALICE),
            BOB,
            1,
            approved_amount
        ));

        System::assert_last_event(
            Event::Approved {
                token_id,
                owner: ALICE,
                spender: BOB,
                amount: approved_amount,
            }
            .into(),
        );

        // allowance is a StorageNMap and so it needs a tuple as an arg
        let allowance = TemplateModule::allowance((1, ALICE, BOB));

        assert_eq!(allowance, approved_amount);
    });
}

#[test]
fn can_transfer_from() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let supply: TokenBalance<Test> = 1000;
        let approved_amount: TokenBalance<Test> = 100;
        let token = create_token(supply);
        let token_id = 1;

        assert_ok!(TemplateModule::mint(
            RuntimeOrigin::signed(ALICE),
            token.name.clone(),
            token.symbol.clone(),
            token.supply
        ));

        assert_ok!(TemplateModule::approve(
            RuntimeOrigin::signed(ALICE),
            BOB,
            token_id,
            approved_amount
        ));

        assert_eq!(TemplateModule::balance_of(token_id, ALICE), supply);

        assert_ok!(TemplateModule::transfer_from(
            RuntimeOrigin::signed(BOB),
            token_id,
            ALICE,
            DAVE,
            approved_amount
        ));

        System::assert_last_event(
            Event::TransferredFrom {
                spender: BOB,
                from: ALICE,
                to: DAVE,
                amount: approved_amount,
            }
            .into(),
        );

        assert_eq!(
            TemplateModule::balance_of(token_id, ALICE),
            supply - approved_amount
        );

        assert_eq!(TemplateModule::balance_of(token_id, DAVE), approved_amount);
    });
}

use crate::{mock::*, Error};
use frame_support::{
    assert_noop, assert_ok,
    traits::{LockableCurrency, OnFinalize, OnInitialize, WithdrawReasons},
};
use frame_system::RawOrigin;
use sp_runtime::traits::SaturatedConversion;
use sp_runtime::DispatchError;

#[test]
fn add_validator_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(TFTBridgeModule::add_bridge_validator(
            RawOrigin::Root.into(),
            bob()
        ));
        assert_ok!(TFTBridgeModule::add_bridge_validator(
            RawOrigin::Root.into(),
            alice()
        ));
    });
}

#[test]
fn add_validator_non_root_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TFTBridgeModule::add_bridge_validator(RuntimeOrigin::signed(alice()), bob()),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn removing_validator_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(TFTBridgeModule::add_bridge_validator(
            RawOrigin::Root.into(),
            bob()
        ));
        assert_ok!(TFTBridgeModule::remove_bridge_validator(
            RawOrigin::Root.into(),
            bob()
        ));
    });
}

#[test]
fn mint_flow() {
    new_test_ext().execute_with(|| {
        prepare_validators();

        run_to_block(1);

        assert_ok!(TFTBridgeModule::mint(
            RuntimeOrigin::signed(eve()),
            "some_tx".as_bytes().to_vec(),
            bob(),
            750000000
        ));

        let executed_mint_tx =
            TFTBridgeModule::executed_mint_transactions("some_tx".as_bytes().to_vec()).unwrap();

        assert_eq!(executed_mint_tx.target, bob());
        assert_eq!(executed_mint_tx.amount, 750000000);
        assert_eq!(executed_mint_tx.block, 1);

        let b = TFTBridgeModule::get_usable_balance(&bob());
        let balances_as_u128: u128 = b.saturated_into::<u128>();
        assert_eq!(balances_as_u128, 2750000000);

        if let Some(fee_account) = TFTBridgeModule::fee_account() {
            let b = Balances::free_balance(&fee_account);
            let balances_as_u128: u128 = b.saturated_into::<u128>();
            assert_eq!(balances_as_u128, 500000000);
        }
    });
}

#[test]
fn swap_to_stellar_valid_address_workds() {
    new_test_ext().execute_with(|| {
        assert_ok!(TFTBridgeModule::withdraw_to_stellar(
            RuntimeOrigin::signed(bob()),
            "GBIYYEQO73AYJEADTHMTF5M42WICTHU55IIT2CPEZBBLLDSJ322OGW7Z"
                .as_bytes()
                .to_vec(),
            2000000000
        ));
    });
}

#[test]
fn swap_to_stellar_non_valid_address_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TFTBridgeModule::withdraw_to_stellar(
                RuntimeOrigin::signed(bob()),
                "some_invalid_text".as_bytes().to_vec(),
                2000000000
            ),
            Error::<TestRuntime>::InvalidStellarPublicKey
        );
    });
}

#[test]
fn burn_more_than_balance_plus_fee_fails() {
    new_test_ext().execute_with(|| {
        prepare_validators();

        let b = TFTBridgeModule::get_usable_balance(&bob());
        let balances_as_u128: u128 = b.saturated_into::<u128>();
        assert_eq!(balances_as_u128, 2500000000);

        assert_noop!(
            TFTBridgeModule::withdraw_to_stellar(
                RuntimeOrigin::signed(bob()),
                "GBIYYEQO73AYJEADTHMTF5M42WICTHU55IIT2CPEZBBLLDSJ322OGW7Z"
                    .as_bytes()
                    .to_vec(),
                2500000001
            ),
            Error::<TestRuntime>::NotEnoughBalanceToSwap
        );
    });
}

#[test]
fn burn_locked_tokens_fails() {
    new_test_ext().execute_with(|| {
        prepare_validators();

        let free_balance = Balances::free_balance(&bob());
        assert_eq!(free_balance, 2500000000);

        let locked_balance = 1000000000;
        let id: u64 = 1;
        Balances::set_lock(
            id.to_be_bytes(),
            &bob(),
            locked_balance,
            WithdrawReasons::all(),
        );

        let usable_balance = TFTBridgeModule::get_usable_balance(&bob());
        assert_eq!(usable_balance, 1500000000);

        assert_noop!(
            TFTBridgeModule::withdraw_to_stellar(
                RuntimeOrigin::signed(bob()),
                "GBIYYEQO73AYJEADTHMTF5M42WICTHU55IIT2CPEZBBLLDSJ322OGW7Z"
                    .as_bytes()
                    .to_vec(),
                usable_balance + 1
            ),
            Error::<TestRuntime>::NotEnoughBalanceToSwap
        );
    });
}

#[test]
fn burn_fails_if_less_than_withdraw_fee_amount() {
    new_test_ext().execute_with(|| {
        prepare_validators();

        assert_noop!(
            TFTBridgeModule::withdraw_to_stellar(
                RuntimeOrigin::signed(alice()),
                "GBIYYEQO73AYJEADTHMTF5M42WICTHU55IIT2CPEZBBLLDSJ322OGW7Z"
                    .as_bytes()
                    .to_vec(),
                490000000
            ),
            Error::<TestRuntime>::AmountIsLessThanWithdrawFee
        );
    });
}

fn prepare_validators() {
    TFTBridgeModule::add_bridge_validator(RawOrigin::Root.into(), alice()).unwrap();
    TFTBridgeModule::add_bridge_validator(RawOrigin::Root.into(), bob()).unwrap();
    TFTBridgeModule::add_bridge_validator(RawOrigin::Root.into(), eve()).unwrap();
    TFTBridgeModule::add_bridge_validator(RawOrigin::Root.into(), ferdie()).unwrap();

    TFTBridgeModule::set_fee_account(RawOrigin::Root.into(), ferdie()).unwrap();
    TFTBridgeModule::set_deposit_fee(RawOrigin::Root.into(), 500000000).unwrap();
    TFTBridgeModule::set_withdraw_fee(RawOrigin::Root.into(), 500000000).unwrap();
}

fn run_to_block(n: u64) {
    while System::block_number() < n {
        TFTBridgeModule::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        TFTBridgeModule::on_initialize(System::block_number());
    }
}

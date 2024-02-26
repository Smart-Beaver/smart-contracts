use psp22_wrapper_tests::traits::PSP22Wrapper;
use psp22_wrapper_tests::token::TokenRef;
use psp22_wrapper_tests::PSP22;

#[cfg(all(test, feature = "e2e-tests"))]
mod tests {
    use std::hash::Hash;
    use super::*;
    use ink_e2e::build_message;
    use ink_e2e::subxt::tx::Signer;
    use ink::primitives::AccountId;
    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    #[ignore]
    //@FIXME failing with error: deploy function isn't exported
    async fn wrapper_works<T>(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // 1. Deploying both contracts (underlying and wrapper).
        let constructor_underlying = TokenRef::new(10, None);
        let contract_acc_underlying_id = client
            .instantiate("psp22_wrapper_tests", &ink_e2e::bob(), constructor_underlying, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;
        let get_underlying = build_message::<TokenRef>(contract_acc_underlying_id)
            .call(|token| token.total_supply());
        let get_res_underlying = client
            .call(&ink_e2e::bob(), get_underlying, 0, None)
            .await
            .expect("get_res_underlying failed");
        let constructor = TokenRef::new(0, Some(contract_acc_underlying_id));
        let contract_acc_id = client
            .instantiate("psp22_wrapper_tests", &ink_e2e::bob(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // 2. Checking initial balances.
        // a) Bob's balance on underlying contract.
        let get_bob_underlying_balance = build_message::<TokenRef>(contract_acc_underlying_id)
            .call(|token| token.balance_of(get_bob_account_id()));
        let get_bob_balance_underlying_init_res = client
            .call(&ink_e2e::bob(), get_bob_underlying_balance.clone(), 0, None)
            .await
            .expect("get_bob_balance_underlying_init failed");
        assert!(matches!(get_bob_balance_underlying_init_res.return_value(), 10));
        // b) Bob's balance on wrapper contract.
        let get_bob_balance = build_message::<TokenRef>(contract_acc_id.clone())
            .call(|token| token.balance_of(get_bob_account_id()));
        let get_bob_balance_init_res = client
            .call(&ink_e2e::bob(), get_bob_balance.clone(), 0, None)
            .await
            .expect("get_bob_balance_init failed");
        assert!(matches!(get_bob_balance_init_res.return_value(), 0));
        // c) Alice's balance on underlying contract.
        let get_alice_underlying_balance = build_message::<TokenRef>(contract_acc_underlying_id)
            .call(|token| token.balance_of(get_alice_account_id()));
        let get_alice_balance_underlying_init_res = client
            .call(&ink_e2e::bob(), get_alice_underlying_balance.clone(), 0, None)
            .await
            .expect("get_alice_balance_underlying_init failed");
        assert!(matches!(get_alice_balance_underlying_init_res.return_value(), 0));
        // d) Alice's balance on wrapper contract.
        let get_alice_balance = build_message::<TokenRef>(contract_acc_id.clone())
            .call(|token| token.balance_of(get_alice_account_id()));
        let get_alice_balance_init_res = client
            .call(&ink_e2e::bob(), get_alice_balance.clone(), 0, None)
            .await
            .expect("get_alice_balance_init failed");
        assert!(matches!(get_alice_balance_init_res.return_value(), 0));

        // 3. Approving wrapper to spend underlying contract tokens.
        let approve_spend_underlying = build_message::<TokenRef>(contract_acc_underlying_id.clone())
            .call(|token| token.approve(contract_acc_id.clone(), 5));
        client
            .call(&ink_e2e::bob(), approve_spend_underlying, 0, None)
            .await
            .expect("approve_spend failed");

        // 4. Depositing tokens.
        let deposit_for = build_message::<TokenRef>(contract_acc_id.clone())
            .call(|token| token.deposit_for(get_alice_account_id(), 5));
        client.call(&ink_e2e::bob(), deposit_for, 0, None)
            .await.expect("deposit_for failed");

        // 5. Checking balances after deposit.
        // a) Bob deposited 5 of his tokens (...)
        let balance_bob_underlying_after_deposit_res = client
            .call(&ink_e2e::bob(), get_bob_underlying_balance.clone(), 0, None)
            .await.expect("balance_bob_underlying_after_deposit failed");
        assert!(matches!(balance_bob_underlying_after_deposit_res.return_value(), 5));
        // b) (...) and appoint Alice as their assignee. She should've receive 5 Wrapped Tokens.
        let balance_alice_after_deposit_res = client
            .call(&ink_e2e::alice(), get_alice_balance.clone(), 0, None)
            .await.expect("balance_alice_after_deposit failed");
        assert!(matches!(balance_alice_after_deposit_res.return_value(), 5));

        // 6. Since Bob has no WrapperTokens he should not be able to withdraw anything.
        // a) Bob makes an attempt to withdraw someone else's tokens.
        let withdraw_which_should_fail = build_message::<TokenRef>(contract_acc_id.clone())
            .call(|token| token.withdraw_to(get_bob_account_id(), 5));
        let withdraw_which_should_fail_failed = client
            .call(&ink_e2e::bob(), withdraw_which_should_fail, 0, None)
            .await.is_err();
        assert!(withdraw_which_should_fail_failed);
        // b) But his vile plan fails.
        let balance_bob_underlying_after_stealing_attempt_res = client
            .call(&ink_e2e::bob(), get_bob_underlying_balance.clone(), 0, None)
            .await.expect("balance_bob_underlying_after_withdraw failed");
        assert!(matches!(balance_bob_underlying_after_stealing_attempt_res.return_value(), 5));

        // 7. Withdrawing tokens.
        let withdraw_to = build_message::<TokenRef>(contract_acc_id.clone())
            .call(|token| token.withdraw_to(get_bob_account_id(), 5));
        client.call(&ink_e2e::alice(), withdraw_to, 0, None).await.expect("withdraw failed");

        // 8. Checking balances again.
        // a) Alice withdrew her tokens to Bob's account.
        // She should've lost all of her WrappedTokens (...)
        let balance_alice_after_withdraw_res = client
            .call(&ink_e2e::alice(), get_alice_balance.clone(), 0, None)
            .await.expect("balance_alice_after_withdraw failed");
        assert!(matches!(balance_alice_after_withdraw_res.return_value(), 0));
        // b) (...) and Bob should have his underlying tokens back.
        let balance_bob_underlying_after_withdraw_res = client
            .call(&ink_e2e::bob(), get_bob_underlying_balance.clone(), 0, None)
            .await.expect("balance_bob_underlying_after_withdraw failed");
        assert!(matches!(balance_bob_underlying_after_withdraw_res.return_value(), 10));

        Ok(())
    }

    fn get_bob_account_id() -> AccountId {
        ink_e2e::account_id(ink_e2e::AccountKeyring::Bob)
    }

    fn get_alice_account_id() -> AccountId {
        ink_e2e::account_id(ink_e2e::AccountKeyring::Alice)
    }
}

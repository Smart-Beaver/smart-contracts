use psp22_wrapper_tests::token::*;

#[cfg(test)]
mod tests {
    use ink::codegen::Env;
    use ink::primitives::AccountId;
    use ink::env::{DefaultEnvironment, test::set_caller};
    use psp22_wrapper_tests::traits::PSP22Wrapper;
    use super::*;

    #[ink::test]
    #[should_panic(expected = "not implemented: off-chain environment does not support contract invocation")]
    fn deposit_for_in_unit_tests_fails() {
        let owner = AccountId::from([0x1; 32]);
        let deposit_for = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let underlying = Token::new(10, None);

        let mut token = Token::new(10, Some(underlying.env().account_id()));
        token.deposit_for(deposit_for, 5).expect("Will fail for unit tests...");
    }

    #[ink::test]
    #[should_panic(expected = "not implemented: off-chain environment does not support contract invocation")]
    fn withdraw_to_in_unit_tests_works() {
        let owner = AccountId::from([0x1; 32]);
        let account = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let underlying = Token::new(10, None);

        let mut token = Token::new(10, Some(underlying.env().account_id()));
        token.withdraw_to(account, 5).expect("Will fail for unit tests...");
    }
}

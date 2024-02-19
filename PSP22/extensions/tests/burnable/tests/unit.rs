
#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use ink::env::{DefaultEnvironment, test::set_caller};
    use psp22_burnable_tests::PSP22;
    use psp22_burnable_tests::token::Token;
    use psp22_burnable_tests::traits::PSP22Burnable;
    use super::*;

    #[ink::test]
    fn burn_all_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1);
        assert_eq!(token.total_supply(), 1);
        token.burn(1).expect("Success expected");
        assert_eq!(token.total_supply(), 0);
    }

    #[ink::test]
    fn burn_part_of_tokens_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(2);
        assert_eq!(token.total_supply(), 2);
        token.burn(1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn burn_for_no_amount_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1);
        assert_eq!(token.total_supply(), 1);
        token.burn(0).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn burn_for_insufficient_amount_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1);
        assert_eq!(token.total_supply(), 1);
        assert!(token.burn(100).is_err());
    }

    #[ink::test]
    fn burn_from_works() {
        let sender = AccountId::from([0x1; 32]);
        let owner = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(2);
        token.approve(sender, 2).expect("Success expected");
        set_caller::<DefaultEnvironment>(sender);
        assert_eq!(token.total_supply(), 2);
        token.burn_from(owner, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
        token.burn_from(owner, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 0);
    }

    #[ink::test]
    fn burn_from_with_insufficient_allowance_fails() {
        let sender = AccountId::from([0x1; 32]);
        let owner = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(2);
        token.approve(sender, 1).expect("Success expected");
        set_caller::<DefaultEnvironment>(sender);
        assert_eq!(token.total_supply(), 2);
        assert!(token.burn_from(owner, 2).is_err());
        assert_eq!(token.total_supply(), 2);
    }

    #[ink::test]
    fn burn_from_with_insufficient_balance_fails() {
        let sender = AccountId::from([0x1; 32]);
        let owner = AccountId::from([0x2; 32]);
        let third_party = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(2);

        token.approve(sender, 2).expect("Success expected");
        token.transfer(third_party, 1, Vec::from([])).expect("Success expected");

        assert_eq!(token.total_supply(), 2);
        assert!(token.burn_from(owner, 2).is_err());
        assert_eq!(token.total_supply(), 2);
    }

    #[ink::test]
    fn burn_nothing_from_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1);
        token.approve(sender, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
        token.burn_from(sender, 0).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }
}

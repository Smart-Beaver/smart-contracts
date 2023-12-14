use psp22_mintable_tests::token::*;
use psp22_mintable_tests::PSP22;
use psp22_mintable_tests::traits::PSP22Mintable;
#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use ink::env::{DefaultEnvironment, test::set_caller};
    use super::*;

    #[ink::test]
    fn mint_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(0);
        assert_eq!(token.total_supply(), 0);
        token.mint(sender, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn mint_no_amount_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(0);
        token.mint(sender, 0).expect("Success expected");
        assert_eq!(token.total_supply(), 0);
    }

    #[ink::test]
    fn mint_amount_out_of_scope_failed() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(100);
        assert!(token.mint(sender, u128::MAX).is_err());
        assert_eq!(token.total_supply(), 100);
    }
}

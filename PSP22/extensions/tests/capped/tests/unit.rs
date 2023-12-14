use psp22_capped_tests::token::*;
use psp22_capped_tests::PSP22;
use psp22_capped_tests::traits::PSP22Mintable;
#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use ink::env::{DefaultEnvironment, test::set_caller};
    use super::*;

    #[ink::test]
    fn mint_below_cap_limit_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(0, 1);
        assert_eq!(token.total_supply(), 0);
        token.mint(sender, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    #[should_panic(expected = "Max cap exceeded")]
    fn mint_above_cap_limit_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1, 1);
        assert_eq!(token.total_supply(), 1);
        token.mint(sender, 1).expect("Will fail here");
    }
}

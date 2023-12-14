use psp22_pausable_tests::token::*;
use psp22_pausable_tests::PSP22;
use psp22_pausable_tests::traits::PSP22Mintable;
use psp22_pausable_tests::traits::PSP22Pausable;

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use ink::env::{DefaultEnvironment, test::set_caller};
    use super::*;
    #[ink::test]
    fn mint_unpaused_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(0, true);
        token.unpause().expect("Should not fail");
        assert_eq!(token.total_supply(), 0);
        token.mint(sender, 1).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    #[should_panic(expected = "Token is paused")]
    fn mint_paused_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1, false);
        token.pause().expect("Should not fail");
        assert_eq!(token.total_supply(), 1);
        token.mint(sender, 1).expect("ok");
    }

    #[ink::test]
    fn transfer_unpaused_works() {
        let sender = AccountId::from([0x2; 32]);
        let recipient = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1, false);
        assert_eq!(token.balance_of(recipient), 0);
        token.transfer(recipient, 1, Vec::from([])).expect("Success expected");
        assert_eq!(token.balance_of(recipient), 1);
    }

    #[ink::test]
    #[should_panic(expected = "Token is paused")]
    fn transfer_paused_fails() {
        let sender = AccountId::from([0x2; 32]);
        let recipient = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new(1, true);
        assert_eq!(token.balance_of(recipient), 0);
        token.transfer(recipient, 1, Vec::from([])).expect("Will fail");
    }

    #[ink::test]
    fn transfer_from_unpaused_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(3, false);
        let sender = AccountId::from([0x2; 32]);
        token.approve(sender, 3).expect("Success expected");
        let recipient = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(sender);
        assert_eq!(token.balance_of(recipient), 0);
        token.transfer_from(owner, recipient, 1, Vec::from([])).expect("Success expected");
        assert_eq!(token.balance_of(recipient), 1);
    }

    #[ink::test]
    #[should_panic(expected = "Token is paused")]
    fn transfer_from_paused_fails() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(3, true);
        let sender = AccountId::from([0x2; 32]);
        token.approve(sender, 3).expect("Success expected");
        let recipient = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(sender);
        token.transfer_from(owner, recipient, 1, Vec::from([])).expect("Will fail");
    }
}

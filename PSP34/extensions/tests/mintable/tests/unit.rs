use psp34_mintable_tests::PSP34Mintable;
use psp34_mintable_tests::token::*;

#[cfg(test)]
mod tests {
    use ink::env::{DefaultEnvironment, test::set_caller};
    use ink::env::test::advance_block;
    use ink::primitives::AccountId;

    use psp34_mintable_tests::{Id, PSP34, PSP34Error};

    use super::*;

    #[ink::test]
    fn mint_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        //Contract owner is the deployer address
        let mut token = Token::new();
        assert_eq!(token.total_supply(), 0);
        token.mint(Id::U8(0)).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn mint_used_id_same_sender_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        assert_eq!(token.total_supply(), 0);
        token.mint(Id::U8(0)).expect("Success expected");

        assert!(token.mint(Id::U8(0)).is_err());

        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn mint_by_non_owner_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        //Contract owner is the deployer address
        let mut token = Token::new();

        //Create a new account
        let alice = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(alice);

        assert_eq!(token.total_supply(), 0);

        assert_eq!(token.owner.unwrap(), sender);

        advance_block::<DefaultEnvironment>();

        assert_eq!(token.mint(Id::U8(0)), Err(PSP34Error::Custom("NotAnOwner".to_string())));

        //Balance didn't change
        assert_eq!(token.total_supply(), 0);
    }
}
use psp34_enumerable_tests::PSP34Mintable;
use psp34_enumerable_tests::token::*;

#[cfg(test)]
mod tests {
    use ink::env::{DefaultEnvironment, test::set_caller};
    use ink::primitives::AccountId;

    use psp34_enumerable_tests::{Id, PSP34Enumerable, PSP34Error};

    use super::*;

    #[ink::test]
    fn read_token_by_index_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(9)).expect("Success expected");

        assert_eq!(token.token_by_index(0), Ok(Id::U8(9)));
    }

    #[ink::test]
    fn read_token_by_index_non_owner_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(7)).expect("Success expected");

        let alice = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(alice);

        assert_eq!(token.token_by_index(0), Ok(Id::U8(7)));
    }

    #[ink::test]
    fn read_token_by_index_unusual_index_incremental_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(0)).expect("Success expected");
        token.mint(Id::U8(10)).expect("Success expected");
        token.mint(Id::U8(3)).expect("Success expected");

        assert_eq!(token.token_by_index(0), Ok(Id::U8(0)));
        assert_eq!(token.token_by_index(1), Ok(Id::U8(10)));
        assert_eq!(token.token_by_index(2), Ok(Id::U8(3)));
    }

    #[ink::test]
    fn read_token_by_index_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let token = Token::new();

        assert_eq!(token.token_by_index(10), Err(PSP34Error::TokenNotExists));
    }

    #[ink::test]
    fn read_owners_token_by_index_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(5)).expect("Success expected");

        assert_eq!(token.owners_token_by_index(sender, 0), Ok(Id::U8(5)));
    }

    #[ink::test]
    fn read_owners_token_by_index_non_owner_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(5)).expect("Success expected");

        assert_eq!(token.owners_token_by_index(sender, 0), Ok(Id::U8(5)));
    }

    #[ink::test]
    fn read_owners_token_unusual_index_incremental_by_index_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        token.mint(Id::U8(0)).expect("Success expected");
        token.mint(Id::U8(10)).expect("Success expected");
        token.mint(Id::U8(3)).expect("Success expected");

        assert_eq!(token.owners_token_by_index(sender, 0), Ok(Id::U8(0)));
        assert_eq!(token.owners_token_by_index(sender, 1), Ok(Id::U8(10)));
        assert_eq!(token.owners_token_by_index(sender, 2), Ok(Id::U8(3)));
    }

    #[ink::test]
    fn read_owners_token_by_index_fails() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let token = Token::new();

        assert_eq!(token.owners_token_by_index(sender, 10), Err(PSP34Error::TokenNotExists));
    }
}
#[cfg(test)]
mod tests {
    use ink::env::{DefaultEnvironment, test::set_caller};
    use ink::primitives::AccountId;

    use psp34_burnable_tests::{Id, PSP34, PSP34Burnable, PSP34Error, PSP34Mintable};
    use psp34_burnable_tests::token::Token;

    #[ink::test]
    fn burn_all_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();
        let example_token_id = Id::U8(0);
        token.mint(example_token_id.clone()).expect("Success expected");
        assert_eq!(token.total_supply(), 1);

        token.burn(sender, example_token_id).expect("Success expected");

        assert_eq!(token.total_supply(), 0);
    }

    #[ink::test]
    fn burn_part_of_tokens_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();
        token.mint(Id::U8(0)).expect("Success expected");
        token.mint(Id::U8(1)).expect("Success expected");

        assert_eq!(token.total_supply(), 2);
        token.burn(sender, Id::U8(1)).expect("Success expected");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn burn_for_not_existing_token_fails_1() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        assert_eq!(token.total_supply(), 0);
        assert_eq!(token.burn(sender, Id::U8(1)), Err(PSP34Error::TokenNotExists));
        assert_eq!(token.total_supply(), 0);
    }

    #[ink::test]
    fn burn_for_not_existing_token_fails_2() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();
        token.mint(Id::U8(0)).expect("Success expected");

        assert_eq!(token.total_supply(), 1);
        assert!(token.burn(sender, Id::U8(23)).is_err());
        assert_eq!(token.total_supply(), 1);
    }

}
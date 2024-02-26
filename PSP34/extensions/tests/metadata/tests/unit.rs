use psp34_metadata_tests::PSP34Mintable;
use psp34_metadata_tests::token::*;

#[cfg(test)]
mod tests {
    use ink::env::{DefaultEnvironment, test::set_caller};
    use ink::primitives::AccountId;

    use psp34_metadata_tests::{Id, PSP34Metadata};

    use super::*;

    #[ink::test]
    fn metadata_read_non_existing_token_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let mut token = Token::new();

        //Choose any token that doesn't exist
        let example_id = Id::U8(0);

        //We expect the token to have no metadata
        //We should get None as a result instead of a error
        assert!(token.get_attribute(example_id, "test".into()).is_none());
    }

    #[ink::test]
    fn metadata_read_on_existing_token_works() {
        let sender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(sender);
        //Contract owner is the deployer address
        let mut token = Token::new();

        let new_token_id = Id::U8(0);

        token.mint(new_token_id.clone()).expect("Success expected");

        //We expect the token to have no metadata
        assert!(token.get_attribute(new_token_id.clone(), "test".into()).is_none());
    }
}
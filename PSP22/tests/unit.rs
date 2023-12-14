use psp22::token::*;
use psp22::PSP22;

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;
    use super::*;
    use ink::env::{DefaultEnvironment, test::set_caller};

    #[ink::test]
    fn total_supply_works() {
        let token = Token::new(1);
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn transfer_all_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let recipient = AccountId::from([0x2; 32]);

        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);

        token.transfer(recipient, 1, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 0);
        assert_eq!(token.balance_of(recipient), 1);
    }

    #[ink::test]
    fn transfer_part_of_tokens_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let recipient = AccountId::from([0x2; 32]);

        let mut token = Token::new(2);
        assert_eq!(token.balance_of(sender), 2);
        assert_eq!(token.balance_of(recipient), 0);

        token.transfer(recipient, 1, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 1);
    }

    #[ink::test]
    fn transfer_nothing_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let recipient = AccountId::from([0x2; 32]);

        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);

        token.transfer(recipient, 0, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);
    }

    #[ink::test]
    fn transfer_to_self_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);

        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);

        token.transfer(sender, 1, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 1);
    }

    #[ink::test]
    fn transfer_with_insufficient_funds_fails() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let recipient = AccountId::from([0x2; 32]);
        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);
        assert!(token.transfer(recipient, 2, Vec::from([1])).is_err());
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);
    }

    #[ink::test]
    fn transfer_from_self_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);
        let recipient = AccountId::from([0x2; 32]);

        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);
        assert_eq!(token.balance_of(recipient), 0);

        token.transfer_from(sender, recipient, 1, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 0);
        assert_eq!(token.balance_of(recipient), 1);
    }

    #[ink::test]
    fn transfer_from_to_self_works() {
        let sender = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(sender);

        let mut token = Token::new(1);
        assert_eq!(token.balance_of(sender), 1);

        token.transfer_from(sender, sender, 1, Vec::from([1])).expect("Success expected");
        assert_eq!(token.balance_of(sender), 1);
    }

    #[ink::test]
    fn transfer_from_with_no_allowance_fails() {
        let owner = AccountId::from([0x1; 32]);
        let spender = AccountId::from([0x2; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let recipient = AccountId::from([0x3; 32]);
        let mut token = Token::new(1);
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.balance_of(owner), 1);
        assert_eq!(token.balance_of(recipient), 0);
        set_caller::<DefaultEnvironment>(spender);
        assert!(token.transfer_from(owner, recipient, 1, Vec::from([1])).is_err());
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.balance_of(owner), 1);
        assert_eq!(token.balance_of(recipient), 0);
    }

    #[ink::test]
    fn transfer_from_with_insufficient_funds_fails() {
        let owner = AccountId::from([0x1; 32]);
        let spender = AccountId::from([0x2; 32]);
        let recipient = AccountId::from([0x3; 32]);
        let third_party = AccountId::from([0x4; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(1);
        token.approve(spender, 1).expect("Cant fail here");
        token.transfer(third_party, 1, Vec::from([1])).expect("Cant fail here");
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.balance_of(owner), 0);
        assert_eq!(token.balance_of(recipient), 0);
        set_caller::<DefaultEnvironment>(spender);
        assert!(token.transfer_from(owner, recipient, 1, Vec::from([1])).is_err());
        assert_eq!(token.balance_of(owner), 0);
        assert_eq!(token.balance_of(recipient), 0);
        assert_eq!(token.balance_of(spender), 0);
    }

    #[ink::test]
    fn transfer_from_with_allowance_works() {
        let owner = AccountId::from([0x1; 32]);
        let spender = AccountId::from([0x2; 32]);
        let recipient = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(1);
        token.approve(spender, 1).expect("Cant fail here");
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.balance_of(owner), 1);
        assert_eq!(token.balance_of(recipient), 0);
        set_caller::<DefaultEnvironment>(spender);
        token.transfer_from(owner, recipient, 1, Vec::from([1])).expect("Transfer should be allowed");
        assert_eq!(token.balance_of(owner), 0);
        assert_eq!(token.balance_of(recipient), 1);
        assert_eq!(token.balance_of(spender), 0);
    }

    #[ink::test]
    fn transfer_from_with_only_part_of_allowance_used_works() {
        let owner = AccountId::from([0x1; 32]);
        let spender = AccountId::from([0x2; 32]);
        let recipient = AccountId::from([0x3; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(2);
        token.approve(spender, 2).expect("Cant fail here");
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.balance_of(owner), 2);
        assert_eq!(token.balance_of(recipient), 0);
        set_caller::<DefaultEnvironment>(spender);
        assert_eq!(token.allowance(owner, spender), 2);
        token.transfer_from(owner, recipient, 1, Vec::from([1])).expect("Transfer should be allowed");
        assert_eq!(token.balance_of(owner), 1);
        assert_eq!(token.balance_of(recipient), 1);
        assert_eq!(token.balance_of(spender), 0);
        assert_eq!(token.allowance(owner, spender), 1);
    }

    #[ink::test]
    fn approval_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let spender = AccountId::from([0x2; 32]);

        let mut token = Token::new(1);
        assert_eq!(token.allowance(owner, spender), 0);

        token.approve(spender, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 1);
    }

    #[ink::test]
    fn increase_decrease_allowance_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let spender = AccountId::from([0x2; 32]);

        let mut token = Token::new(1);
        assert_eq!(token.allowance(owner, spender), 0);

        token.approve(spender, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 1);

        token.increase_allowance(spender, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 2);

        token.decrease_allowance(spender, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 1);

        token.decrease_allowance(spender, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 0);

        assert!(token.decrease_allowance(spender, 1).is_err());
        assert_eq!(token.allowance(owner, spender), 0);
    }

    #[ink::test]
    fn increase_decrease_self_allowance_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);

        let mut token = Token::new(1);

        token.approve(owner, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, owner), 0);

        token.increase_allowance(owner, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, owner), 0);

        token.decrease_allowance(owner, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, owner), 0);
    }

    #[ink::test]
    fn approve_self_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(1);
        token.approve(owner, 1).expect("Success expected");
        assert_eq!(token.allowance(owner, owner), 0);
    }

    #[ink::test]
    fn disapprove_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let spender = AccountId::from([0x2; 32]);
        let mut token = Token::new(66);
        assert_eq!(token.allowance(owner, spender), 0);
        token.approve(spender, 66).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 66);

        token.approve(spender, 0).expect("Success expected");
        assert_eq!(token.allowance(owner, spender), 0);
    }

    #[ink::test]
    fn minting_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(0);
        assert_eq!(token.total_supply(), 0);
        token.data.mint(owner,1).expect("Minting should be possible");
        assert_eq!(token.total_supply(), 1);
    }

    #[ink::test]
    fn burning_works() {
        let owner = AccountId::from([0x1; 32]);
        set_caller::<DefaultEnvironment>(owner);
        let mut token = Token::new(1);
        assert_eq!(token.total_supply(), 1);
        token.data.burn(owner,1).expect("Burning should be possible");
        assert_eq!(token.total_supply(), 0);
    }
}

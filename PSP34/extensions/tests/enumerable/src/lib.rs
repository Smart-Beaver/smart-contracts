
#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use data::{Id, PSP34Data, PSP34Event};
pub use errors::PSP34Error;
pub use traits::{PSP34, PSP34Burnable, PSP34Enumerable, PSP34Metadata, PSP34Mintable};
mod data;
mod errors;
mod traits;
mod unit_tests;
mod test_utils;

#[cfg(feature = "contract")]
#[ink::contract]
pub mod token {
    use crate::traits::Ownable;
    use crate::errors::OwnableError;
    use crate::traits::PSP34Mintable;
    use crate::traits::PSP34Enumerable;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;
    use crate::{Id, PSP34, PSP34Data, PSP34Error, PSP34Event};
    use crate::data::Data;

    #[ink(storage)]
    pub struct Token {
        pub data: PSP34Data,
        pub owner: Option<AccountId>,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                data: PSP34Data::new(),
                owner: Some(Self::env().caller()),
            }
        }

        fn emit_events(&self, events: ink::prelude::vec::Vec<PSP34Event>) {
            for event in events {
                match event {
                    PSP34Event::Approval { owner, operator, id, approved } => {
                        self.env()
                            .emit_event(Approval {
                                owner,
                                operator,
                                id,
                                approved,
                            })
                    }
                    PSP34Event::Transfer { from, to, id } => {
                        self.env().emit_event(Transfer { from, to, id })
                    }
                    PSP34Event::AttributeSet { id, key, data } => {
                        self.env().emit_event(AttributeSet { id, key, data })
                    }
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl PSP34 for Token {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            self.data.collection_id(self.env().account_id())
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(
            &self,
            owner: AccountId,
            operator: AccountId,
            id: Option<Id>,
        ) -> bool {
            self.data.allowance(owner, operator, id.as_ref())
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            data: ink::prelude::vec::Vec<u8>,
        ) -> Result<(), PSP34Error> {
            let events = self.data.transfer(self.env().caller(), to, id, data)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let events = self.data.approve(self.env().caller(), operator, id, approved)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.data.owner_of(&id)
        }
    }

    impl PSP34Enumerable for Token {
        #[ink(message)]
        fn owners_token_by_index(
            &self,
            owner: AccountId,
            index: u128,
        ) -> Result<Id, PSP34Error> {
            self.data.owners_token_by_index(owner, index)
        }

        #[ink(message)]
        fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
            self.data.token_by_index(index)
        }
    }

    impl PSP34Mintable for Token {
        #[ink(message)]
        fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            if self.owner != Some(caller) {
                return Err(PSP34Error::Custom(OwnableError::NotAnOwner.to_string()));
            }
            let events = self.data.mint(caller, id)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl Ownable for Token {
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.owner
        }

        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
            assert_eq!(
                Some(self.env().caller()), self.owner,
                "Only owner can renounce ownership"
            );
            self.owner = None;
            Ok(())
        }

        #[ink(message)]
        fn transfer_ownership(
            &mut self,
            new_owner: Option<AccountId>,
        ) -> Result<(), OwnableError> {
            assert_eq!(
                Some(self.env().caller()), self.owner,
                "Only owner can transfer ownership"
            );
            self.owner = new_owner;
            Ok(())
        }
    }
}
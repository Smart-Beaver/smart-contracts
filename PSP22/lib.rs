//! Main module for the PSP22 token implementation.
//!
//! This module defines the main `Token` struct and re-exports key components from other modules.
#![cfg_attr(not(feature = "std"), no_std, no_main)]



pub mod data;
pub mod errors;
pub mod traits;

pub use data::{PSP22Data, PSP22Event};
pub use errors::PSP22Error;
pub use traits::PSP22;

/// PSP22 token implementation.
///
/// This struct represents a PSP22 compliant fungible token.
#[cfg(feature = "contract")]
#[ink::contract]
pub mod token {
    use ink::prelude::vec::Vec;

    use crate::{PSP22, PSP22Data, PSP22Error, PSP22Event};

    #[ink(storage)]
    pub struct Token {
        pub data: PSP22Data,
    }

    impl Token {
        /// Creates a new PSP22 token with a specified initial supply.
        ///
        /// # Arguments
        ///
        /// * `supply` - The total number of tokens to be issued initially.
        ///
        /// # Returns
        ///
        /// A new instance of `Token`.
        #[ink(constructor)]
        pub fn new(
            supply: u128,
        ) -> Self {
            Self {
                data: PSP22Data::new(supply, Self::env().caller()),
            }
        }

        /// Emits specified PSP22 events.
        ///
        /// # Arguments
        ///
        /// * `events` - A vector of `PSP22Event` to be emitted.
        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    impl PSP22 for Token {
        /// Returns the total supply of tokens.
        ///
        /// # Returns
        ///
        /// The total number of tokens in existence.
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        /// Gets the balance of the specified address.
        ///
        /// # Arguments
        ///
        /// * `owner` - The address to query the balance of.
        ///
        /// # Returns
        ///
        /// Number of tokens owned by the given address.
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        /// Gets the amount of tokens that an owner allowed to a spender.
        ///
        /// # Arguments
        ///
        /// * `owner` - The address which owns the funds.
        /// * `spender` - The address which will spend the funds.
        ///
        /// # Returns
        ///
        /// The number of tokens still available for the spender.
        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        /// Transfers tokens to a specified address.
        ///
        /// This method moves the `value` amount of tokens from the caller's account
        /// to the `to` account.
        ///
        /// # Arguments
        ///
        /// * `to` - The address of the recipient.
        /// * `value` - The amount of tokens to be transferred.
        /// * `_data` - Additional data passed with the transfer.
        ///
        /// # Returns
        ///
        /// An `Ok(())` if the transfer is successful, otherwise a `PSP22Error`.
        ///
        /// # Events
        ///
        /// Emits a `Transfer` event on successful transfer.
        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        /// Transfers tokens from one address to another.
        ///
        /// This method moves the `value` amount of tokens from the `from` account
        /// to the `to` account using the allowance mechanism. The caller must
        /// have an allowance from `from` for at least `value` tokens.
        ///
        /// # Arguments
        ///
        /// * `from` - The address of the sender.
        /// * `to` - The address of the recipient.
        /// * `value` - The amount of tokens to be transferred.
        /// * `_data` - Additional data passed with the transfer.
        ///
        /// # Returns
        ///
        /// An `Ok(())` if the transfer is successful, otherwise a `PSP22Error`.
        ///
        /// # Events
        ///
        /// Emits a `Transfer` event and potentially an `Approval` event on successful transfer.
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        /// Approves the passed address to spend the specified amount of tokens on behalf of the caller.
        ///
        /// # Arguments
        ///
        /// * `spender` - The address which will spend the funds.
        /// * `value` - The amount of tokens to be spent.
        ///
        /// # Returns
        ///
        /// An `Ok(())` if the approval is successful, otherwise a `PSP22Error`.
        ///
        /// # Events
        ///
        /// Emits an `Approval` event on successful approval.
        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        /// Increases the allowance granted to a spender.
        ///
        /// This method adds the `delta_value` to the allowance the caller has granted
        /// to the `spender`.
        ///
        /// # Arguments
        ///
        /// * `spender` - The address which will spend the funds.
        /// * `delta_value` - The amount by which the allowance is to be increased.
        ///
        /// # Returns
        ///
        /// An `Ok(())` if the increase is successful, otherwise a `PSP22Error`.
        ///
        /// # Events
        ///
        /// Emits an `Approval` event with the new allowance amount.
        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        /// Decreases the allowance granted to a spender.
        ///
        /// This method subtracts the `delta_value` from the allowance the caller has
        /// granted to the `spender`.
        ///
        /// # Arguments
        ///
        /// * `spender` - The address which will spend the funds.
        /// * `delta_value` - The amount by which the allowance is to be decreased.
        ///
        /// # Returns
        ///
        /// An `Ok(())` if the decrease is successful, otherwise a `PSP22Error`.
        ///
        /// # Events
        ///
        /// Emits an `Approval` event with the new allowance amount.
        ///
        /// # Errors
        ///
        /// Reverts with `InsufficientAllowance` if the `delta_value` exceeds the current allowance.
        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }
}

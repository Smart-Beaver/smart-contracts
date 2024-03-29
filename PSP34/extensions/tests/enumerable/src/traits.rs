use ink::{
    prelude::vec::Vec,
    primitives::AccountId,
};

use crate::data::Id;
use crate::errors::{OwnableError, PSP34Error};

#[ink::trait_definition]
pub trait PSP34 {
    /// Returns the collection `Id` of the NFT token.
    ///
    /// This can represents the relationship between tokens/contracts/pallets.
    #[ink(message)]
    fn collection_id(&self) -> Id;

    /// Returns the current total supply of the NFT.
    #[ink(message)]
    fn total_supply(&self) -> u128;

    /// Returns the account balance for the specified `owner`.
    ///
    /// This represents the amount of unique tokens the owner has.
    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    /// Returns `true` if the operator is approved by the owner to withdraw `id` token.
    ///
    /// If `id` is `None`, returns `true` if the operator is approved to withdraw all owner's tokens.
    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    /// Transfer approved or owned token from caller.
    ///
    /// On success a `Transfer` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `TokenNotExists` error if `id` does not exist.
    ///
    /// Returns `NotApproved` error if `from` doesn't have allowance for transferring.
    ///
    /// Returns `SafeTransferCheckFailed` error if `to` doesn't accept transfer.
    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    /// Approves `operator` to withdraw  the `id` token from the caller's account.
    /// If `id` is `None` approves or disapproves the operator for all tokens of the caller.
    ///
    /// An `Approval` event is emitted.
    ///
    /// # Errors
    ///
    /// Returns `SelfApprove` error if it is self approve.
    ///
    /// Returns `NotApproved` error if caller is not owner of `id`.
    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    /// Returns the owner of the token if any.
    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;
}

#[ink::trait_definition]
pub trait PSP34Metadata {
    /// Returns the attribute of `id` for the given `key`.
    ///
    /// If `id` is a collection id of the token, it returns attributes for collection.
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}

#[ink::trait_definition]
pub trait PSP34Mintable {
    /// Mints a token to the sender's account.
    ///
    /// # Events
    ///
    /// On success a `Transfer` event is emitted with `None` sender.
    ///
    /// # Errors
    ///
    /// Reverts with `TokenExists`` if token id is already in the library.
    ///
    /// Reverts with `Custom (max supply exceeded)` if the incremented by 1 total
    /// supply exceeds maximal value of `u128` type.
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP34Error>;
}

#[ink::trait_definition]
pub trait PSP34Burnable {
    /// Burns token from the selected account.
    ///
    /// # Events
    ///
    /// On success a `Transfer` event is emitted with `None` recipient.
    ///
    /// # Errors
    ///
    /// Reverts with `TokenExists` if token id is already in the library.
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}

#[ink::trait_definition]
pub trait PSP34Enumerable {
    /// Returns a token `Id` owned by `owner` at a given `index` of its token list.
    /// Use along with `balance_of` to enumerate all of ``owner``'s tokens.
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    /// Returns a token `Id` at a given `index` of all the tokens stored by the contract.
    /// Use along with `total_supply` to enumerate all tokens.
    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}

/// Trait for ownership-related functionalities.
///
/// Provides methods for managing ownership of the contract, including
/// transferring and renouncing ownership.
#[ink::trait_definition]
pub trait Ownable {
    /// Returns the address of the current owner.
    ///
    /// # Returns
    ///
    /// The `AccountId` of the current owner.
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;

    /// Renounces ownership of the contract.
    ///
    /// This method is used to permanently transfer control of the contract
    /// away from the current owner, leaving it without an owner.
    ///
    /// # Returns
    ///
    /// A `Result<(), OwnableError>` indicating whether the operation was successful.
    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), OwnableError>;

    /// Transfers ownership of the contract to a new account.
    ///
    /// # Arguments
    ///
    /// * `new_owner` - The `AccountId` of the new owner.
    ///
    /// # Returns
    ///
    /// A `Result<(), OwnableError>` indicating whether the operation was successful.
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: Option<AccountId>) -> Result<(), OwnableError>;
}

/// Trait for pausing and unpausing token transfers.
///
/// This trait allows the contract owner to pause or unpause token transfers,
/// which can be useful in emergency situations or during maintenance.
#[ink::trait_definition]
pub trait PSP22Pausable {
    /// Pauses all token transfers.
    ///
    /// This method is used to temporarily halt all transfer operations.
    ///
    /// # Returns
    ///
    /// A `Result<(), PSP22Error>` indicating whether the operation was successful.
    #[ink(message)]
    fn pause(&mut self) -> Result<(), PSP34Error>;

    /// Unpauses all token transfers.
    ///
    /// This method re-enables token transfer operations.
    ///
    /// # Returns
    ///
    /// A `Result<(), PSP22Error>` indicating whether the operation was successful.
    #[ink(message)]
    fn unpause(&mut self) -> Result<(), PSP34Error>;
}


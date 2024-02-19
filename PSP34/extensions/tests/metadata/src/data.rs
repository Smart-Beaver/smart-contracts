use ink::{
    prelude::{string::String, vec, vec::Vec},
    primitives::AccountId,
    storage::Mapping,
};
#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

use crate::data::balance_manager::Balances;
use crate::PSP34Error;

/// Type for a PSP34 token id.
/// Contains all the possible permutations of id according to the standard.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
pub enum Id {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Bytes(Vec<u8>),
}

/// Temporary type for events emitted during operations that change the
/// state of PSP34Data struct.
/// This is meant to be replaced with proper ink! events as soon as the
/// language allows for event definitions outside contracts.
pub enum PSP34Event {
    Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        id: Id,
    },
    Approval {
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    },
    AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    },
}

/// A class implementing the internal logic of a PSP34 token.
//
/// Holds the state of all account balances and approvals.
/// Each method of this class corresponds to one type of transaction
/// as defined in the PSP34 standard.
//
/// Since this code is outside of `ink::contract` macro, the caller's
/// address cannot be obtained automatically. Because of that, all
/// the methods that need to know the caller require an additional argument
/// (compared to transactions defined by the PSP34 standard or the PSP34 trait).
//
/// `lib.rs` contains an example implementation of a smart contract using this class.
#[ink::storage_item]
#[derive(Debug, Default)]
pub struct PSP34Data {
    token_owner: Mapping<Id, AccountId>,
    operator_approvals: Mapping<(AccountId, AccountId, Option<Id>), ()>,
    balance: Balances,
}

impl PSP34Data {
    /// Creates a token with default values for every field.
    /// Initially held by the 'creator' account.
    pub fn new() -> PSP34Data {
        Default::default()
    }

    pub fn total_supply(&self) -> u128 {
        self.balance.total_supply()
    }

    pub fn balance_of(&self, owner: AccountId) -> u32 {
        self.balance.balance_of(&owner)
    }

    pub fn owner_of(&self, id: &Id) -> Option<AccountId> {
        self.token_owner.get(id)
    }

    pub fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<&Id>) -> bool {
        self.operator_approvals
            .get((owner, operator, &None))
            .is_some()
            || id.is_some() && self.operator_approvals.get((owner, operator, id)).is_some()
    }

    pub fn collection_id(&self, account_id: AccountId) -> Id {
        Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
    }


    /// Sets a new `approved` for a token `id` or for all tokens if no `id` is provided,
    /// granted by `caller` to `operator`.
    /// Overwrites the previously granted value.
    pub fn approve(
        &mut self,
        mut caller: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        if let Some(id) = &id {
            let owner = self.owner_of(id).ok_or(PSP34Error::TokenNotExists)?;
            if approved && owner == operator {
                return Err(PSP34Error::SelfApprove);
            }

            if owner != caller && !self.allowance(owner, caller, None) {
                return Err(PSP34Error::NotApproved);
            }

            if !approved && self.allowance(owner, operator, None) {
                return Err(PSP34Error::Custom(String::from(
                    "Cannot revoke approval for a single token, when the operator has approval for all tokens."
                )));
            }
            caller = owner;
        }

        if approved {
            self.operator_approvals
                .insert((caller, operator, id.as_ref()), &());
        } else {
            self.operator_approvals
                .remove((caller, operator, id.as_ref()));
        }

        Ok(vec![PSP34Event::Approval {
            owner: caller,
            operator,
            id,
            approved,
        }])
    }

    /// Transfers `value` tokens from `caller` to `to`.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        id: Id,
        _data: Vec<u8>,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        let owner = self.owner_of(&id).ok_or(PSP34Error::TokenNotExists)?;

        if owner == to {
            return Ok(vec![]);
        }

        if owner != caller && !self.allowance(owner, caller, Some(&id)) {
            return Err(PSP34Error::NotApproved);
        }

        self.balance.decrease_balance(&owner, &id, false);

        self.operator_approvals.remove((owner, caller, Some(&id)));
        self.token_owner.remove(&id);

        self.token_owner.insert(&id, &to);
        self.balance.increase_balance(&to, &id, false)?;

        Ok(vec![PSP34Event::Transfer {
            from: Some(caller),
            to: Some(to),
            id,
        }])
    }

    /// Mints a token `id` to `account`.
    pub fn mint(&mut self, account: AccountId, id: Id) -> Result<Vec<PSP34Event>, PSP34Error> {
        if self.owner_of(&id).is_some() {
            return Err(PSP34Error::TokenExists);
        }
        self.balance.increase_balance(&account, &id, true)?;
        self.token_owner.insert(&id, &account);

        Ok(vec![PSP34Event::Transfer {
            from: None,
            to: Some(account),
            id,
        }])
    }

    /// Burns token `id` from `account`, conducted by `caller`
    pub fn burn(
        &mut self,
        caller: AccountId,
        account: AccountId,
        id: Id,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        if self.owner_of(&id).is_none() {
            return Err(PSP34Error::TokenNotExists);
        }
        if account != caller && !self.allowance(caller, account, None) {
            return Err(PSP34Error::NotApproved);
        }
        self.balance.decrease_balance(&account, &id, true);
        self.token_owner.remove(&id);

        Ok(vec![PSP34Event::Transfer {
            from: Some(account),
            to: None,
            id,
        }])
    }

    pub fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
        self.balance.owners_token_by_index(owner, index)
    }

    pub fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
        self.balance.token_by_index(index)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::U128(0)
    }
}

impl From<Id> for u128 {
    fn from(id: Id) -> Self {
        match id {
            Id::U8(val) => val as u128,
            Id::U16(val) => val as u128,
            Id::U32(val) => val as u128,
            Id::U64(val) => val as u128,
            Id::U128(val) => val,
            Id::Bytes(val) => u128::from_be_bytes(val.as_slice().try_into().unwrap()),
        }
    }
}

//For non-enumerable version only visit: https://github.com/Cardinal-Cryptography/PSP34/blob/main/data.rs


pub mod balance_manager {
    use ink::{prelude::vec::Vec, primitives::AccountId, storage::Mapping};

    use crate::{data::Id, PSP34Error};

    #[ink::storage_item]
    #[derive(Default, Debug)]
    pub struct Balances {
        enumerable: Mapping<Option<AccountId>, Vec<Id>>,
    }

    impl Balances {
        pub fn new() -> Balances {
            Default::default()
        }

        pub fn owners_token_by_index(
            &self,
            owner: AccountId,
            index: u128,
        ) -> Result<Id, PSP34Error> {
            self._get_value(&Some(owner), index)
                .ok_or(PSP34Error::TokenNotExists)
        }

        pub fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
            self._get_value(&None, index)
                .ok_or(PSP34Error::TokenNotExists)
        }

        fn _get_value(&self, key: &Option<AccountId>, index: u128) -> Option<Id> {
            self.enumerable
                .get(key)
                .and_then(|values| values.get(usize::try_from(index).unwrap()).cloned())
        }

        fn _insert(&mut self, key: &Option<AccountId>, value: &Id) {
            let mut values = self.enumerable.get(key).unwrap_or_default();
            values.push(value.clone());
            self.enumerable.insert(key, &values);
        }

        pub fn insert(&mut self, key: &Option<AccountId>, value: &Id){
            self._insert(key, value);
        }

        fn _remove(&mut self, key: &Option<AccountId>, value: &Id) {
            if let Some(mut values) = self.enumerable.get(key) {
                if let Some(pos) = values.iter().position(|v| v == value) {
                    values.swap_remove(pos);
                    self.enumerable.insert(key, &values);
                }
            }
        }

        fn _count(&self, key: &Option<AccountId>) -> u128 {
            self.enumerable
                .get(key)
                .map_or(0, |values| values.len())
                .try_into()
                .unwrap()
        }

        pub fn balance_of(&self, owner: &AccountId) -> u32 {
            self._count(&Some(*owner)) as u32
        }

        pub fn increase_balance(
            &mut self,
            owner: &AccountId,
            id: &Id,
            increase_supply: bool,
        ) -> Result<(), PSP34Error> {
            self._insert(&Some(*owner), id);
            if increase_supply {
                self._insert(&None, id);
            }

            Ok(())
        }

        pub fn decrease_balance(&mut self, owner: &AccountId, id: &Id, decrease_supply: bool) {
            self._remove(&Some(*owner), id);
            if self.balance_of(owner) == 0 {
                self.enumerable.remove(Some(owner));
            }
            if decrease_supply {
                self._remove(&None, id);
            }
        }

        pub fn total_supply(&self) -> u128 {
            self._count(&None)
        }
    }
}

#[ink::storage_item]
#[derive(Default, Debug)]
pub struct Data {
    attributes: Mapping<(Id, Vec<u8>), Vec<u8>>,
}

impl Data {
    pub fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
        self.attributes.get((&id, &key))
    }

    pub fn set_attribute(
        &mut self,
        id: Id,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<Vec<PSP34Event>, PSP34Error> {
        self.attributes.insert((&id, &key), &value);
        Ok(vec![PSP34Event::AttributeSet {
            id,
            key,
            data: value,
        }])
    }
}
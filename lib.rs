#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod aaw {
    use ink::prelude::{vec::Vec, string::String};
    use psp34::{
        types::Id, PSP34Data, PSP34Enumerable, PSP34Error, PSP34Event, PSP34Metadata,
        PSP34Mintable, PSP34,
    };

    #[ink(storage)]
    pub struct Aaw {
        psp34: PSP34Data,
        owner: AccountId,
    }

    impl Aaw {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                psp34: PSP34Data::new(10_000),
                owner: Self::env().caller(),
            }
        }

        fn emit_events(&self, events: Vec<PSP34Event>) {
            for event in events {
                match event {
                    PSP34Event::Transfer { from, to, id } => {
                        self.env().emit_event(Transfer { from, to, id })
                    }
                    PSP34Event::Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    } => self.env().emit_event(Approval {
                        owner,
                        operator,
                        id,
                        approved,
                    }),
                    PSP34Event::AttributeSet { id, key, data } => {
                        self.env().emit_event(AttributeSet { id, key, data })
                    }
                }
            }
        }
    }

    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        id: Id,
    }

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl PSP34 for Aaw {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            let account_id = self.env().account_id();
            let collection_id = Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec());
            collection_id
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.psp34.balance_of(owner)
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.psp34.owner_of(id)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            self.psp34.allowance(owner, operator, id)
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let events = self
                .psp34
                .approve(self.env().caller(), operator, id, approved)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
            let events = self.psp34.transfer(self.env().caller(), to, id, data)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Result<(), PSP34Error> {
            let events = self.psp34.transfer_from(from, to, id, data)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.psp34.total_supply()
        }

        #[ink(message)]
        fn max_supply(&self) -> Balance {
            self.psp34.max_supply()
        }
    }

    impl PSP34Mintable for Aaw {
        #[ink(message)]
        fn mint(&mut self, account: AccountId) -> Result<(), PSP34Error> {
            if self.env().caller() != self.owner {
                return Err(PSP34Error::Custom(String::from(
                    "this message is only callable by the owner of the contract",
                )));
            }

            let events = self.psp34.mint(account)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn mint_with_attributes(
            &mut self,
            account: AccountId,
            attributes: Vec<(Vec<u8>, Vec<u8>)>,
        ) -> Result<(), PSP34Error> {
            if self.env().caller() != self.owner {
                return Err(PSP34Error::Custom(String::from(
                    "this message is only callable by the owner of the contract",
                )));
            }

            let events = self.psp34.mint_with_attributes(account, attributes)?;
            self.emit_events(events);
            Ok(())
        }
    }

    impl PSP34Metadata for Aaw {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
            self.psp34.get_attribute(id, key)
        }
    }

    impl PSP34Enumerable for Aaw {
        #[ink(message)]
        fn token_by_index(&self, index: u128) -> Option<Id> {
            self.psp34.token_by_index(index)
        }

        #[ink(message)]
        fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
            self.psp34.owners_token_by_index(owner, index)
        }
    }
}

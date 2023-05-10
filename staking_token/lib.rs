#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// A PSP-22 compliant staking token with metadata.
#[openbrush::contract]
pub mod dao_governance_token {
    use openbrush::{
        contracts::psp22::extensions::metadata::*,
        traits::{self, Storage},
    };
    use ink::prelude::string::{String, ToString};
    // use openbrush::{contracts::psp22::extensions::metadata::*, traits::Storage};
    // use ink::storage::traits::SpreadAllocate;
    // use ink_storage::traits::StorageLayout;
    // use ink_storage::traits::{PackedLayout, SpreadLayout};
    /// The main storage structure of the `DaoGovernanceToken` contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DaoGovernanceToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
      
    }

    /// Implementation of the PSP22 standard for this contract.
    impl PSP22 for DaoGovernanceToken {}

    /// Implementation of the PSP22Metadata extension for this contract.
    impl PSP22Metadata for DaoGovernanceToken {}

    /// Implementation of the `DaoGovernanceToken` contract.
    impl DaoGovernanceToken {
        /// Creates a new `DaoGovernanceToken` instance with the given `name`,`symbol`,
        /// `decimals` and `initial_supply`.
        #[ink(constructor)]
        pub fn new(
            name: Option<traits::String>,
            symbol: Option<traits::String>,
            decimals: u8,
            initial_supply: Balance,
        ) -> Self {
            let mut instance = Self::default();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimals;

            assert!(
                instance
                    ._mint_to(instance.env().caller(), initial_supply)
                    .is_ok(),
                "Failed to mint tokens to the contract creator"
            );

            instance
        }

        #[ink(message)]
        pub fn distribute_token( &mut self, to:AccountId, amount:u128 ) -> Result<(), PSP22Error>{
            let caller = self.env().caller();
            // if caller != self.dao_account_id {
            //     return Err(PSP22Error::Custom(
            //         "This function can be called by thd dao.".to_string().into(),
            //     ));

            // }
            match self._transfer_from_to(
                self.env().account_id(),
                to,
                amount,
                "transfer_data".as_bytes().to_vec(),
            ) {
                Ok(()) => Ok(()),
                Err(e) => {
                    // ink_env::debug_println!("     ########## transfer error : {:?}", e);
                    Err(PSP22Error::Custom("Transfering is failure.".to_string().into()))
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use openbrush::test_utils::*;

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        #[ink::test]
        fn constructor_sets_name_symbol_and_decimals() {
            let name = Some(traits::String::from("My Staking Token"));
            let symbol = Some(traits::String::from("MST"));
            let instance =
                DaoGovernanceToken::new(name.clone(), symbol.clone(), 18, INITIAL_SUPPLY);

            assert_eq!(instance.token_name(), name);
            assert_eq!(instance.token_symbol(), symbol);
            assert_eq!(instance.token_decimals(), 18);
        }

        #[ink::test]
        fn constructor_distributes_tokens_correctly() {
            let name = Some(traits::String::from("My Staking Token"));
            let symbol = Some(traits::String::from("MST"));
            let instance =
                DaoGovernanceToken::new(name.clone(), symbol.clone(), 18, INITIAL_SUPPLY);
            let owner = accounts().alice;

            assert_eq!(instance.total_supply(), INITIAL_SUPPLY);
            assert_eq!(instance.balance_of(owner), INITIAL_SUPPLY);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        use ink_e2e::build_message;
        use openbrush::contracts::psp22::{
            extensions::metadata::psp22metadata_external::PSP22Metadata, psp22_external::PSP22,
        };

        const INITIAL_SUPPLY: u128 = 1_000_000_000 * 10u128.pow(18);

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its constructor.
        #[ink_e2e::test]
        async fn instantiation_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = DaoGovernanceTokenRef::new(
                Some(traits::String::from("My Staking Token")),
                Some(traits::String::from("MST")),
                18,
                INITIAL_SUPPLY,
            );

            // Instantiate the contract
            let contract_account_id = client
                .instantiate("staking_token", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Check Token Name
            let token_name = build_message::<DaoGovernanceTokenRef>(contract_account_id.clone())
                .call(|token| token.token_name());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_name, 0, None)
                    .await
                    .return_value(),
                Some(traits::String::from("My Staking Token"))
            );

            // Check Token Symbol
            let token_symbol =
                build_message::<DaoGovernanceTokenRef>(contract_account_id.clone())
                    .call(|token| token.token_symbol());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_symbol, 0, None)
                    .await
                    .return_value(),
                Some(traits::String::from("MST"))
            );

            // Check Token Decimals
            let token_decimals =
                build_message::<DaoGovernanceTokenRef>(contract_account_id.clone())
                    .call(|token| token.token_decimals());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &token_decimals, 0, None)
                    .await
                    .return_value(),
                18
            );

            // Check Total Supply
            let total_supply =
                build_message::<DaoGovernanceTokenRef>(contract_account_id.clone())
                    .call(|token| token.total_supply());
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::alice(), &total_supply, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY
            );

            // Check Balance of Contract Owner (Alice)
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let alice_balance =
                build_message::<DaoGovernanceTokenRef>(contract_account_id.clone())
                    .call(|token| token.balance_of(alice_account));
            assert_eq!(
                client
                    .call_dry_run(&ink_e2e::bob(), &alice_balance, 0, None)
                    .await
                    .return_value(),
                INITIAL_SUPPLY
            );

            Ok(())
        }
    }
}

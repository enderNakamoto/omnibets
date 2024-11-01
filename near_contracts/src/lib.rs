// Find all our documentation at https://docs.near.org
use hex::decode;
use near_sdk::{env, ext_contract, near, require, Gas, NearToken, Promise, AccountId};
use serde::Serialize;

const PUBLIC_RLP_ENCODED_METHOD_NAMES: [&'static str; 1] = ["6a627842000000000000000000000000"];
const COST: NearToken = NearToken::from_near(1);
const MPC_CONTRACT_ACCOUNT_ID: &str = "v1.signer-prod.testnet";
const GAS: Gas = Gas::from_tgas(250);
const ATTACHED_DEPOSIT: NearToken = NearToken::from_yoctonear(50000000000000000000000);
const DEFAULT_MATURITY: u256 = 1_730_394_000_000_000_000; // October 31, 2024 12:00:00 PM GMT-05:00


#[derive(Serialize)]
pub struct SignRequest {
    pub payload: [u8; 32],
    pub path: String,
    pub key_version: u32,
}
// interface for cross contract call to mpc contract
#[ext_contract(mpc)]
trait MPC {
    fn sign(&self, request: SignRequest) -> Promise;
}

// automatically init the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            temperature: 0,
            cutoff_temperature: 3000, // Default cutoff at 30.00°C
            weatherman: "weatherman.near".parse().unwrap(),
            maturity: DEFAULT_MATURITY,
        }
    }
}

#[near(contract_state)]
pub struct Contract {
    // Temperature stored as u64, multiplied by 100 (e.g., 2550 represents 25.50°C)
    temperature: u64,
    // Cutoff temperature also stored as u64, multiplied by 100
    cutoff_temperature: u64,
    weatherman: AccountId,
    maturity: u256,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn new(weatherman: AccountId, cutoff_temperature: u64) -> Self {
        Self {
            temperature: 0,
            cutoff_temperature,
            weatherman,
            maturity: DEFAULT_MATURITY,
        }
    }

    /// Get the temperature in hundredths of a degree
    /// (e.g., 2550 represents 25.50°C)
    pub fn get_temperature(&self) -> u64 {
        self.temperature
    }

    /// Get the cutoff temperature in hundredths of a degree
    pub fn get_cutoff_temperature(&self) -> u64 {
        self.cutoff_temperature
    }

    pub fn get_maturity_date(&self) -> u256 {
        self.maturity
    }

    /// Set the temperature in hundredths of a degree
    /// (e.g., pass 2550 to represent 25.50°C)
    pub fn set_temperature(&mut self, temperature: u64) {
        assert_eq!(
            env::predecessor_account_id(),
            self.weatherman,
            "Only the weatherman can set the temperature"
        );
        self.temperature = temperature;
    }

    // Set the maturity date
    pub fn set_maturity_date(&mut self, maturity_date: u256) {
        let owner = env::predecessor_account_id() == env::current_account_id();
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the owner can set the maturity date"
        );
        self.maturity_date = maturity_date;
    }

    /// Set the cutoff temperature in hundredths of a degree
    pub fn set_cutoff_temperature(&mut self, cutoff_temperature: u64) {
        let owner = env::predecessor_account_id() == env::current_account_id();
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the owner can set the cutoff temperature"
        );
        self.cutoff_temperature = cutoff_temperature;
    }

    /// Check if current temperature is above cutoff
    /// Returns true if temperature is above cutoff, false otherwise
    pub fn is_above_cutoff(&self) -> bool {
        self.temperature > self.cutoff_temperature
    }

    pub fn get_weatherman(&self) -> AccountId {
        self.weatherman.clone()
    }

    pub fn is_matured(&self) -> bool {
        env::block_timestamp() >= self.maturity
    }


    // proxy to call MPC_CONTRACT_ACCOUNT_ID method sign if COST is deposited
    pub fn liquidate(&mut self, rlp_payload: String, path: String, key_version: u32) -> Promise {
        let owner = env::predecessor_account_id() == env::current_account_id();

        // check if rlp encoded eth transaction is calling a public method name
        let mut public = false;
        for n in PUBLIC_RLP_ENCODED_METHOD_NAMES {
            if rlp_payload.find(n).is_some() {
                public = true
            }
        }

        // only the Near contract owner can call sign of arbitrary payloads for chain signature accounts based on env::current_account_id()
        if !public {
            require!(
                owner,
                "only contract owner can sign arbitrary EVM transactions"
            );
        }

        // hash rlp encoded payload
        let payload: [u8; 32] = env::keccak256_array(&decode(rlp_payload).unwrap())
            .try_into()
            .unwrap();

        //check if it is not matured
        require!(!self.is_matured(), "Contract is matured");
        // chek the cut off temperature
        require!(self.is_above_cutoff(), "Temperature is below the cutoff");
        

        // call mpc sign and return promise
        mpc::ext(MPC_CONTRACT_ACCOUNT_ID.parse().unwrap())
            .with_static_gas(GAS)
            .with_attached_deposit(ATTACHED_DEPOSIT)
            .sign(SignRequest {
                payload,
                path,
                key_version,
            })
    }

    // proxy to call MPC_CONTRACT_ACCOUNT_ID method sign if COST is deposited
    pub fn mature(&mut self, rlp_payload: String, path: String, key_version: u32) -> Promise {
        let owner = env::predecessor_account_id() == env::current_account_id();

        // check if rlp encoded eth transaction is calling a public method name
        let mut public = false;
        for n in PUBLIC_RLP_ENCODED_METHOD_NAMES {
            if rlp_payload.find(n).is_some() {
                public = true
            }
        }

        // only the Near contract owner can call sign of arbitrary payloads for chain signature accounts based on env::current_account_id()
        if !public {
            require!(
                owner,
                "only contract owner can sign arbitrary EVM transactions"
            );
        }

        // hash rlp encoded payload
        let payload: [u8; 32] = env::keccak256_array(&decode(rlp_payload).unwrap())
            .try_into()
            .unwrap();

        // check if the market is already matured
        let deposit = env::attached_deposit();
        if is_matured {
            println!("Contract is matured");
        }

        // call mpc sign and return promise
        mpc::ext(MPC_CONTRACT_ACCOUNT_ID.parse().unwrap())
            .with_static_gas(GAS)
            .with_attached_deposit(ATTACHED_DEPOSIT)
            .sign(SignRequest {
                payload,
                path,
                key_version,
            })
    }

    // proxy to call MPC_CONTRACT_ACCOUNT_ID method sign if COST is deposited
    // #[payable]
    // pub fn create_maket(&mut self, rlp_payload: String, path: String, key_version: u32) -> Promise {
    //     let owner = env::predecessor_account_id() == env::current_account_id();

    //     // check if rlp encoded eth transaction is calling a public method name
    //     let mut public = false;
    //     for n in PUBLIC_RLP_ENCODED_METHOD_NAMES {
    //         if rlp_payload.find(n).is_some() {
    //             public = true
    //         }
    //     }

    //     // only the Near contract owner can call sign of arbitrary payloads for chain signature accounts based on env::current_account_id()
    //     if !public {
    //         require!(
    //             owner,
    //             "only contract owner can sign arbitrary EVM transactions"
    //         );
    //     }

    //     // hash rlp encoded payload
    //     let payload: [u8; 32] = env::keccak256_array(&decode(rlp_payload).unwrap())
    //         .try_into()
    //         .unwrap();

    //     // check deposit requirement, contract owner doesn't pay
    //     let deposit = env::attached_deposit();
    //     if !owner {
    //         require!(deposit >= COST, "need moolah to market make");
    //     }

    //     // call mpc sign and return promise
    //     mpc::ext(MPC_CONTRACT_ACCOUNT_ID.parse().unwrap())
    //         .with_static_gas(GAS)
    //         .with_attached_deposit(ATTACHED_DEPOSIT)
    //         .sign(SignRequest {
    //             payload,
    //             path,
    //             key_version,
    //         })
    // }

}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_get_default_temperature() {
        let contract = Contract::default();
        assert_eq!(contract.get_temperature(), 0);
        assert_eq!(contract.get_cutoff_temperature(), 3000); // 30.00°C
    }

    #[test]
    fn test_set_then_get_temperature() {
        let weatherman: AccountId = "weatherman.near".parse().unwrap();
        let context = get_context(weatherman.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(weatherman, 3000); // Cutoff at 30.00°C
        
        // Set temperature to 25.50°C (2550)
        contract.set_temperature(2550);
        assert_eq!(contract.get_temperature(), 2550);
    }

    #[test]
    #[should_panic(expected = "Only the weatherman can set the temperature")]
    fn test_set_temperature_unauthorized() {
        let weatherman: AccountId = "weatherman.near".parse().unwrap();
        let unauthorized: AccountId = "unauthorized.near".parse().unwrap();
        let context = get_context(unauthorized);
        testing_env!(context.build());

        let mut contract = Contract::new(weatherman, 3000);
        
        // Should panic
        contract.set_temperature(2550);
    }

    #[test]
    fn test_temperature_cutoff_comparison() {
        let weatherman: AccountId = "weatherman.near".parse().unwrap();
        let context = get_context(weatherman.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(weatherman, 2500); // Cutoff at 25.00°C
        
        // Set temperature to 26.50°C (2650) - above cutoff
        contract.set_temperature(2650);
        assert!(contract.is_above_cutoff());

        // Set temperature to 24.50°C (2450) - below cutoff
        contract.set_temperature(2450);
        assert!(!contract.is_above_cutoff());

        // Change cutoff to 24.00°C (2400)
        contract.set_cutoff_temperature(2400);
        assert!(contract.is_above_cutoff()); // 24.50°C now above cutoff
    }

    #[test]
    #[should_panic(expected = "Only the weatherman can set the cutoff temperature")]
    fn test_set_cutoff_unauthorized() {
        let weatherman: AccountId = "weatherman.near".parse().unwrap();
        let unauthorized: AccountId = "unauthorized.near".parse().unwrap();
        let context = get_context(unauthorized);
        testing_env!(context.build());

        let mut contract = Contract::new(weatherman, 2500);
        contract.set_cutoff_temperature(3000); // Should panic
    }
}
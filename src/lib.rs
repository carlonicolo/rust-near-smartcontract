use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

//const PASSWORD_NUMBER: u8 = 1; //no needed with [init] inside impl Contract

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    password_solution: String

}

#[near_bindgen]
impl Contract {
    // initialize the contract
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            password_solution: solution,
        }
    }
    // ADD CONTRACT METHODS HERE
    // &self is a ref to the current object. self vs &self
    // with the init we don't need anymore this method
    /* 
    pub fn get_password_number(&self) -> u8 {
        PASSWORD_NUMBER
    }
    */

    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }

    /* is not safe permit to set so with [init] don't need anymore this method
    pub fn set_solution(&mut self, solution: String) {
        self.password_solution = solution;
    }
     */

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        if hashed_input_hex == self.password_solution {
            env::log_str("You may enter!!! This is the right password");
            true
        } else {
            env::log_str("You shall not pass. Please try again.");
            false
        }

        /* 
        if solution == self.password_solution {
            env::log_str("You may enter!!! This is the right password")
        } else {
            env::log_str("You shall not pass. Please try again.")
        }
        */
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // debugging and iteration of a unit test
    #[test]
    fn debug_get_hash() {

        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "pippo";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    // write a function that will guess the solution
    // get an account ID
    // setup a testing context and test environment
    // setup the contract for the test - call a new method

    fn check_guess_solution() {
        let carlonicolo = AccountId::new_unchecked("carlonicolo.testnet".to_string());
        let context = get_context(carlonicolo);
        testing_env!(context.build());

        let mut contract = Contract::new("a2242ead55c94c3deb7cf2340bfef9d5bcaca22dfe66e646745ee4371c633fc8".to_string(),);
        
        let mut guess_result = contract.guess_solution("wrong answer".to_string());
        assert!(!guess_result, "Expectation: this is incorrect");
        assert_eq!(get_logs(), ["You shall not pass. Please try again."], "Expected a failure in logs");

        guess_result = contract.guess_solution("pippo".to_string());
        assert!(guess_result, "Expectation: this is the correct answer");
        assert_eq!(get_logs(), ["You shall not pass. Please try again.", "You may enter!!! This is the right password"]);


    }

}

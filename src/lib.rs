use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

const PASSWORD_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    password_solution: String

}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    // &self is a ref to the current object. self vs &self

    pub fn get_password_number(&self) -> u8 {
        PASSWORD_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.password_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) {
        if solution == self.password_solution {
            env::log_str("You may enter!!! This is the right password")
        } else {
            env::log_str("You shall not pass. Please try again.")
        }
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

    // TESTS HERE
}

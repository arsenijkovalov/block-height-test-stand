use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {}

#[near_bindgen]
impl Counter {
    pub fn current_block(&self) -> u64 {
        return env::block_index();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    #[test]
    fn unit_test() {
        let mut context = VMContextBuilder::new().build();
        testing_env!(context.clone());
        let contract = Counter {};
        assert_eq!(0, contract.current_block());
        context.block_index = 1;
        testing_env!(context);
        assert_eq!(1, contract.current_block());
    }
}

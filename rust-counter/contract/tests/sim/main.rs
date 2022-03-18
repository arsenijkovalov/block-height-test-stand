pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk::serde_json::json;
use near_sdk_sim::{
    deploy, init_simulator, lazy_static_include, view, ContractAccount, UserAccount,
};
use rust_counter_tutorial::CounterContract;

lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../out/main.wasm",
}

fn init() -> (UserAccount, ContractAccount<CounterContract>) {
    let root = init_simulator(None);
    let counter: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: "counter".to_string(),
        bytes: &COUNTER_BYTES,
        signer_account: root
    );
    (root, counter)
}

#[test]
fn simulation_test() {
    let (root, counter) = init();
    let block_index: u64 = view!(counter.current_block()).unwrap_json();
    let block_height: u64 = root
        .view(
            counter.account_id(),
            "current_block",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    assert_eq!(&block_index, &block_height);
}

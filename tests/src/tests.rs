use crate::Loader;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use ckb_testtool::context::Context;

// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for more examples

// generated unit test for contract order-contract
#[test]
fn test_order_contract() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("order-contract");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(vec![42]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

// generated unit test for contract bondings-curve-contract
#[test]
fn test_bondings_curve_contract() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("bondings-curve-contract");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(vec![42]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

// generated unit test for contract unique_liquidity_manager
#[test]
fn test_unique_liquidity_manager() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("unique_liquidity_manager");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(vec![42]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, 10_000_000)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

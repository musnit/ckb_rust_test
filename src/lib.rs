#[test]
fn it_works() {
    // load contract code
    let mut code = Vec::new();
    File::open("contract/target/riscv64imac-unknown-none-elf/debug/contract").unwrap().read_to_end(&mut code).expect("read code");
    let code = Bytes::from(code);

    // build contract context
    let mut context = Context::default();
    context.deploy_contract(code.clone());
    let tx = TxBuilder::default().lock_bin(code).inject_and_build(&mut context).expect("build tx");

    // do the verification
    let max_cycles = 50_000u64;
    let verify_result = context.verify_tx(&tx, max_cycles);
    verify_result.expect("pass test");
}

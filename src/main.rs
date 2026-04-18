mod vm;

use vm::evm::EVM;

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("Provide bytecode");

    let bytecode = hex_to_bytes(&input);

    let mut evm = EVM::new(bytecode);

    evm.run();

    println!("Final Stack: {:?}", evm.stack.peek());
}
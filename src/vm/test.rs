#[cfg(test)]
mod tests {
    use crate::vm::evm::EVM;

    fn hex_to_bytes(hex: &str) -> Vec<u8> {
        let hex = hex.trim_start_matches("0x");
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn test_add() {
        let bytecode = hex_to_bytes("0x6001600201");
        let mut evm = EVM::new(bytecode);
        evm.run();

        assert_eq!(evm.stack.peek().unwrap().as_u32(), 3);
    }
}
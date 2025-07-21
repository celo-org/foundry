use alloy_primitives::{Address, U256};
use revm::context::{ContextTr, JournalTr};

/// Celo transfer precompile implementation
/// Address: 0xfd (253)
/// Input: from (32 bytes) || to (32 bytes) || value (32 bytes) = 96 bytes total
/// Gas cost: 9000
pub fn celo_transfer_precompile<CTX: ContextTr>(
    mut context: CTX,
    inputs: &revm::interpreter::InputsImpl,
    gas_limit: u64,
) -> Result<Option<revm::interpreter::InterpreterResult>, String> {
    use revm::interpreter::{Gas, InstructionResult, InterpreterResult};

    // Gas cost for Celo transfer precompile
    const CELO_TRANSFER_GAS_COST: u64 = 9000;

    // Check minimum gas requirement
    if gas_limit < CELO_TRANSFER_GAS_COST {
        return Err("Insufficient gas for Celo transfer precompile".to_string());
    }

    let mut result = InterpreterResult {
        result: InstructionResult::Return,
        gas: Gas::new(gas_limit),
        output: Default::default(),
    };

    // Record gas cost
    if !result.gas.record_cost(CELO_TRANSFER_GAS_COST) {
        return Err("Out of gas in Celo transfer precompile".to_string());
    }

    // Validate input length (must be exactly 96 bytes: 32 + 32 + 32)
    let input_bytes = inputs.input.bytes(&mut context);
    if input_bytes.len() != 96 {
        return Err(format!(
            "Invalid input length for Celo transfer precompile: expected 96 bytes, got {}",
            input_bytes.len()
        ));
    }

    // Parse input: from (bytes 12-32), to (bytes 44-64), value (bytes 64-96)
    let from_bytes = &input_bytes[12..32];
    let to_bytes = &input_bytes[44..64];
    let value_bytes = &input_bytes[64..96];

    let from_address = Address::from_slice(from_bytes);
    let to_address = Address::from_slice(to_bytes);
    let value = U256::from_be_slice(value_bytes);

    eprintln!("\t[Celo Transfer] from: {from_address:?}, to: {to_address:?}, value: {value}");

    // Perform the transfer using JournalTr.transfer
    if let Err(e) = context.journal().transfer(from_address, to_address, value) {
        eprintln!("[Celo Transfer] Transfer failed: {e:?}");
        result.result = InstructionResult::PrecompileError;
        return Ok(Some(result));
    }

    eprintln!("\t[Celo Transfer] Transfer successful");

    // No output data for successful transfer
    result.output = alloy_primitives::Bytes::new();
    result.result = InstructionResult::Return;

    Ok(Some(result))
}

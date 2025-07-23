use super::celo_precompile::celo_transfer_precompile;
use alloy_evm::precompiles::PrecompilesMap;
use alloy_primitives::{Address, address};
use revm::{
    context::{Cfg, ContextTr},
    handler::PrecompileProvider,
};
use std::fmt::Debug;

const CELO_TRANSFER_ADDRESS: Address = address!("0x00000000000000000000000000000000000000fd");

/// Hybrid precompile provider that combines standard precompiles with stateful precompiles
#[derive(Debug)]
pub struct HybridPrecompileProvider {
    /// Standard precompiles map
    pub standard_precompiles: PrecompilesMap,
    // This should be mapping, but we can't store generic function pointers in a HashMap and
    // ContextTr is not dyn-compatible.
    // stateful_precompiles: HashMap<
    //     Address,
    //     fn(
    //         &mut CTX,
    //         &revm::interpreter::InputsImpl,
    //         u64,
    //     ) -> Result<Option<revm::interpreter::InterpreterResult>, String>,
    // >,
    /// Stateful precompile addresses
    stateful_precompile_addresses: Vec<Address>,
}

impl HybridPrecompileProvider {
    /// Create a new hybrid provider with standard precompiles and stateful precompiles
    pub fn new(standard_precompiles: PrecompilesMap) -> Self {
        Self { standard_precompiles, stateful_precompile_addresses: vec![CELO_TRANSFER_ADDRESS] }
    }

    /// Create a new hybrid provider with only standard precompiles
    pub fn standard_only(standard_precompiles: PrecompilesMap) -> Self {
        Self { standard_precompiles, stateful_precompile_addresses: Vec::new() }
    }

    /// Get all addresses (both standard and stateful precompiles)
    pub fn addresses(&self) -> Box<impl Iterator<Item = Address>> {
        let standard_addresses = self.standard_precompiles.addresses().copied();
        Box::new(self.stateful_precompile_addresses.iter().copied().chain(standard_addresses))
    }
}

impl<CTX> PrecompileProvider<CTX> for HybridPrecompileProvider
where
    CTX: ContextTr,
    PrecompilesMap: PrecompileProvider<CTX, Output = revm::interpreter::InterpreterResult>,
{
    type Output = revm::interpreter::InterpreterResult;

    fn set_spec(&mut self, spec: <<CTX as ContextTr>::Cfg as Cfg>::Spec) -> bool {
        <PrecompilesMap as PrecompileProvider<CTX>>::set_spec(&mut self.standard_precompiles, spec)
    }

    fn run(
        &mut self,
        context: &mut CTX,
        address: &Address,
        inputs: &revm::interpreter::InputsImpl,
        is_static: bool,
        gas_limit: u64,
    ) -> Result<Option<Self::Output>, String> {
        if self.stateful_precompile_addresses.contains(address) {
            // Check if this is the Celo transfer precompile
            if address == &CELO_TRANSFER_ADDRESS {
                return celo_transfer_precompile(context, inputs, gas_limit);
            }
        }

        // Fall back to standard precompiles
        self.standard_precompiles.run(context, address, inputs, is_static, gas_limit)
    }

    fn warm_addresses(&self) -> Box<impl Iterator<Item = Address>> {
        let standard_addresses =
            <PrecompilesMap as PrecompileProvider<CTX>>::warm_addresses(&self.standard_precompiles);
        Box::new(self.stateful_precompile_addresses.iter().copied().chain(standard_addresses))
    }

    fn contains(&self, address: &Address) -> bool {
        // Check stateful precompiles first
        if self.stateful_precompile_addresses.contains(address) {
            return true;
        }

        // Check standard precompiles
        <PrecompilesMap as PrecompileProvider<CTX>>::contains(&self.standard_precompiles, address)
    }
}

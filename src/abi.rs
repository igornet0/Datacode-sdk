//! Re-export of datacode_abi for plugin authors.

pub use datacode_abi::{
    abi_compatible, DatacodeError, DatacodeModule, DatacodeModuleFn, NativeAbiFn,
    NativeHandle, VmContext, AbiValue, AbiVersion, DATACODE_ABI_VERSION, DATACODE_MODULE_SYMBOL,
};

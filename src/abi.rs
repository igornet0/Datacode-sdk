//! Re-export of datacode_abi for plugin authors.

pub use datacode_abi::{
    abi_compatible, AbiClassDescriptor, AbiExport, AbiExportTable, AbiGlobalDescriptor,
    AbiModuleDescriptor, DatacodeError, DatacodeModule, DatacodeModuleEntryFn, DatacodeModuleFn,
    DatacodeModuleLegacy, NativeAbiFn, NativeHandle, VmContext, AbiValue, AbiVersion,
    DATACODE_ABI_VERSION, DATACODE_MODULE_ENTRY_SYMBOL, DATACODE_MODULE_SYMBOL,
};

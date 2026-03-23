//! Macros for defining modules and registering functions.

/// Registers a native function. `$fn` must be a function with signature
/// `fn(&[AbiValue]) -> AbiValue`. A trampoline is generated and registered.
///
/// Example: `dc_fn!(ctx, "hello", my_hello_fn);`
#[macro_export]
macro_rules! dc_fn {
    ($ctx:expr, $name:expr, $fn:path) => {{
        $crate::paste::paste! {
            extern "C" fn [< __dc_trampoline_ $fn >](
                _ctx: *mut $crate::abi::VmContext,
                args_ptr: *const $crate::abi::AbiValue,
                argc: usize,
            ) -> $crate::abi::AbiValue {
                let args: &[$crate::abi::AbiValue] = if args_ptr.is_null() || argc == 0 {
                    &[]
                } else {
                    unsafe { std::slice::from_raw_parts(args_ptr, argc) }
                };
                $fn(args)
            }
            $ctx.register_fn($name, [< __dc_trampoline_ $fn >]);
        }
    }};
}

/// Defines `datacode_module()` with **callback registration** (ABI 1.1+ layout).
/// The module uses `export_table = null` and `register = Some(...)`.
/// Prefer [`define_module_descriptor!`] or [`define_module_entry!`] when exports are known at compile time.
///
/// `$minor`: use `0` to mean ABI 1.1 layout with effective minor 1 (same as `1`).
#[macro_export]
macro_rules! define_module {
    ($name:literal, $major:literal, $minor:literal, $register_fn:path) => {
        use std::ffi::CString;
        use $crate::abi::{AbiVersion, DatacodeModule};

        static __DC_MODULE_NAME: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
        static __DC_MODULE_DESCRIPTOR: std::sync::OnceLock<*const DatacodeModule> = std::sync::OnceLock::new();

        #[no_mangle]
        pub extern "C" fn datacode_module() -> *const DatacodeModule {
            let name = __DC_MODULE_NAME.get_or_init(|| CString::new($name).expect("module name contains null"));
            *__DC_MODULE_DESCRIPTOR.get_or_init(|| {
                Box::into_raw(Box::new(DatacodeModule {
                    abi_version: AbiVersion {
                        major: $major,
                        minor: if $minor == 0 { 1 } else { $minor },
                    },
                    name: name.as_ptr(),
                    export_table: std::ptr::null(),
                    register: Some($register_fn),
                }))
            })
        }
    };
}

/// Static export table only: VM reads [`AbiExportTable`] inside [`DatacodeModule`] and does **not** call `register`.
///
/// Example:
/// `define_module_descriptor!("hello_module", 1, 1; "hello" => hello_fn);`
#[macro_export]
macro_rules! define_module_descriptor {
    ($name:literal, $major:literal, $minor:literal; $($export_name:literal => $fn:path),* $(,)?) => {
        use std::ffi::CString;
        use std::os::raw::c_char;
        use $crate::abi::{AbiExport, AbiExportTable, AbiVersion, DatacodeModule};

        static __DC_MODULE_NAME: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
        static __DC_MODULE_DESCRIPTOR: std::sync::OnceLock<*const DatacodeModule> = std::sync::OnceLock::new();
        static __DC_ABI_TABLE: std::sync::OnceLock<*const AbiExportTable> = std::sync::OnceLock::new();

        $crate::paste::paste! {
            $(
                extern "C" fn [< __dc_trampoline_desc_ $fn >](
                    _ctx: *mut $crate::abi::VmContext,
                    args_ptr: *const $crate::abi::AbiValue,
                    argc: usize,
                ) -> $crate::abi::AbiValue {
                    let args: &[$crate::abi::AbiValue] = if args_ptr.is_null() || argc == 0 {
                        &[]
                    } else {
                        unsafe { std::slice::from_raw_parts(args_ptr, argc) }
                    };
                    $fn(args)
                }
            )*

            static __DC_EXPORTS: &[AbiExport] = &[
                $(
                    AbiExport {
                        name: concat!($export_name, "\0").as_ptr() as *const c_char,
                        func: [< __dc_trampoline_desc_ $fn >],
                        arity: 0,
                        flags: 0,
                    },
                )*
            ];
        }

        #[no_mangle]
        pub extern "C" fn datacode_module() -> *const DatacodeModule {
            let name = __DC_MODULE_NAME.get_or_init(|| CString::new($name).expect("module name contains null"));
            let table = *__DC_ABI_TABLE.get_or_init(|| {
                Box::into_raw(Box::new(AbiExportTable {
                    exports: __DC_EXPORTS.as_ptr(),
                    exports_len: __DC_EXPORTS.len(),
                }))
            });
            *__DC_MODULE_DESCRIPTOR.get_or_init(|| {
                Box::into_raw(Box::new(DatacodeModule {
                    abi_version: AbiVersion {
                        major: $major,
                        minor: if $minor == 0 { 1 } else { $minor },
                    },
                    name: name.as_ptr(),
                    export_table: table,
                    register: None,
                }))
            })
        }
    };
}

/// Production entry: **`datacode_module_entry()`** → `*const AbiModuleDescriptor` (no [`DatacodeModule`] wrapper).
/// Build with `[lib] crate-type = ["cdylib"]` for dynamic loading.
///
/// Example: `define_module_entry!("hello_module", 1, 2; "hello" => hello_fn);`
#[macro_export]
macro_rules! define_module_entry {
    ($name:literal, $major:literal, $minor:literal; $($export_name:literal => $fn:path),* $(,)?) => {
        use std::ffi::CString;
        use std::os::raw::c_char;
        use $crate::abi::{AbiExport, AbiModuleDescriptor, AbiVersion};

        static __DC_MODULE_NAME: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
        // Store a pointer: `OnceLock<AbiModuleDescriptor>` is not `Sync` (raw pointers in the struct).
        static __DC_ROOT_DESC: std::sync::OnceLock<*const AbiModuleDescriptor> = std::sync::OnceLock::new();

        $crate::paste::paste! {
            $(
                extern "C" fn [< __dc_trampoline_entry_ $fn >](
                    _ctx: *mut $crate::abi::VmContext,
                    args_ptr: *const $crate::abi::AbiValue,
                    argc: usize,
                ) -> $crate::abi::AbiValue {
                    let args: &[$crate::abi::AbiValue] = if args_ptr.is_null() || argc == 0 {
                        &[]
                    } else {
                        unsafe { std::slice::from_raw_parts(args_ptr, argc) }
                    };
                    $fn(args)
                }
            )*

            static __DC_EXPORTS: &[AbiExport] = &[
                $(
                    AbiExport {
                        name: concat!($export_name, "\0").as_ptr() as *const c_char,
                        func: [< __dc_trampoline_entry_ $fn >],
                        arity: 0,
                        flags: 0,
                    },
                )*
            ];
        }

        #[no_mangle]
        pub extern "C" fn datacode_module_entry() -> *const AbiModuleDescriptor {
            let name = __DC_MODULE_NAME.get_or_init(|| CString::new($name).expect("module name contains null"));
            *__DC_ROOT_DESC.get_or_init(|| {
                let desc = AbiModuleDescriptor {
                    abi_version: AbiVersion {
                        major: $major,
                        minor: if $minor == 0 { 2 } else { $minor },
                    },
                    name: name.as_ptr(),
                    functions: __DC_EXPORTS.as_ptr(),
                    functions_len: __DC_EXPORTS.len(),
                    classes: std::ptr::null(),
                    classes_len: 0,
                    globals: std::ptr::null(),
                    globals_len: 0,
                };
                Box::into_raw(Box::new(desc))
            })
        }
    };
}

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

/// Defines the `datacode_module` entry point and static module descriptor.
/// `$register_fn` must be an `extern "C" fn(*mut VmContext)` that registers your natives.
///
/// Example: `define_module!("hello_module", 1, 0, register);`
#[macro_export]
macro_rules! define_module {
    ($name:expr, $major:expr, $minor:expr, $register_fn:path) => {
        use std::ffi::CString;
        use $crate::abi::{AbiVersion, DatacodeModule};

        static __DC_MODULE_NAME: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
        static __DC_MODULE_DESCRIPTOR: std::sync::OnceLock<DatacodeModule> = std::sync::OnceLock::new();

        #[no_mangle]
        pub extern "C" fn datacode_module() -> *const DatacodeModule {
            __DC_MODULE_NAME.get_or_init(|| CString::new($name).expect("module name contains null"));
            __DC_MODULE_DESCRIPTOR.get_or_init(|| DatacodeModule {
                abi_version: AbiVersion {
                    major: $major,
                    minor: $minor,
                },
                name: __DC_MODULE_NAME.get().unwrap().as_ptr(),
                register: $register_fn,
            });
            __DC_MODULE_DESCRIPTOR.get().unwrap() as *const DatacodeModule
        }
    };
}

//! Example native module: exports `hello()` that prints and returns Null.

use datacode_sdk::{define_module, dc_fn, types::*, ModuleContext};
use datacode_sdk::abi::{AbiValue, VmContext};

fn hello_fn(_args: &[AbiValue]) -> AbiValue {
    println!("Hello from DataCode SDK!");
    abi_null()
}

extern "C" fn register(ctx: *mut VmContext) {
    let mut wrapper = ModuleContext::new(ctx);
    dc_fn!(wrapper, "hello", hello_fn);
}

define_module!("hello_module", 1, 0, register);

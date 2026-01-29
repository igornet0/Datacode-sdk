# DataCode SDK

SDK for writing **DataCode native modules and plugins** using the DataCode ABI. You can implement extensions in Rust (or C) without diving into the VM internals.

## Features

- **ABI types** — re-export of `datacode_abi`: `AbiValue`, `VmContext`, `DatacodeModule`, version checks.
- **Ergonomic Rust API** — `ModuleContext`, `register_fn`, and helpers in `types` (`get_int`, `abi_str`, etc.).
- **Macros** — `define_module!(name, major, minor, register)` and `dc_fn!(ctx, "name", fn)` to register functions with signature `fn(&[AbiValue]) -> AbiValue`.
- **C support** — `include/datacode.h` for writing plugins in C.
- **Examples** — `hello_module` (Rust cdylib) and .dc scripts (`hello_world.dc`, `math_module.dc`, `telegram_bot.dc`).
- **Tools** — `tools/build_abi.sh` to build a module and optionally copy the .so/.dylib.

## Quick start (Rust module)

1. Add to your crate:

   ```toml
   [lib]
   crate-type = ["cdylib"]
   [dependencies]
   datacode_sdk = { path = "..." }
   ```

2. Implement and export the module:

   ```rust
   use datacode_sdk::{define_module, dc_fn, types::*, ModuleContext};
   use datacode_sdk::abi::{AbiValue, VmContext};

   fn hello(_args: &[AbiValue]) -> AbiValue {
       println!("Hello from DataCode!");
       datacode_sdk::abi_null()
   }

   extern "C" fn register(ctx: *mut VmContext) {
       let mut ctx = ModuleContext::new(ctx);
       dc_fn!(ctx, "hello", hello);
   }

   define_module!("hello_module", 1, 0, register);
   ```

3. Build: `cargo build --release --lib`. Put `libhello_module.dylib` (or `.so`) where the VM looks (script dir or cwd).

4. In DataCode: `import hello_module` then `hello()`.

## Layout

- **src/** — library: `abi`, `types`, `context`, `macros`.
- **include/** — `datacode.h` for C plugins.
- **examples/** — `hello_module/` (Rust), `hello_world.dc`, `math_module.dc`, `telegram_bot.dc`.
- **tools/** — `build_abi.sh`, `gen_bindings.rs` (stub).
- **docs/** — `ABI.md`, `modules.md`, `examples.md`.

## Documentation

- [ABI.md](docs/ABI.md) — contract, types, versioning, entry point.
- [modules.md](docs/modules.md) — how to write a module in Rust or C and load it.
- [examples.md](docs/examples.md) — description of the included examples.

## ABI version

The VM and modules must use the same ABI **major** version; module **minor** must be ≤ VM minor. Current: **1.0**.

# DataCode SDK

SDK for writing **DataCode native modules and plugins** using the DataCode ABI. You can implement extensions in Rust (or C) without diving into the VM internals.

## Features

- **ABI types** — re-export of `datacode_abi`: `AbiValue`, `VmContext`, `DatacodeModule`, version checks.
- **Ergonomic Rust API** — `ModuleContext`, `register_fn`, and helpers in `types` (`get_int`, `abi_str`, etc.).
- **Macros** — `define_module!(name, major, minor, register)` and `dc_fn!(ctx, "name", fn)` to register functions with signature `fn(&[AbiValue]) -> AbiValue`.
- **C support** — `include/datacode.h` for writing plugins in C.
- **Examples** — `hello_module` (Rust cdylib) and .dc scripts (`hello_world.dc`, `math_module.dc`, `telegram_bot.dc`).
- **Tools** — `tools/build_abi.sh` to build a module and optionally copy the .so/.dylib.
- **Artifact root (`DATACODE_DIST`)** — `module_dist::DIST_RUSTC_ENV`, macros `dist_rel_path!` / `dist_root!`; set via `build.rs` so packaging scripts know where to place `dist/` data (no hardcoded paths in the SDK).

## Artifact directory (`DATACODE_DIST`)

The SDK does **not** embed a fixed `dist/` path. Your module’s `build.rs` sets the **rustc** env var (name in `datacode_sdk::module_dist::DIST_RUSTC_ENV`, usually `DATACODE_DIST`):

```toml
[build-dependencies]
datacode_sdk = { path = "..." }
```

```rust
fn main() {
    println!(
        "cargo:rustc-env={}=dist",
        datacode_sdk::module_dist::DIST_RUSTC_ENV
    );
    println!("cargo:rerun-if-changed=build.rs");
}
```

Then in library code (expanded in **your** crate, not in the SDK):

```rust
use datacode_sdk::dist_rel_path;

const DATA: &str = dist_rel_path!("/datasets/mnist/train-images.idx3-ubyte");
```

Use `dist_root!()` for the root segment only. Copy datasets and other files into `{manifest_dir}/{value}/...` during your Makefile or CI step.

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

- **src/** — library: `abi`, `types`, `context`, `macros`, `module_dist` (`DATACODE_DIST`, `dist_rel_path!`, `dist_root!`).
- **include/** — `datacode.h` for C plugins.
- **examples/** — `hello_module/` (Rust), `hello_world.dc`, `math_module.dc`, `telegram_bot.dc`.
- **tools/** — `build_abi.sh`, `gen_bindings.rs` (optional placeholder; not required for builds).
- **docs/** — index [docs/README.md](docs/README.md) (RU + links to EN); [docs/en/](docs/en/) (English); [docs/ru/](docs/ru/) (Russian).

## Documentation

- [docs/README.md](docs/README.md) — Russian index and links to English pages.
- [ENREADME.md](docs/ENREADME.md) — short English overview.
- English: [ABI](docs/en/ABI.md), [modules](docs/en/modules.md), [examples](docs/en/examples.md).
- Russian: [ABI](docs/ru/ABI.md), [modules](docs/ru/modules.md), [examples](docs/ru/examples.md).

## ABI version

The VM and modules must use the same ABI **major** version; module **minor** must be ≤ VM minor. The canonical value is `datacode_abi::DATACODE_ABI_VERSION` (currently **1.7**). See [docs/en/ABI.md](docs/en/ABI.md) or [docs/ru/ABI.md](docs/ru/ABI.md).

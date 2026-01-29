# Writing native modules

How to implement a DataCode native module in Rust or C and use it from .dc code.

## Rust (with datacode_sdk)

### 1. Create a library with cdylib

In your module's `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
datacode_sdk = { path = "..." }  # or from crates.io when published
```

### 2. Implement the entry point and register functions

```rust
use datacode_sdk::{define_module, dc_fn, types::*, ModuleContext};
use datacode_sdk::abi::{AbiValue, VmContext};

fn my_fn(args: &[AbiValue]) -> AbiValue {
    let x = get_int(args, 0).unwrap_or(0);
    abi_int(x + 1)
}

extern "C" fn register(ctx: *mut VmContext) {
    let mut wrapper = ModuleContext::new(ctx);
    dc_fn!(wrapper, "my_fn", my_fn);
}

define_module!("my_module", 1, 0, register);
```

- **define_module!(name, major, minor, register_fn)** — generates `datacode_module()` and the static descriptor.
- **dc_fn!(ctx, "name", fn_path)** — registers a function; the given function must have signature `fn(&[AbiValue]) -> AbiValue`.

### 3. Build

```bash
cargo build --release --lib
```

Output: `target/release/lib<name>.dylib` (macOS) or `target/release/lib<name>.so` (Linux).

Use the SDK script:

```bash
./tools/build_abi.sh path/to/your_module [output_dir]
```

### 4. Use from DataCode

Place the built library where the VM can find it (script directory or current directory). Then:

```dc
import my_module
println(my_fn(42))   # 43
```

## C

1. Include `include/datacode.h` and implement:
   - `const DatacodeModule* datacode_module(void);` — return a pointer to a static `DatacodeModule`.
   - In `DatacodeModule.register`, call `register_native(ctx, "name", your_c_function)` for each export.

2. Your native function must have signature:

   ```c
   DatacodeValue your_fn(DatacodeVmContext* ctx, const DatacodeValue* args, size_t argc);
   ```

3. Build a shared library that exports `datacode_module`.

4. Name the library `lib<module_name>.so` / `lib<module_name>.dylib` and place it as for Rust.

## Finding the library

The VM looks for the native module in:

1. The **base path** (usually the directory of the script being run).
2. The **current working directory**.

So for `import hello_module` it looks for `libhello_module.dylib` (or `.so`) in those locations.

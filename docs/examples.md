# SDK examples

## hello_module (Rust)

Location: `examples/hello_module/`.

A minimal native module that exports one function, `hello()`, which prints a message and returns null.

- **Build**: from `datacode_sdk/examples/hello_module` run `cargo build --release --lib`, or from `datacode_sdk/tools` run `./build_abi.sh ../examples/hello_module`.
- **Output**: `libhello_module.dylib` (or `.so`). Copy it to the directory where you run DataCode scripts, or run scripts from that directory.
- **Usage** (DataCode):

  ```dc
  import hello_module
  hello()
  ```

## hello_world.dc

Imports `hello_module` and calls `hello()`. Requires `libhello_module.dylib` in the same directory (or current dir) when running:

```bash
datacode hello_world.dc
```

## math_module.dc

Placeholder example for a hypothetical native "math" module that would export `add`, `mul`, etc. Uncomment and adapt when such a module exists.

## telegram_bot.dc

Placeholder for using a Telegram bot via a native module. The actual Telegram integration may live in `datacode_lib/gram_api` or a separate ABI module; this file shows the intended usage pattern.

## Manual bindings

If you want a .dc file that re-exports or wraps native functions (e.g. for documentation), you can write it by hand. A future `gen_bindings` tool could generate such files from a list of function names or from Rust annotations.

# DataCode ABI

Minimal, stable, C-compatible contract between the DataCode VM and native plugins (.so / .dylib / .dll).

## Version

ABI is versioned as **major.minor**. The VM checks compatibility when loading a module:

- **Same major**: required.
- **Module minor ≤ VM minor**: VM 1.2 accepts modules 1.0 and 1.2; module 1.3 is rejected.

Current VM ABI version: **1.0** (see `datacode_abi::DATACODE_ABI_VERSION`).

## Types

### AbiVersion

```c
typedef struct {
    uint16_t major;
    uint16_t minor;
} DatacodeAbiVersion;
```

### AbiValue (Value)

Single value type on the boundary. Pointers (strings, arrays, object handles) are valid only for the duration of the native call.

- **Int(i64)** — integer
- **Float(f64)** — float
- **Bool(bool)** — boolean
- **Str(*const c_char)** — UTF-8, null-terminated; not owned
- **Null**
- **Array(*mut AbiValue, usize)** — pointer and length; valid during call
- **Object(NativeHandle)** — opaque handle to VM object/dict

Layout must match Rust `#[repr(C)]` in `datacode_abi::value::Value`.

### VmContext

Opaque context passed to `register`. The module uses only these callbacks:

- **alloc(size: usize) -> *mut u8** — allocate via VM; null on failure
- **throw_error(code: DatacodeError, msg: *const c_char)** — report error to VM (try/catch)
- **register_native(ctx, name: *const c_char, func: NativeAbiFn)** — register one native function

### NativeAbiFn

```c
AbiValue (*NativeAbiFn)(VmContext* ctx, const AbiValue* args, size_t argc);
```

### DatacodeModule

Descriptor returned by the module entry point:

- **abi_version** — module ABI version (must be compatible with VM)
- **name** — module name (UTF-8, null-terminated)
- **register** — `extern "C" fn(*mut VmContext)`; VM calls it after ABI check

## Entry point

The VM looks for the symbol **`datacode_module`** in the loaded library.

Signature:

```c
const DatacodeModule* datacode_module(void);
```

Must return a non-null pointer to a valid `DatacodeModule` (usually static).

## Loading flow

1. VM resolves `import foo` and looks for `libfoo.so` / `libfoo.dylib` / `foo.dll` in base path and current directory.
2. VM loads the library and gets `datacode_module`.
3. VM calls `datacode_module()`, checks `abi_compatible(module->abi_version, VM_ABI_VERSION)`.
4. VM calls `module->register(&vm_context)`.
5. Module calls `register_native(ctx, "function_name", my_fn)` for each exported function.
6. VM builds the module object (name → native function index) and exposes it to DataCode.

## C header

See `include/datacode.h` in this SDK for C declarations. Layout must match the Rust types in `datacode_abi` (and in the main VM crate `data_code::abi`).

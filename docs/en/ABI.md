# DataCode ABI

Minimal, stable, C-compatible contract between the DataCode VM and native plugins (`.so` / `.dylib` / `.dll`).

**Source of truth:** Rust crate [`datacode_abi`](../../datacode_abi/) in this repo. The VM re-exports it as `data_code::abi`. Do not duplicate layouts.

## Version

ABI is versioned as **major.minor**. The VM checks compatibility when loading a module (`abi_compatible` in `datacode_abi::version`):

- **Same major** is required.
- **Module minor ≤ VM minor** (e.g. VM **1.7** accepts modules **1.0** … **1.7**; module **1.8** is rejected by VM **1.7**).

Current contract version: **`datacode_abi::DATACODE_ABI_VERSION`** (today **1.7**).

Minor bumps document additive changes (new `AbiValue` variants, descriptor fields, etc.). Bump **major** only for breaking FFI layout or semantics.

## Types

### AbiVersion

```c
typedef struct {
    uint16_t major;
    uint16_t minor;
} DatacodeAbiVersion;
```

### AbiValue (Value)

Single value type on the boundary. Pointers (strings, arrays, table cells, bytes) are valid only for the duration of the native call.

Rust `#[repr(C)]` enum in `datacode_abi::value::Value` (exported as `AbiValue`):

- **Int** / **Float** / **Bool** / **Str** / **Null** / **Array** / **Object** — baseline.
- **PluginOpaque { tag, id }** — ABI 1.3+ opaque plugin handles.
- **Table { headers, cells, rows, cols }** — ABI 1.4+ VM `Table` → native.
- **Bytes { ptr, len }** — ABI 1.6+ dense buffers without per-element arrays.

### VmContext

Opaque context passed to legacy `register` and to native callbacks. The module uses the callbacks exposed on `VmContext` (see `datacode_abi::vm_context`).

### NativeAbiFn

```c
AbiValue (*NativeAbiFn)(VmContext* ctx, const AbiValue* args, size_t argc);
```

## Entry points

1. **Preferred (ABI 1.2+):** export **`datacode_module_entry`** → returns `*const AbiModuleDescriptor` (symbol `DATACODE_MODULE_ENTRY_SYMBOL`). The VM reads `abi_version` and static export tables from the descriptor.

2. **Transitional:** **`datacode_module`** (`DATACODE_MODULE_SYMBOL`) returning `DatacodeModule` / legacy three-field layout — still supported for older modules.

See `datacode_abi::module` and SDK macros `define_module!`, `define_module_descriptor!`, `define_module_entry!`.

## Loading flow (summary)

1. VM resolves a native import and loads the dynamic library.
2. VM resolves `datacode_module_entry` or `datacode_module`, checks ABI compatibility.
3. Exports are taken from the descriptor and/or `register` callback (legacy paths).
4. VM exposes the module object to DataCode user code.

## C header (`include/datacode.h`)

The checked-in C header is a **minimal / legacy-oriented** snapshot (early ABI subset). It does **not** list every variant or the full `AbiModuleDescriptor` layout. For new native code, **prefer the Rust SDK** (`datacode_sdk`) or mirror the exact `#[repr(C)]` definitions in `datacode_abi` when writing C by hand.

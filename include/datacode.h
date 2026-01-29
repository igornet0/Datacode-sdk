/**
 * DataCode ABI — C header for native modules.
 * Layout must match the Rust repr(C) types in datacode_abi.
 *
 * Usage:
 * 1. Implement datacode_module() returning a pointer to a static DatacodeModule.
 * 2. In register(DatacodeVmContext*), call register_native for each exported function.
 * 3. Build as .so / .dylib / .dll; VM loads lib<name>.so and resolves "datacode_module".
 */

#ifndef DATACODE_ABI_H
#define DATACODE_ABI_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- ABI version --- */
typedef struct {
    uint16_t major;
    uint16_t minor;
} DatacodeAbiVersion;

/* --- Error codes (module → VM) --- */
typedef enum {
    DatacodeErrorOk = 0,
    DatacodeErrorTypeError = 1,
    DatacodeErrorRuntimeError = 2,
    DatacodeErrorPanic = 3
} DatacodeError;

/* --- Value (tagged union; layout matches Rust repr(C) enum) --- */
typedef struct DatacodeValue DatacodeValue;

typedef enum {
    DC_VAL_INT,
    DC_VAL_FLOAT,
    DC_VAL_BOOL,
    DC_VAL_STR,
    DC_VAL_NULL,
    DC_VAL_ARRAY,
    DC_VAL_OBJECT
} DatacodeValueTag;

struct DatacodeValue {
    uint8_t tag;
    uint8_t _pad[7];
    union {
        int64_t int_val;
        double float_val;
        bool bool_val;
        const char* str_val;
        struct {
            DatacodeValue* ptr;
            size_t len;
        } array_val;
        void* object_val;
    } u;
};

/* --- VM context (opaque; module uses only the callbacks) --- */
typedef struct DatacodeVmContext DatacodeVmContext;

typedef DatacodeValue (*DatacodeNativeFn)(DatacodeVmContext* ctx, const DatacodeValue* args, size_t argc);

struct DatacodeVmContext {
    void* (*alloc)(size_t size);
    void (*throw_error)(DatacodeError code, const char* msg);
    void (*register_native)(DatacodeVmContext* ctx, const char* name, DatacodeNativeFn func);
};

/* --- Module descriptor --- */
typedef struct {
    DatacodeAbiVersion abi_version;
    const char* name;
    void (*register_fn)(DatacodeVmContext* ctx);
} DatacodeModule;

/* Entry point symbol: export a function with this name. */
#define DATACODE_MODULE_SYMBOL "datacode_module"

/* Module must export: const DatacodeModule* datacode_module(void); */

#ifdef __cplusplus
}
#endif

#endif /* DATACODE_ABI_H */

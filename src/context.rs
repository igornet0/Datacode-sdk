//! Ergonomic wrapper over VmContext for registering native functions.

use std::ffi::CString;

use crate::abi::{NativeAbiFn, VmContext};

/// Context wrapper that keeps C string names alive and exposes `register_fn`.
pub struct ModuleContext {
    ctx_ptr: *mut VmContext,
    name_strings: Vec<CString>,
}

impl ModuleContext {
    /// Create a context wrapper around the raw VmContext pointer.
    /// Call this from your `register` callback.
    pub fn new(ctx: *mut VmContext) -> Self {
        Self {
            ctx_ptr: ctx,
            name_strings: Vec::new(),
        }
    }

    /// Register a native function. The trampoline must have ABI signature
    /// `(ctx, *const AbiValue, usize) -> AbiValue`; use the `dc_fn!` macro
    /// to wrap a `fn(&[AbiValue]) -> AbiValue` and get a trampoline.
    pub fn register_fn(&mut self, name: &str, trampoline: NativeAbiFn) {
        if self.ctx_ptr.is_null() {
            return;
        }
        let cname = match CString::new(name) {
            Ok(c) => c,
            Err(_) => return,
        };
        self.name_strings.push(cname);
        let ptr = self.name_strings.last().unwrap().as_ptr();
        let ctx = unsafe { &*self.ctx_ptr };
        (ctx.register_native)(self.ctx_ptr, ptr, trampoline);
    }
}

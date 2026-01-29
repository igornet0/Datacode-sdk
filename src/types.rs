//! Type aliases and safe helpers for working with AbiValue.

use std::ffi::CStr;

use crate::abi::AbiValue;

/// DataCode integer (ABI: i64).
pub type DCInt = i64;

/// DataCode float (ABI: f64).
pub type DCFloat = f64;

// --- Reading from args (valid only for the duration of the native call) ---

/// Get argument at index as i64, or None if missing/wrong type.
#[inline]
pub fn get_int(args: &[AbiValue], i: usize) -> Option<i64> {
    match args.get(i) {
        Some(AbiValue::Int(n)) => Some(*n),
        _ => None,
    }
}

/// Get argument at index as f64, or None if missing/wrong type.
#[inline]
pub fn get_float(args: &[AbiValue], i: usize) -> Option<f64> {
    match args.get(i) {
        Some(AbiValue::Int(n)) => Some(*n as f64),
        Some(AbiValue::Float(f)) => Some(*f),
        _ => None,
    }
}

/// Get argument at index as bool, or None if missing/wrong type.
#[inline]
pub fn get_bool(args: &[AbiValue], i: usize) -> Option<bool> {
    match args.get(i) {
        Some(AbiValue::Bool(b)) => Some(*b),
        _ => None,
    }
}

/// Get argument at index as &str. Pointer is valid only during the native call.
/// Returns None if missing, null, or invalid UTF-8.
#[inline]
pub fn get_str(args: &[AbiValue], i: usize) -> Option<&str> {
    match args.get(i) {
        Some(AbiValue::Str(p)) if !p.is_null() => unsafe { CStr::from_ptr(*p).to_str().ok() },
        _ => None,
    }
}

// --- Building return values ---

#[inline]
pub fn abi_int(i: i64) -> AbiValue {
    AbiValue::Int(i)
}

#[inline]
pub fn abi_float(f: f64) -> AbiValue {
    AbiValue::Float(f)
}

#[inline]
pub fn abi_bool(b: bool) -> AbiValue {
    AbiValue::Bool(b)
}

#[inline]
pub fn abi_null() -> AbiValue {
    AbiValue::Null
}

/// Returns a string value. Uses Rust allocation; VM copies the string and does not free this pointer.
/// Prefer returning short-lived strings only.
pub fn abi_str(s: &str) -> AbiValue {
    match std::ffi::CString::new(s) {
        Ok(cs) => AbiValue::Str(cs.into_raw()),
        Err(_) => AbiValue::Null,
    }
}

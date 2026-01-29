//! DataCode SDK for writing native modules and plugins via ABI.
//!
//! Re-exports ABI types, ergonomic context wrapper, value helpers, and macros.

pub mod abi;
pub mod types;
pub mod context;
pub mod macros;

pub use abi::*;
pub use types::*;
pub use context::ModuleContext;
// Macros use paste internally; re-export so macro expansions resolve.
pub use paste;

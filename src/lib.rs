//! DataCode SDK for writing native modules and plugins via ABI.
//!
//! Re-exports ABI types, ergonomic context wrapper, value helpers, and macros.
//!
//! Use `define_module_entry!` for the **production** path (`datacode_module_entry` → root
//! [`AbiModuleDescriptor`]), `define_module_descriptor!` for [`DatacodeModule`] + nested
//! [`AbiExportTable`], or `define_module!` for the register callback. Build plugins as **cdylib**.
//! See the `datacode_abi` crate docs for version and layout rules.

pub mod abi;
pub mod context;
pub mod macros;
/// Переменная rustc `DATACODE_DIST` и макросы путей артефактов (`dist_rel_path!`, `dist_root!`).
pub mod module_dist;
pub mod types;

pub use abi::*;
pub use context::ModuleContext;
pub use types::*;
// Macros use paste internally; re-export so macro expansions resolve.
pub use paste;

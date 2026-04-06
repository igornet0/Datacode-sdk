//! Optional tool: generate DataCode `.dc` binding stubs from manifests or Rust sources.
//!
//! **Status:** intentionally unimplemented. The project does not rely on this binary;
//! keep manual `.dc` wrappers or extend this crate when you want codegen.
//!
//! Related: `tools/build_abi.sh`, `datacode_sdk/docs/examples.md`, and the ABI types in
//! `datacode_sdk/datacode_abi`.

fn main() {
    eprintln!(
        "gen_bindings: not implemented. Build modules with datacode_sdk + datacode_abi; see datacode_sdk/docs/ABI.md."
    );
}

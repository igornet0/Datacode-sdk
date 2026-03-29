//! Корень каталога артефактов при сборке нативного модуля (обычно `dist/`).
//!
//! В `build.rs` вашего пакета задайте значение **перед** компиляцией кода, который использует
//! макросы `dist_rel_path!` или `dist_root!`:
//!
//! ```ignore
//! println!("cargo:rustc-env={}=dist", datacode_sdk::module_dist::DIST_RUSTC_ENV);
//! ```
//!
//! или явно:
//!
//! ```ignore
//! println!("cargo:rustc-env=DATACODE_DIST=dist");
//! println!("cargo:rerun-if-changed=build.rs");
//! ```
//!
//! Тогда при сборке скрипты и Makefile могут класть файлы в `{CARGO_MANIFEST_DIR}/{значение}/...`
//! (например `dist/datasets/mnist/`), а пути в коде собираются через `dist_rel_path!`.

/// Имя переменной окружения **rustc** (`env!`), не OS `std::env`.
pub const DIST_RUSTC_ENV: &str = "DATACODE_DIST";

#[macro_export]
macro_rules! dist_rel_path {
    ($suffix:literal) => {
        concat!(env!("DATACODE_DIST"), $suffix)
    };
}

#[macro_export]
macro_rules! dist_root {
    () => {
        env!("DATACODE_DIST")
    };
}

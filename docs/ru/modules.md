# Нативные модули

Как реализовать нативный модуль DataCode на Rust или C и вызывать его из кода `.dc`.

## Жизненный цикл вызова (VM → ABI)

Полный путь от байткода через `AbiValue` к вашей нативной функции и обратно описан в документации репозитория: [Жизненный цикл вызова](https://github.com/igornet0/DataCode/blob/main/docs/ru/200-разработчикам/native-call-lifecycle.md).

## Rust (с datacode_sdk)

### 1. Библиотека с типом cdylib

В `Cargo.toml` вашего модуля:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
datacode_sdk = { path = "..." }  # или с crates.io после публикации
```

### 2. Точка входа и регистрация функций

```rust
use datacode_sdk::{define_module, dc_fn, types::*, ModuleContext};
use datacode_sdk::abi::{AbiValue, VmContext};

fn my_fn(args: &[AbiValue]) -> AbiValue {
    let x = get_int(args, 0).unwrap_or(0);
    abi_int(x + 1)
}

extern "C" fn register(ctx: *mut VmContext) {
    let mut wrapper = ModuleContext::new(ctx);
    dc_fn!(wrapper, "my_fn", my_fn);
}

define_module!("my_module", 1, 0, register);
```

- **define_module!(имя, major, minor, register_fn)** — генерирует `datacode_module()` и статический дескриптор.
- **dc_fn!(ctx, "имя", путь_к_функции)** — регистрирует функцию; сигнатура должна быть `fn(&[AbiValue]) -> AbiValue`.

### 3. Сборка

```bash
cargo build --release --lib
```

Артефакт: `target/release/lib<имя>.dylib` (macOS) или `target/release/lib<имя>.so` (Linux).

Можно использовать скрипт SDK:

```bash
./tools/build_abi.sh путь/к/вашему_модулю [каталог_вывода]
```

### 4. Использование из DataCode

Положите собранную библиотеку туда, где VM её найдёт (каталог скрипта или текущий каталог). Затем:

```dc
import my_module
println(my_fn(42))   # 43
```

## C

1. Подключите `include/datacode.h` и реализуйте:
   - `const DatacodeModule* datacode_module(void);` — указатель на статический `DatacodeModule`.
   - В `DatacodeModule.register` вызывайте `register_native(ctx, "имя", ваша_c_функция)` для каждого экспорта.

2. Сигнатура нативной функции:

   ```c
   DatacodeValue your_fn(DatacodeVmContext* ctx, const DatacodeValue* args, size_t argc);
   ```

3. Соберите разделяемую библиотеку с экспортом `datacode_module`.

4. Имя файла: `lib<имя_модуля>.so` / `lib<имя_модуля>.dylib`, размещение — как для Rust.

## Поиск библиотеки

VM ищет нативный модуль в таком порядке:

1. **Базовый путь** (обычно каталог запускаемого скрипта).
2. **Текущий рабочий каталог**.

Для `import hello_module` ожидается `libhello_module.dylib` (или `.so`) в этих местах.

**English:** [en/modules.md](../en/modules.md)

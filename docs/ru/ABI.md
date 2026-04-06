# DataCode ABI

Минимальный стабильный контракт, совместимый с C, между виртуальной машиной DataCode и нативными плагинами (`.so` / `.dylib` / `.dll`).

**Источник правды:** Rust-крейт [`datacode_abi`](../../datacode_abi/) в этом репозитории. VM реэкспортирует его как `data_code::abi`. Не дублируйте раскладку полей вручную.

## Версия

ABI версионируется как **major.minor**. При загрузке модуля VM проверяет совместимость (`abi_compatible` в `datacode_abi::version`):

- **Одинаковый major** обязателен.
- **minor модуля ≤ minor VM** (например, VM **1.7** принимает модули **1.0** … **1.7**; модуль **1.8** будет отклонён VM **1.7**).

Текущая версия контракта: **`datacode_abi::DATACODE_ABI_VERSION`** (сейчас **1.7**).

Увеличение **minor** фиксирует аддитивные изменения (новые варианты `AbiValue`, поля дескрипторов и т.д.). **Major** повышают только при несовместимых изменениях раскладки FFI или семантики.

## Типы

### AbiVersion

```c
typedef struct {
    uint16_t major;
    uint16_t minor;
} DatacodeAbiVersion;
```

### AbiValue (Value)

Единый тип значения на границе вызова. Указатели (строки, массивы, ячейки таблицы, байты) действительны только на время нативного вызова.

Перечисление Rust `#[repr(C)]` в `datacode_abi::value::Value` (в SDK экспортируется как `AbiValue`):

- **Int** / **Float** / **Bool** / **Str** / **Null** / **Array** / **Object** — базовый набор.
- **PluginOpaque { tag, id }** — ABI 1.3+, непрозрачные дескрипторы плагина.
- **Table { headers, cells, rows, cols }** — ABI 1.4+, передача VM-таблицы в нативный код.
- **Bytes { ptr, len }** — ABI 1.6+, плотные буферы без массива «по байту».

### VmContext

Непрозрачный контекст, передаваемый в legacy-`register` и в нативные колбэки. Модуль использует функции, доступные через `VmContext` (см. `datacode_abi::vm_context`).

### NativeAbiFn

```c
AbiValue (*NativeAbiFn)(VmContext* ctx, const AbiValue* args, size_t argc);
```

## Точки входа

1. **Предпочтительно (ABI 1.2+):** экспорт **`datacode_module_entry`** → возвращает `*const AbiModuleDescriptor` (символ `DATACODE_MODULE_ENTRY_SYMBOL`). VM читает `abi_version` и статические таблицы экспорта из дескриптора.

2. **Переходный вариант:** **`datacode_module`** (`DATACODE_MODULE_SYMBOL`), возвращающий `DatacodeModule` / старый трёхполевой layout — по-прежнему поддерживается для старых модулей.

Подробности: `datacode_abi::module` и макросы SDK `define_module!`, `define_module_descriptor!`, `define_module_entry!`.

## Схема загрузки (кратко)

1. VM разрешает нативный импорт и загружает динамическую библиотеку.
2. VM находит `datacode_module_entry` или `datacode_module`, проверяет совместимость ABI.
3. Экспорты берутся из дескриптора и/или колбэка `register` (legacy-пути).
4. VM отдаёт объект модуля пользовательскому коду DataCode.

## C-заголовок (`include/datacode.h`)

В репозитории лежит **минимальный / ориентированный на legacy** снимок (раннее подмножество ABI). В нём **нет** полного списка вариантов и полного расклада `AbiModuleDescriptor`. Для нового нативного кода **предпочтительнее Rust SDK** (`datacode_sdk`) или точное копирование определений `#[repr(C)]` из `datacode_abi`, если пишете на C вручную.

**English:** [en/ABI.md](../en/ABI.md)

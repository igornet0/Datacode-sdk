# Документация DataCode SDK

Здесь собрана документация по **ABI**, написанию **нативных модулей** и **примерам** для пакета `datacode_sdk`.

## Документация на русском

| Раздел | Описание |
|--------|----------|
| [ABI (RU)](ru/ABI.md) | Версионирование ABI, типы `AbiValue`, точки входа, загрузка модулей |
| [Нативные модули (RU)](ru/modules.md) | Rust и C: сборка `cdylib`, регистрация функций, поиск библиотеки VM |
| [Примеры (RU)](ru/examples.md) | `hello_module`, скрипты `.dc`, заготовки под math/Telegram |

## English documentation

🇺🇸 Documentation: **[ENREADME.md](ENREADME.md)**.

Отдельные страницы (English):

- [ABI (EN)](en/ABI.md)
- [Native modules (EN)](en/modules.md)
- [Examples (EN)](en/examples.md)

## Где искать код

- Крейт **ABI (источник правды):** `datacode_sdk/datacode_abi/`
- **SDK:** `datacode_sdk/src/` (макросы `define_module!`, `dc_fn!`, типы)
- **C-заголовок (legacy-подмножество):** `datacode_sdk/include/datacode.h`
- **Примеры:** `datacode_sdk/examples/`
- **Сборка:** `datacode_sdk/tools/build_abi.sh`

Версия контракта задаётся в `datacode_abi::DATACODE_ABI_VERSION` (см. также раздел «Версия» в [ru/ABI.md](ru/ABI.md)).

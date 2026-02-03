# Parser fmts Parsing and Conversion of Financial Data in Rust

Проект для парсинга, сериализации и анализа банковских транзакций
в разных форматах.

Проект реализован в виде workspace и состоит из:
- библиотечного крейта `parser`
- CLI-приложения `cli`

---

## Поддерживаемые форматы

- **CSV** - табличный формат
- **TXT** - текстовый формат (одна транзакция на строку)
- **BIN** - бинарный формат

Все форматы могут быть конвертированы друг в друга.

---

## Структура проекта

```text
parser_fmts/
├── Cargo.toml
├── parser/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs
│       ├── model.rs
│       ├──── parsers
│       ├──── csv.rs
│       ├──── txt.rs
│       ├──── bin.rs
│       └── tests/
│           └── roundtrip.rs
└── cli/
    ├── Cargo.toml
    └── src/
        ├── main.rs         # converter
        └── bin/
            └── comparer.rs
```


### CLI Converter
Конвертация данных между форматами.
Пример использования
```
cargo run -p ypbank_cli -- \
  --input data.csv \
  --input-format csv \
  --output-format txt \
  > output.txt
```
Поддерживаемые форматы: csv, txt, bin


### CLI Comparer
Сравнение двух файлов с транзакциями в любых форматах.

Пример использования
```
cargo run -p ypbank_cli --bin comparer -- \
--file1 data.bin --format1 bin \
--file2 data.csv --format2 csv
```

### Тесты
Для библиотеки реализованы модульные тесты:
```
cargo test
```
Проверяется корректность сериализации
(CSV -> struct -> CSV, BIN -> struct -> BIN).

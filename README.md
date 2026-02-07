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
│           └── mod.rs
│           └── bin.rs
│           └── csv.rs
│           └── text.rs
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
cd cli
```

```
 cargo run --bin cli -- \
  --input ../examples/transactions.csv \
  --input-format csv \
  --output-format bin \
  > ../examples/transactions.bin
```
Поддерживаемые форматы: csv, txt, bin


### CLI Comparer
Сравнение двух файлов с транзакциями в любых форматах.

Пример использования

```
cd cli
```

```
cargo run --bin comparer -- \
  --file1 ../examples/transactions.csv \
  --format1 csv \
  --file2 ../examples/transactions.txt \
  --format2 txt
```

### Тесты
Для библиотеки реализованы модульные тесты:
```
cargo test
```
Проверяется корректность сериализации
(CSV -> struct -> CSV, BIN -> struct -> BIN).

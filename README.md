Year Helper
====================

[![CI](https://github.com/magiclen/year-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/year-helper/actions/workflows/ci.yml)

This library provides some useful functions to deal with years.

## Usage

```rust
let year = 2020;
let month = 2;
let leap_year = year_helper::is_leap_year(year);

let days_in_month = year_helper::get_days_in_month(leap_year, month);
let days_in_year = year_helper::get_days_in_year(leap_year);
```

## Crates.io

https://crates.io/crates/year-helper

## Documentation

https://docs.rs/year-helper

## License

[MIT](LICENSE)
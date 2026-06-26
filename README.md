Year Helper
====================

[![CI](https://github.com/magiclen/year-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/year-helper/actions/workflows/ci.yml)

This library provides some useful functions to deal with years.

The `year` parameter follows the ISO 8601 proleptic Gregorian calendar, using astronomical year numbering like Chrono's `NaiveDate`. This means year `0` is `1 BCE`, year `-1` is `2 BCE`, and so on.

## Usage

```rust
let year = 2020;
let month = 2;

let leap_year = year_helper::is_leap_year(year);

let days_in_month = year_helper::get_days_in_month(year, month);
let days_in_year = year_helper::get_days_in_year(year);

// or use the following functions if the year has been determined whether it is a leap year
let days_in_month = year_helper::get_days_in_month_with_leap_year(leap_year, month);
let days_in_year = year_helper::get_days_in_year_with_leap_year(leap_year);
```

## Crates.io

https://crates.io/crates/year-helper

## Documentation

https://docs.rs/year-helper

## License

[MIT](LICENSE)

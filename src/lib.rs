/*!
# Year Helper

This library provides some useful functions to deal with years.

## Usage

```rust
let year = 2020;
let month = 2;

let leap_year = year_helper::is_leap_year(year);

let days_in_month = year_helper::get_days_in_month(year, month);
let days_in_year = year_helper::get_days_in_year(year);

// or use the following functions if the year has been determined where it is a leap year
let days_in_month = year_helper::get_days_in_month_2(leap_year, month);
let days_in_year = year_helper::get_days_in_year_2(leap_year);
```
 */

#![no_std]

/// Determine whether a year is a leap year or not.
///
/// A year is a leap year if following conditions are satisfied:
///
/// * Year is multiple of 400.
/// * Year is multiple of 4 and not multiple of 100.
#[inline]
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Calculate how many days in a month.
///
/// If the input `month` is not between `1` and `12`, this function will return `None`.
#[inline]
pub const fn get_days_in_month(year: i32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => {
            if is_leap_year(year) {
                Some(29)
            } else {
                Some(28)
            }
        }
        _ => None,
    }
}

/// Calculate how many days in a month.
///
/// If the input `month` is not between `1` and `12`, this function will return `None`.
#[inline]
pub const fn get_days_in_month_2(leap_year: bool, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => {
            if leap_year {
                Some(29)
            } else {
                Some(28)
            }
        }
        _ => None,
    }
}

/// Calculate how many days in a year.
#[inline]
pub const fn get_days_in_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

/// Calculate how many days in a year.
#[inline]
pub const fn get_days_in_year_2(leap_year: bool) -> u16 {
    if leap_year {
        366
    } else {
        365
    }
}

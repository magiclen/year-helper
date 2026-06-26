#[test]
fn test_is_leap_year() {
    let cases = [
        (1999, false),
        (1900, false),
        (2000, true),
        (2001, false),
        (2002, false),
        (2003, false),
        (2004, true),
        (2100, false),
        (2400, true),
        (0, true),
        (-1, false),
        (-4, true),
        (-100, false),
        (-400, true),
    ];

    for (year, expect) in cases {
        assert_eq!(expect, year_helper::is_leap_year(year));
    }
}

#[test]
fn test_get_days_in_month() {
    let cases = [
        (1999, 1, Some(31)),
        (1999, 2, Some(28)),
        (1999, 3, Some(31)),
        (1999, 4, Some(30)),
        (1999, 12, Some(31)),
        (1999, 11, Some(30)),
        (2000, 1, Some(31)),
        (2000, 2, Some(29)),
        (2000, 3, Some(31)),
        (2000, 4, Some(30)),
        (2000, 12, Some(31)),
        (2000, 11, Some(30)),
        (2000, 0, None),
        (2000, 13, None),
        (2000, 255, None),
    ];

    for (year, month, expect) in cases {
        assert_eq!(expect, year_helper::get_days_in_month(year, month));
    }
}

#[test]
fn test_get_days_in_month_with_leap_year() {
    let cases = [
        (false, 1, Some(31)),
        (false, 2, Some(28)),
        (true, 2, Some(29)),
        (true, 4, Some(30)),
        (true, 255, None),
    ];

    for (leap_year, month, expect) in cases {
        assert_eq!(expect, year_helper::get_days_in_month_with_leap_year(leap_year, month));
    }
}

#[test]
fn test_get_days_in_year() {
    let cases = [
        (1999, 365),
        (1900, 365),
        (2000, 366),
        (2001, 365),
        (2002, 365),
        (2003, 365),
        (2004, 366),
        (2100, 365),
        (2400, 366),
    ];

    for (year, expect) in cases {
        assert_eq!(expect, year_helper::get_days_in_year(year));
    }
}

#[test]
fn test_get_days_in_year_with_leap_year() {
    assert_eq!(365, year_helper::get_days_in_year_with_leap_year(false));
    assert_eq!(366, year_helper::get_days_in_year_with_leap_year(true));
}

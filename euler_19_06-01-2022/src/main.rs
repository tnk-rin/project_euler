fn main() {
    let mut week_day = 2u8;
    let mut month = 1u8;
    let mut year = 1901u16;
    let mut total_sunday_starts = 0u16;

    loop {
        (week_day, month, year) = update_week_day(week_day, month, year);
        if month == 12 && year == 2000 {
            break;
        }

        if week_day == 7 {
            total_sunday_starts += 1;
            continue;
        }
    }
    println!("{:#?}", (week_day, month, year));
    println!("{}", total_sunday_starts);
}

fn update_week_day(mut day: u8, mut month: u8, mut year: u16) -> (u8, u8, u16) {
    day += month_adder(month, year);
    if day > 7 { day -= 7; }
    month += 1;
    if month > 12 { month -= 12; year += 1; }
    (day, month, year)
}

fn month_adder(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 3,
        4 | 6 | 9 | 11 => 2,
        2 => {
            if is_leap_year(year) { 1 } else { 0 }
        },
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    if (year % 100) == 0 {
        if (year % 400) == 0 {
            return true;
        }
    } else if (year % 4) == 0 {
        return true
    }
    return false;
}

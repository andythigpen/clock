use js_sys::Date;

pub fn short_day(day: u32) -> &'static str {
    match day {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => panic!("invalid day {day}"),
    }
}

pub fn short_month(month: u32) -> &'static str {
    match month {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        4 => "May",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
        10 => "Nov",
        11 => "Dec",
        _ => panic!("invalid month {month}"),
    }
}

pub fn formatted_date(date: &Date) -> String {
    let day = short_day(date.get_day());
    let month = short_month(date.get_month());
    let date = date.get_date();
    format!("{day} {month} {date}")
}

pub fn formatted_time(date: &Date) -> String {
    let hour = twelve_hour(date.get_hours());
    let minutes = date.get_minutes();
    format!("{hour:02}:{minutes:02}")
}

pub fn twelve_hour(hour: u32) -> u32 {
    match hour % 12 {
        0 => 12,
        h => h,
    }
}

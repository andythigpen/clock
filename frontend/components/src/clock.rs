use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::prelude::*;

fn short_day(day: u32) -> &'static str {
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

fn short_month(month: u32) -> &'static str {
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

fn formatted_date(date: &Date) -> String {
    let day = short_day(date.get_day());
    let month = short_month(date.get_month());
    let date = date.get_date();
    format!("{day} {month} {date}")
}

fn formatted_time(date: &Date) -> String {
    let hour = match date.get_hours() % 12 {
        0 => 12,
        h => h,
    };
    let minutes = date.get_minutes();
    format!("{hour:02}:{minutes:02}")
}

#[function_component(Clock)]
pub fn clock() -> Html {
    let now = use_state(|| Date::new_0());

    use_effect_with_deps(
        {
            let now = now.clone();
            move |_| {
                let interval = Interval::new(1000, move || {
                    now.set(Date::new_0());
                });
                || {
                    interval.cancel();
                }
            }
        },
        (),
    );

    html! {
        <div class="flex flex-col min-w-[40%]">
            <div class="text-8xl">
                {formatted_date(&*now)}
            </div>
            <div class="text-[20rem] leading-[20rem]">
                {formatted_time(&*now)}
            </div>
        </div>
    }
}

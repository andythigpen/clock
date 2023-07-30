use js_sys::Date;
use stores::WeatherStore;
use time::{format_description::well_known::Iso8601, OffsetDateTime, UtcOffset};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{date_util::*, sun_icon};

#[function_component(Sun)]
pub fn sun() -> Html {
    let sun = use_selector(|s: &WeatherStore| s.sun.clone());

    let date = OffsetDateTime::parse(
        if sun.rising { &sun.rise } else { &sun.set },
        &Iso8601::DEFAULT,
    )
    .unwrap();
    let off = -(Date::new_0().get_timezone_offset() as i32 / 60);
    let date = date.to_offset(UtcOffset::from_hms(off as i8, 0, 0).unwrap());
    let hour = date.hour();
    let hour = twelve_hour(hour as u32);
    let hour = format!("{hour:02}");
    let min = date.minute();
    let min = format!("{min:02}");
    html! {
        <div class="flex-1 flex flex-row justify-center items-center">
            <img src={sun_icon(sun.rising)} class="h-96"/>
            <span class="text-9xl ml-8">{"@"}</span>
            <span class="text-[15rem]">{hour}{":"}{min}</span>
        </div>
    }
}

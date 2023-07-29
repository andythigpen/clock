use dto::WeatherCondition;
use stores::WeatherStore;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::date_util::*;
use crate::weather_util::*;

#[function_component(WeatherPrecipitation)]
pub fn weather_precipitation() -> Html {
    let weather = use_selector(|s: &WeatherStore| s.weather.clone());
    let current = weather.condition.clone();

    let change = match precipitation_change(&weather) {
        Some(f) => f,
        None => {
            return html! {
                <div class="flex-1 flex flex-row justify-center items-center">
                    <span class="grow text-[6rem] leading-[6rem]">{"No precipitation change"}</span>
                </div>
            }
        }
    };

    let probability = match change.precipitation_chance {
        0...49 => "possible",
        50...69 => "likely",
        70...100 => "very likely",
        _ => "",
    };
    let action = if is_precipitation(&current) && !is_precipitation(&change.condition) {
        "stopping"
    } else if !is_precipitation(&current) && is_precipitation(&change.condition) {
        probability
    } else {
        "continues"
    };

    let condition = if is_precipitation(&current) {
        &current
    } else {
        &change.condition
    };

    let precipitation_type = match condition {
        WeatherCondition::Rain
        | WeatherCondition::Thunderstorms
        | WeatherCondition::ThunderstormsRain => "Rain",
        WeatherCondition::Sleet => "Sleet",
        WeatherCondition::Snow => "Snow",
        WeatherCondition::Hail => "Hail",
        _ => "Unknown",
    };

    let hour = twelve_hour(change.hour as u32);
    let hour = format!("{hour:02}");

    html! {
        <div class="flex-1 flex flex-row justify-center items-center">
            <img src={weather_icon(condition)} class="h-96"/>
            <span class="grow text-[9rem] leading-[9rem]">{precipitation_type}{" "}<span class="text-[9rem]">{action}</span></span>
            <span class="text-8xl ml-8">{"@"}</span>
            <span class="text-[10rem]">{hour}</span>
        </div>
    }
}

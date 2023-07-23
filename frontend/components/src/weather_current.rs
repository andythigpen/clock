use dto::WeatherCondition;
use js_sys::Date;
use stores::WeatherStore;
use yew::prelude::*;
use yewdux::prelude::*;

fn is_day() -> bool {
    let now = Date::new_0();
    let hour = now.get_hours();
    hour < 6 || hour > 19
}

fn weather_icon(condition: &WeatherCondition) -> String {
    let svg = match condition {
        WeatherCondition::Cloudy => {
            if is_day() {
                "overcast-day"
            } else {
                "overcast-night"
            }
        }
        WeatherCondition::Clear => {
            if is_day() {
                "clear-day"
            } else {
                "clear-night"
            }
        }
        WeatherCondition::Fog => {
            if is_day() {
                "fog-day"
            } else {
                "fog-night"
            }
        }
        WeatherCondition::Hail => "hail",
        WeatherCondition::Thunderstorms => {
            if is_day() {
                "thunderstorms-day"
            } else {
                "thunderstorms-night"
            }
        }
        WeatherCondition::ThunderstormsRain => {
            if is_day() {
                "thunderstorms-day-rain"
            } else {
                "thunderstorms-night-rain"
            }
        }
        WeatherCondition::PartlyCloudy => {
            if is_day() {
                "partly-cloudy-day"
            } else {
                "partly-cloudy-night"
            }
        }
        WeatherCondition::Rain => "rain",
        WeatherCondition::Snow => "snow",
        WeatherCondition::Sleet => "sleet",
        WeatherCondition::Windy => "wind",
        WeatherCondition::Exceptional => "code-red",
        WeatherCondition::Unknown => "code-orange",
    };
    format!("/assets/icons/weather/{svg}.svg")
}

#[function_component(WeatherCurrent)]
pub fn weather_current() -> Html {
    let condition = use_selector(|s: &WeatherStore| s.weather.condition.clone());
    let temp = use_selector(|s: &WeatherStore| s.weather.temp_f);
    html! {
        <div class="flex-1 flex flex-row gap-4 justify-center items-center">
            <img src={weather_icon(&*condition)} class="h-96"/>
            <span class="text-[14rem]">{temp}<span class="text-9xl">{"°F"}</span></span>
        </div>
    }
}

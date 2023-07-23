use stores::WeatherStore;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::weather_util::*;

#[function_component(WeatherCurrent)]
pub fn weather_current() -> Html {
    let condition = use_selector(|s: &WeatherStore| s.weather.condition.clone());
    let temp = use_selector(|s: &WeatherStore| s.weather.temp_f);
    html! {
        <div class="flex-1 flex flex-row gap-4 justify-center items-center">
            <img src={weather_icon(&*condition)} class="h-96"/>
            <span class="text-[15rem]">{temp}<span class="text-9xl">{"°F"}</span></span>
        </div>
    }
}

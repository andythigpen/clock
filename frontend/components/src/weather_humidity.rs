use stores::WeatherStore;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(WeatherHumidity)]
pub fn weather_humidity() -> Html {
    let humidity = use_selector(|s: &WeatherStore| s.weather.humidity);
    html! {
        <div class="flex-1 flex flex-row justify-center items-center">
            <img src="/assets/icons/weather/humidity.svg" class="h-96"/>
            <span class="text-[15rem]">{humidity}</span>
        </div>
    }
}

use yew::prelude::*;

#[function_component(WeatherHumidity)]
pub fn weather_humidity() -> Html {
    html! {
        <div class="flex-1 flex flex-row justify-center items-center">
            <img src="/assets/icons/weather/humidity.svg" class="h-96"/>
            <span class="text-[11rem]">{"57"}</span>
        </div>
    }
}

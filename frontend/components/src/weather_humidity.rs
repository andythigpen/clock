use yew::prelude::*;

#[function_component(WeatherHumidity)]
pub fn weather_humidity() -> Html {
    html! {
        <div class="flex-1 flex flex-row justify-center items-center">
            <span class="text-[11rem] w-96">{"57"}</span>
            <img src="/assets/icons/weather/humidity.svg" class="h-96"/>
        </div>
    }
}

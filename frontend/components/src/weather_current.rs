use yew::prelude::*;

#[function_component(WeatherCurrent)]
pub fn weather_current() -> Html {
    html! {
        <div class="flex-1 flex flex-row gap-4 justify-center items-center">
            <img src="/assets/icons/weather/clear-day.svg" class="h-96"/>
            <span class="text-[14rem]">{"89"}<span class="text-9xl">{"°F"}</span></span>
        </div>
    }
}

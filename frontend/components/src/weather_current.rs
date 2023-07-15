use yew::prelude::*;

#[function_component(WeatherCurrent)]
pub fn weather_current() -> Html {
    html! {
        <div class="flex-1 flex flex-row gap-4 justify-center">
            <img src="/assets/icons/weather/clear-day.svg" class="h-96"/>
            <div class="flex flex-col items-center">
                <div class="flex flex-row items-center">
                    <span class="text-[11rem]">{"89"}</span>
                    <img src="/assets/icons/weather/fahrenheit.svg" class="h-96 -m-20"/>
                </div>
                <div class="flex-1 text-7xl">{"Sunny"}</div>
            </div>
        </div>
    }
}

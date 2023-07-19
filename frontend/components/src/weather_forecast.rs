use yew::prelude::*;

#[function_component(WeatherForecast)]
pub fn weather_forecast() -> Html {
    html! {
        <div class="flex-1 flex flex-col">
            <div class="flex-1 flex flex-row gap-4 justify-center items-center">
                <img src="/assets/icons/weather/thunderstorms-rain.svg" class="h-96"/>
                <span class="text-[14rem]">{"82"}<span class="text-9xl">{"°F"}</span></span>
                <span class="text-9xl ml-8">{"@"}</span>
                <span class="text-[13rem]">{"11"}</span>
            </div>
        </div>
    }
}

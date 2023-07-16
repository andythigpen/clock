use components::{clock::Clock, weather_current::WeatherCurrent};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="bg-gradient-45 from-darkbg-900 via-darkbg-800 to-darkbg-900 flex items-center justify-center h-screen w-screen animate-gradient bg-[size:400%_400%]">
            <Clock />
            <WeatherCurrent />
        </div>
    }
}

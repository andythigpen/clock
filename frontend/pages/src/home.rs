use components::{clock::Clock, weather_current::WeatherCurrent};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Clock />
            <WeatherCurrent />
        </>
    }
}

use stores::WeatherStore;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::date_util::*;
use crate::weather_util::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub index: u8,
}

#[function_component(WeatherForecast)]
pub fn weather_forecast(props: &Props) -> Html {
    let index = props.index as usize;
    let store = use_store_value::<WeatherStore>();
    if index >= store.weather.forecast.len() {
        return html! {
            <div class="flex-1 flex flex-col">
                {"Forecast unavailable"}
            </div>
        };
    }
    let forecast = store.weather.forecast[index].clone();
    let hour = twelve_hour(forecast.hour as u32);
    let hour = format!("{hour:02}");
    html! {
        <div class="flex-1 flex flex-col">
            <div class="flex-1 flex flex-row gap-4 justify-center items-center">
                <img src={weather_icon(&forecast.condition)} class="h-96"/>
                <span class="text-[15rem]">{forecast.temp_f}<span class="text-9xl">{"°F"}</span></span>
                <span class="text-9xl ml-8">{"@"}</span>
                <span class="text-[15rem]">{hour}</span>
            </div>
        </div>
    }
}

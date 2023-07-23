use dto::Weather;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Default)]
pub struct WeatherStore {
    pub weather: Weather,
}

use dto::Weather;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Default)]
pub struct WeatherStore {
    pub weather: Weather,
    pub ready: bool,
}

#[derive(Clone, PartialEq)]
pub enum Widget {
    WeatherCurrent,
    WeatherForecast(u8),
    WeatherHumidity,
    WeatherPrecipitation,
}

#[derive(Clone, PartialEq, Store, Default)]
pub struct WidgetStore {
    enabled: Vec<Widget>,
    current: usize,
    pub display: bool,
    pub opacity: u8,
}

impl WidgetStore {
    pub fn current(&self) -> Option<Widget> {
        if self.enabled.is_empty() {
            return None;
        }
        Some(self.enabled[self.current].clone())
    }

    pub fn next(&mut self) -> Option<Widget> {
        self.current += 1;
        if self.current >= self.enabled.len() {
            self.current = 0
        }
        self.current()
    }

    pub fn enable(&mut self, widget: Widget) {
        self.enabled.push(widget)
    }

    pub fn disable(&mut self, widget: Widget) {
        self.enabled = self
            .enabled
            .iter()
            .filter(|w| *w != &widget)
            .cloned()
            .collect();
    }
}

use dto::{Sun, Weather};
use sorted_vec::SortedSet;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Default)]
pub struct WeatherStore {
    pub weather: Weather,
    pub sun: Sun,
}

/// Ordering here determines the display order in the frontend carousel.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Widget {
    WeatherCurrent,
    WeatherForecast(u8),
    WeatherHumidity,
    WeatherPrecipitation,
    Sun,
}

#[derive(Clone, PartialEq, Store, Default)]
pub struct WidgetStore {
    enabled: SortedSet<Widget>,
    current: usize,
    pub display: bool,
    pub opacity: u8,
}

impl WidgetStore {
    pub fn current(&self) -> Option<Widget> {
        if self.enabled.is_empty() {
            return None;
        }
        if self.current >= self.enabled.len() {
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
        if self.enabled.contains(&widget) {
            return;
        }
        self.enabled.push(widget);
    }

    pub fn disable(&mut self, widget: Widget) {
        self.enabled.remove_item(&widget);
        if self.current >= self.enabled.len() {
            self.current = 0;
        }
    }
}

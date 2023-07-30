use dto::{HourForecast, Sun, Weather, WeatherCondition};
use js_sys::Date;
use time::{ext::NumericalDuration, format_description::well_known::Iso8601, OffsetDateTime};

pub fn is_day() -> bool {
    let now = Date::new_0();
    let hour = now.get_hours();
    hour >= 6 && hour <= 19
}

pub fn weather_icon(condition: &WeatherCondition) -> String {
    let svg = match condition {
        WeatherCondition::Cloudy => {
            if is_day() {
                "overcast-day"
            } else {
                "overcast-night"
            }
        }
        WeatherCondition::Clear => {
            if is_day() {
                "clear-day"
            } else {
                "clear-night"
            }
        }
        WeatherCondition::Fog => {
            if is_day() {
                "fog-day"
            } else {
                "fog-night"
            }
        }
        WeatherCondition::Hail => "hail",
        WeatherCondition::Thunderstorms => {
            if is_day() {
                "thunderstorms-day"
            } else {
                "thunderstorms-night"
            }
        }
        WeatherCondition::ThunderstormsRain => {
            if is_day() {
                "thunderstorms-day-rain"
            } else {
                "thunderstorms-night-rain"
            }
        }
        WeatherCondition::PartlyCloudy => {
            if is_day() {
                "partly-cloudy-day"
            } else {
                "partly-cloudy-night"
            }
        }
        WeatherCondition::Rain => "rain",
        WeatherCondition::Snow => "snow",
        WeatherCondition::Sleet => "sleet",
        WeatherCondition::Windy => "wind",
        WeatherCondition::Exceptional => "code-red",
        WeatherCondition::Unknown => "code-orange",
    };
    format!("/assets/icons/weather/{svg}.svg")
}

pub fn is_precipitation(condition: &WeatherCondition) -> bool {
    match condition {
        WeatherCondition::Hail
        | WeatherCondition::Rain
        | WeatherCondition::Sleet
        | WeatherCondition::Snow
        | WeatherCondition::Thunderstorms
        | WeatherCondition::ThunderstormsRain => true,
        _ => false,
    }
}

/// Returns Some if the precipitation conditions are changing over the forecast window, None
/// otherwise.
pub fn precipitation_change(weather: &Weather) -> Option<HourForecast> {
    let current = is_precipitation(&weather.condition);

    let forecast: Vec<HourForecast> = weather
        .forecast
        .iter()
        .skip_while(|c| is_precipitation(&c.condition) == current)
        .take(1)
        .map(|c| c.clone())
        .collect();

    forecast.first().cloned()
}

/// Returns true if the rise/set fields are occurring soon (< 2h).
pub fn sun_change(sun: &Sun) -> bool {
    let now = Date::new_0().to_iso_string();
    let now = OffsetDateTime::parse(&now.as_string().unwrap(), &Iso8601::DEFAULT).unwrap();
    let soon = now.checked_add(2.hours()).unwrap();
    let rise_date = OffsetDateTime::parse(&sun.rise, &Iso8601::DEFAULT).unwrap();
    let set_date = OffsetDateTime::parse(&sun.set, &Iso8601::DEFAULT).unwrap();

    (rise_date >= now && rise_date < soon) || (set_date >= now && set_date < soon)
}

pub fn sun_icon(rising: bool) -> String {
    let svg = if rising { "sunrise" } else { "sunset" };
    format!("/assets/icons/weather/{svg}.svg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precipitation_change() {
        let weather = Weather {
            condition: WeatherCondition::Cloudy,
            temp_f: 80,
            humidity: 65,
            forecast: vec![
                HourForecast {
                    condition: WeatherCondition::Cloudy,
                    temp_f: 81,
                    hour: 11,
                    precipitation_chance: 20,
                },
                HourForecast {
                    condition: WeatherCondition::Thunderstorms,
                    temp_f: 81,
                    hour: 12,
                    precipitation_chance: 40,
                },
                HourForecast {
                    condition: WeatherCondition::Thunderstorms,
                    temp_f: 82,
                    hour: 13,
                    precipitation_chance: 60,
                },
            ],
        };
        assert_eq!(
            precipitation_change(&weather),
            Some(HourForecast {
                condition: WeatherCondition::Thunderstorms,
                temp_f: 81,
                hour: 12,
                precipitation_chance: 40,
            })
        );
    }

    #[test]
    fn test_precipitation_no_change() {
        let weather = Weather {
            condition: WeatherCondition::Cloudy,
            temp_f: 80,
            humidity: 65,
            forecast: vec![
                HourForecast {
                    condition: WeatherCondition::Clear,
                    temp_f: 81,
                    hour: 11,
                    precipitation_chance: 20,
                },
                HourForecast {
                    condition: WeatherCondition::PartlyCloudy,
                    temp_f: 81,
                    hour: 12,
                    precipitation_chance: 30,
                },
                HourForecast {
                    condition: WeatherCondition::Cloudy,
                    temp_f: 82,
                    hour: 13,
                    precipitation_chance: 20,
                },
            ],
        };
        assert_eq!(precipitation_change(&weather), None);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Message {
    Weather(Weather),
    TaskReminders(Vec<Reminder>),
    CalendarReminders(Vec<Reminder>),
    Alerts(Vec<Alert>),
    Sun(Sun),
    DisplayStateChange(DisplayState),
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum WeatherCondition {
    Clear,
    Cloudy,
    Exceptional,
    Fog,
    Hail,
    PartlyCloudy,
    Rain,
    Sleet,
    Snow,
    Thunderstorms,
    ThunderstormsRain,
    #[default]
    Unknown,
    Windy,
}

impl From<&str> for WeatherCondition {
    fn from(value: &str) -> Self {
        match value {
            "clear-night" => Self::Clear,
            "cloudy" => Self::Cloudy,
            "fog" => Self::Fog,
            "hail" => Self::Hail,
            "lightning" | "thunderstorms" => Self::Thunderstorms,
            "lightning-rainy" | "thunderstorms-rain" => Self::ThunderstormsRain,
            "partlycloudy" | "partly-cloudy" => Self::PartlyCloudy,
            "pouring" | "rainy" | "rain" => Self::Rain,
            "snowy" | "snow" => Self::Snow,
            "snowy-rainy" | "snowy-rain" | "sleet" => Self::Sleet,
            "sunny" | "clear-day" => Self::Clear,
            "windy" | "windy-variant" => Self::Windy,
            "exceptional" | "alert" => Self::Exceptional,
            _ => Self::Unknown,
        }
    }
}

impl From<&String> for WeatherCondition {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Weather {
    pub condition: WeatherCondition,
    pub temp_f: u8,
    pub humidity: u8,
    pub forecast: Vec<HourForecast>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct HourForecast {
    pub condition: WeatherCondition,
    pub temp_f: u8,
    pub hour: u8,
    pub precipitation_chance: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub name: String,
    pub due: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Alert {
    pub name: String,
    pub severity: AlertSeverity,
    pub persist: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Sun {
    pub rise: String,
    pub set: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DisplayState {
    On,
    Off,
}

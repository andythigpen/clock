use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use dto::{HourForecast, Message};
use log::{debug, error, warn};
use reqwest::{header::AUTHORIZATION, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use tokio::sync::broadcast::Sender;
use tokio::{macros::support::poll_fn, time::Instant};
use tokio_util::time::DelayQueue;

#[derive(Deserialize, Debug)]
struct Forecast {
    datetime: String,
    condition: String,
    precipitation_probability: u8,
    temperature: u8,
}

#[derive(Deserialize, Debug)]
struct WeatherEntityAttributes {
    temperature: u8,
    #[serde(default)]
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct WeatherEntity {
    state: String,
    attributes: WeatherEntityAttributes,
}

#[derive(Deserialize, Debug)]
struct ForecastEntityAttributes {
    forecast: Vec<Forecast>,
}

#[derive(Deserialize, Debug)]
struct ForecastEntity {
    attributes: ForecastEntityAttributes,
}

#[derive(Deserialize, Debug)]
struct SunEntityAttributes {
    rising: bool,
    next_rising: String,
    next_setting: String,
}

#[derive(Deserialize, Debug)]
struct SunEntity {
    attributes: SunEntityAttributes,
}

pub async fn fetch_entity(entity_id: &str) -> Result<Value> {
    let client = reqwest::Client::new();
    let base_url = env::var("HA_URL").unwrap();
    let token = env::var("HA_TOKEN").unwrap();
    let response = client
        .get(format!("{base_url}/api/states/{entity_id}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await?;
    match response.status() {
        StatusCode::OK => {
            let json = response.json().await?;
            Ok(json)
        }
        e @ _ => {
            error!("unexpected response: {e:?}");
            Err(anyhow!(e))
        }
    }
}

async fn fetch_weather(weather_entity_id: &str, forecast_entity_id: &str) -> Result<Message> {
    let json = fetch_entity(&weather_entity_id).await?;
    let entity: WeatherEntity = serde_json::from_value(json)?;
    let json = fetch_entity(&forecast_entity_id).await?;
    let forecast_entity: ForecastEntity = serde_json::from_value(json)?;

    let mut forecasts = vec![];
    for hour in forecast_entity.attributes.forecast {
        let datetime = OffsetDateTime::parse(&hour.datetime, &Iso8601::DEFAULT)?;
        let now = OffsetDateTime::now_utc().to_offset(datetime.offset());
        if datetime < now {
            continue;
        }
        forecasts.push(HourForecast {
            condition: (&hour.condition).into(),
            temp_f: hour.temperature,
            precipitation_chance: hour.precipitation_probability,
            hour: datetime.hour(),
        });
        if forecasts.len() >= 3 {
            break;
        }
    }

    Ok(Message::Weather(dto::Weather {
        condition: (&entity.state).into(),
        temp_f: entity.attributes.temperature,
        humidity: entity.attributes.humidity,
        forecast: forecasts,
    }))
}

async fn fetch_sun(entity_id: &str) -> Result<Message> {
    let json = fetch_entity(&entity_id).await?;
    let entity: SunEntity = serde_json::from_value(json)?;
    Ok(Message::Sun(dto::Sun {
        rising: entity.attributes.rising,
        rise: entity.attributes.next_rising,
        set: entity.attributes.next_setting,
    }))
}

#[derive(Debug, Clone)]
pub enum PollerEvent {
    FetchWeather {
        weather_entity_id: String,
        forecast_entity_id: String,
    },
    FetchSun {
        entity_id: String,
    },
}

struct Poller {
    tx: Sender<Message>,
    queue: DelayQueue<PollerEvent>,
}

impl Poller {
    async fn handle(&self, event: PollerEvent) -> Result<Message> {
        match event {
            PollerEvent::FetchWeather {
                weather_entity_id,
                forecast_entity_id,
            } => fetch_weather(&weather_entity_id, &forecast_entity_id).await,
            PollerEvent::FetchSun { entity_id } => fetch_sun(&entity_id).await,
        }
    }

    fn init(&mut self) {
        let weather_entity_id = env::var("HA_WEATHER_ENTITY").unwrap();
        let forecast_entity_id = env::var("HA_FORECAST_ENTITY").unwrap();
        self.queue.insert_at(
            PollerEvent::FetchWeather {
                weather_entity_id,
                forecast_entity_id,
            },
            Instant::now(),
        );

        let sun_entity_id = env::var("HA_SUN_ENTITY").unwrap();
        self.queue.insert_at(
            PollerEvent::FetchSun {
                entity_id: sun_entity_id,
            },
            Instant::now(),
        );
    }

    async fn loop_forever(&mut self) {
        self.init();
        loop {
            tokio::select! {
                Some(expired) = poll_fn(|cx| self.queue.poll_expired(cx)) => {
                    debug!("schedule expired: {expired:?}");
                    let event = expired.into_inner();
                    self.queue.insert(event.clone(), Duration::from_secs(60));
                    match self.handle(event).await {
                        Ok(msg) => {
                            if let Err(e) = self.tx.send(msg) {
                                error!("failed to send msg: {e}");
                            }
                        }
                        Err(e) => error!("error handling event: {e}"),
                    }
                }
                else => { break }
            }
        }
        warn!("exiting home assistant poller loop");
    }
}

pub async fn run(tx: Sender<Message>) {
    let mut poller = Poller {
        tx,
        queue: DelayQueue::new(),
    };
    poller.loop_forever().await;
}

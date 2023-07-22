use std::env;
use std::time::Duration;

use anyhow::{anyhow, Result};
use dto::{HourForecast, Message};
use log::{debug, error, warn};
use reqwest::{header::AUTHORIZATION, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use tokio::sync::mpsc::Sender;
use tokio::{macros::support::poll_fn, time::Instant};
use tokio_util::time::DelayQueue;

#[derive(Deserialize)]
struct WeatherEntityForecast {
    datetime: String,
    condition: String,
    precipitation_probability: u8,
    temperature: u8,
}

#[derive(Deserialize)]
struct WeatherEntityAttributes {
    temperature: u8,
    humidity: u8,
    forecast: Vec<WeatherEntityForecast>,
}

#[derive(Deserialize)]
struct WeatherEntity {
    state: String,
    attributes: WeatherEntityAttributes,
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

async fn fetch_weather(entity_id: &str) -> Result<Message> {
    let json = fetch_entity(&entity_id).await?;
    let entity: WeatherEntity = serde_json::from_value(json)?;
    let forecast = entity.attributes.forecast;
    Ok(Message::Weather(dto::Weather {
        condition: (&entity.state).into(),
        temp_f: entity.attributes.temperature,
        humidity: entity.attributes.humidity,
        forecast: vec![
            HourForecast {
                condition: (&forecast[0].condition).into(),
                temp_f: forecast[0].temperature,
                precipitation_chance: forecast[0].precipitation_probability,
                hour: OffsetDateTime::parse(&forecast[0].datetime, &Iso8601::DEFAULT)?.hour(),
            },
            HourForecast {
                condition: (&forecast[1].condition).into(),
                temp_f: forecast[1].temperature,
                precipitation_chance: forecast[1].precipitation_probability,
                hour: OffsetDateTime::parse(&forecast[1].datetime, &Iso8601::DEFAULT)?.hour(),
            },
            HourForecast {
                condition: (&forecast[2].condition).into(),
                temp_f: forecast[2].temperature,
                precipitation_chance: forecast[2].precipitation_probability,
                hour: OffsetDateTime::parse(&forecast[2].datetime, &Iso8601::DEFAULT)?.hour(),
            },
            HourForecast {
                condition: (&forecast[3].condition).into(),
                temp_f: forecast[3].temperature,
                precipitation_chance: forecast[3].precipitation_probability,
                hour: OffsetDateTime::parse(&forecast[3].datetime, &Iso8601::DEFAULT)?.hour(),
            },
        ],
    }))
}

#[derive(Debug, Clone)]
pub enum PollerEvent {
    FetchWeather { entity_id: String },
}

struct Poller {
    tx: Sender<Message>,
    queue: DelayQueue<PollerEvent>,
}

impl Poller {
    async fn handle(&self, event: PollerEvent) -> Result<Message> {
        match event {
            PollerEvent::FetchWeather { entity_id } => fetch_weather(&entity_id).await,
        }
    }

    async fn loop_forever(&mut self) {
        let weather_entity_id = env::var("HA_WEATHER_ENTITY").unwrap();
        self.queue.insert_at(
            PollerEvent::FetchWeather {
                entity_id: weather_entity_id,
            },
            Instant::now(),
        );

        loop {
            tokio::select! {
                Some(expired) = poll_fn(|cx| self.queue.poll_expired(cx)) => {
                    debug!("schedule expired: {expired:?}");
                    let event = expired.into_inner();
                    self.queue.insert(event.clone(), Duration::from_secs(30));
                    match self.handle(event).await {
                        Ok(msg) => {
                            if let Err(e) = self.tx.send(msg).await {
                                error!("failed to send msg: {e:?}");
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

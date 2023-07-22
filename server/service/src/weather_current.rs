// use anyhow::Result;
// // use axum::extract::ws::{Message, WebSocket};
// // use futures_util::sink::SinkExt;
// // use futures_util::stream::SplitSink;
// // use serde_json;
// use tokio::sync::mpsc;
//
// use dto::{Message as DTOMessage, WeatherCondition, WeatherCurrent};
//
// // pub async fn send_state(mut sender: SplitSink<WebSocket, Message>) -> Result<()> {
// //     // TODO: move to shared state
// //     let msg = DTOMessage::WeatherCurrent(WeatherCurrent {
// //         condition: WeatherCondition::Hail,
// //         temp_f: 99,
// //         humidity: 1,
// //     });
// //     let msg = serde_json::to_string(&msg)?;
// //     sender.send(Message::Text(msg)).await?;
// //     Ok(())
// // }
//
// pub async fn send_state(tx: mpsc::Sender<DTOMessage>) -> Result<()> {
//     let msg = DTOMessage::WeatherCurrent(WeatherCurrent {
//         condition: WeatherCondition::Hail,
//         temp_f: 99,
//         humidity: 1,
//     });
//     tx.send(msg).await?;
//     Ok(())
// }

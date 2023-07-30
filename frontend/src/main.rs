use anyhow::Result;
use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage};
use gloo_timers::future::TimeoutFuture;
use log::{debug, error, info};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use components::{precipitation_change, sun_change, Overlay};
use dto::{DisplayState, Message};
use pages::home::Home;
use router::Route;
use stores::{WeatherStore, Widget, WidgetStore};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

fn handle_message(msg: String) -> Result<()> {
    let msg = serde_json::from_str::<Message>(&msg)?;
    match msg {
        Message::Weather(weather) => Dispatch::<WeatherStore>::new().reduce_mut(|s| {
            Dispatch::<WidgetStore>::new().reduce_mut(|s| {
                s.enable(Widget::WeatherCurrent);
                s.enable(Widget::WeatherForecast(0));
                s.enable(Widget::WeatherForecast(1));
                s.enable(Widget::WeatherHumidity);

                if let Some(_) = precipitation_change(&weather) {
                    s.enable(Widget::WeatherPrecipitation);
                } else {
                    s.disable(Widget::WeatherPrecipitation);
                }
            });
            s.weather = weather;
        }),
        Message::TaskReminders(_) => error!("unimplemented"),
        Message::CalendarReminders(_) => error!("unimplemented"),
        Message::Alerts(_) => error!("unimplemented"),
        Message::Sun(sun) => Dispatch::<WeatherStore>::new().reduce_mut(|s| {
            Dispatch::<WidgetStore>::new().reduce_mut(|s| {
                if sun_change(&sun) {
                    s.enable(Widget::Sun);
                } else {
                    s.disable(Widget::Sun);
                }
            });
            s.sun = sun;
        }),
        Message::DisplayStateChange(state) => {
            Dispatch::<WidgetStore>::new().reduce_mut(|s| {
                s.display = match state {
                    DisplayState::On => true,
                    DisplayState::Off => false,
                }
            });
        }
        Message::DisplayBrightness(brightness) => {
            Dispatch::<WidgetStore>::new().reduce_mut(|s| s.opacity = 100 - brightness);
        }
    }
    Ok(())
}

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async move {
        loop {
            let host = window().unwrap().location().host().unwrap();
            let search = window().unwrap().location().search().unwrap();
            let url = if search.contains("dev") {
                format!("ws://localhost:8081/api/ws")
            } else {
                format!("ws://{host}/api/ws")
            };
            let ws = WebSocket::open(&url).unwrap();
            let (_, mut read) = ws.split();

            while let Some(msg) = read.next().await {
                debug!("{msg:?}");
                match msg {
                    Ok(WsMessage::Text(msg)) => {
                        if let Err(e) = handle_message(msg) {
                            error!("failed to handle message: {e}");
                        }
                    }
                    Ok(WsMessage::Bytes(_)) => error!("unexepected bytes message"),
                    Err(e) => error!("websocket error: {e}"),
                }
            }
            info!("websocket closed");
            TimeoutFuture::new(1000).await;
        }
    });
    html! {
        <BrowserRouter>
            <Overlay />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

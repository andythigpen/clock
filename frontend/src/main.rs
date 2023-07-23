use anyhow::Result;
use futures::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage};
use log::{debug, error, info};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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
            s.weather = weather;
            if !s.ready {
                Dispatch::<WidgetStore>::new().reduce_mut(|s| {
                    s.enable(Widget::WeatherCurrent);
                    s.enable(Widget::WeatherForecast(0));
                    s.enable(Widget::WeatherForecast(1));
                    s.enable(Widget::WeatherHumidity);
                });
            }
            s.ready = true;
            debug!("ready");
        }),
        Message::TaskReminders(_) => error!("unimplemented"),
        Message::CalendarReminders(_) => error!("unimplemented"),
        Message::Alerts(_) => error!("unimplemented"),
        Message::Sun(_) => error!("unimplemented"),
        Message::DisplayStateChange(state) => {
            Dispatch::<WidgetStore>::new().reduce_mut(|s| {
                s.display = match state {
                    DisplayState::On => true,
                    DisplayState::Off => false,
                }
            });
        }
    }
    Ok(())
}

#[function_component(App)]
pub fn app() -> Html {
    let host = window().unwrap().location().host().unwrap();
    let ws = WebSocket::open(&format!("ws://{host}/api/ws")).unwrap();
    let (_, mut read) = ws.split();
    spawn_local(async move {
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
    });
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

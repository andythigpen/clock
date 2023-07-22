use futures::StreamExt;
use gloo_net::websocket::futures::WebSocket;
use log::info;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use pages::home::Home;
use router::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let mut ws = WebSocket::open("ws://localhost:8081/api/ws").unwrap();
    let (mut write, mut read) = ws.split();
    spawn_local(async move {
        while let Some(msg) = read.next().await {
            info!("{msg:?}");
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

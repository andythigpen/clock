use axum::{
    extract::connect_info::ConnectInfo,
    extract::ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    extract::TypedHeader,
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use log::{debug, error};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use tokio::sync::mpsc;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        debug!("Pinged {}...", who);
    } else {
        debug!("Could not send ping {}!", who);
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            debug!("client {who} abruptly disconnected");
            return;
        }
    }

    // Since each client gets individual statemachine, we can pause handling
    // when necessary to wait for some external event (in this case illustrated by sleeping).
    // Waiting for this client to finish getting its greetings does not prevent other clients from
    // connecting to server and receiving their greetings.
    // for i in 1..5 {
    //     if socket
    //         .send(Message::Text(format!("Hi {i} times!")))
    //         .await
    //         .is_err()
    //     {
    //         debug!("client {who} abruptly disconnected");
    //         return;
    //     }
    //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    // }

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::channel(32);

    let poller_task = tokio::spawn(async move {
        service::home_assistant::run(tx).await;
    });

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move {
        // if let Err(e) = service::weather_current::send_state(tx).await {
        //     error!("error: {e}");
        // }

        while let Some(msg) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&msg) {
                if let Err(e) = sender.send(Message::Text(msg)).await {
                    error!("error sending: {e}");
                }
            }
        }

        // let n_msg = 20;
        // for i in 0..n_msg {
        //     // In case of any websocket error, we exit.
        //     if sender
        //         .send(Message::Text(format!("Server message {i} ...")))
        //         .await
        //         .is_err()
        //     {
        //         return i;
        //     }
        //
        //     tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        // }
        //
        // debug!("Sending close to {who}...");
        // if let Err(e) = sender
        //     .send(Message::Close(Some(CloseFrame {
        //         code: axum::extract::ws::close_code::NORMAL,
        //         reason: Cow::from("Goodbye"),
        //     })))
        //     .await
        // {
        //     debug!("Could not send Close due to {}, probably it is ok?", e);
        // }
        // n_msg
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => debug!("messages sent to {}", who),
                Err(a) => debug!("Error sending messages {:?}", a)
            }
            recv_task.abort();
            poller_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => debug!("Received {} messages", b),
                Err(b) => debug!("Error receiving messages {:?}", b)
            }
            send_task.abort();
            poller_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    debug!("Websocket context {} destroyed", who);
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            debug!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            debug!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                debug!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                debug!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            debug!(">>> {} sent pong with {:?}", who, v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            debug!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}

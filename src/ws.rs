use crate::{CANData, Client, Clients};
use futures::{FutureExt, StreamExt};
use std::ptr;
use tokio::sync::mpsc;
use tokio::time::*;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, data: CANData, clients: Clients) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().as_simple().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };
    clients.lock().await.insert(uuid.clone(), new_client);
    let mut test = [0; 8];
    {
        let mut qwer = data.lock().unwrap();
        test = (*qwer).Data;
    }
    client_msg(Message::text(format!("{:x?}", test)), &clients).await;

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(msg, &clients).await;
    }
    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(msg: Message, clients: &Clients) {
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    let locked = clients.lock().await;
    for it in locked.values() {
        println!("sending message");
        let _ = it.sender.as_ref().unwrap().send(Ok(Message::text(message)));
    }
    return;
}

use crate::{ws, CANData, Clients, Result};

use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, data: CANData, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, data, clients)))
}

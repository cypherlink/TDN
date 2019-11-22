use std::net::SocketAddr;
use async_std::sync::{channel, Sender, Receiver};
use async_std::io::Result;
use async_std::task;

use crate::{Message, new_channel};

pub(crate) struct P2pServer {
    out_send: Sender<Message>,
}

impl P2pServer {
    pub fn new(out_send: Sender<Message>) -> Self {
        // TODO set group config

        Self { out_send }
    }

    pub async fn start(&self) -> Result<Sender<Message>> {
        let (send, recv) = new_channel();

        // start chamomile

        // start listen self recv

        Ok(send)
    }
}

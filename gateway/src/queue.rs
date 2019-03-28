// Credits to Serenity for this awesome shard queue.

use std::sync::Arc;

use futures::{AsyncSink, Poll, Sink, StartSend, sync::mpsc::{SendError, UnboundedSender}};
use parking_lot::Mutex;
use tokio_tungstenite::tungstenite::{
    Error as TungsteniteError,
    Message as TungsteniteMessage,
};

use crate::Shard;

pub struct MessageSink {
    pub shard: Arc<Mutex<Shard>>,
    pub sender: UnboundedSender<(Arc<Mutex<Shard>>, TungsteniteMessage)>,
}

impl Sink for MessageSink {
    type SinkItem = TungsteniteMessage;
    type SinkError = MessageSinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(match self.sender.start_send((self.shard.clone(), item))? {
            AsyncSink::NotReady((_, item)) => AsyncSink::NotReady(item),
            AsyncSink::Ready => AsyncSink::Ready,
        })
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.sender.poll_complete()
            .map_err(From::from)
    }

    fn close(&mut self) -> Poll<(), Self::SinkError> {
        self.sender.close().map_err(From::from)
    }
}

pub enum MessageSinkError {
    MpscSend(SendError<(Arc<Mutex<Shard>>, TungsteniteMessage)>),
    Tungstenite(TungsteniteError),
}


impl From<SendError<(Arc<Mutex<Shard>>, TungsteniteMessage)>> for MessageSinkError {
    fn from(e: SendError<(Arc<Mutex<Shard>>, TungsteniteMessage)>) -> Self {
        MessageSinkError::MpscSend(e)
    }
}

impl From<TungsteniteError> for MessageSinkError {
    fn from(e: TungsteniteError) -> Self {
        MessageSinkError::Tungstenite(e)
    }
}

impl ::std::fmt::Debug for MessageSinkError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use std::error::Error;

        write!(f, "{}", match *self {
            MessageSinkError::MpscSend(ref err) => err.description(),
            MessageSinkError::Tungstenite(ref err) => err.description(),
        })
    }
}

use std::sync::Arc;

use futures::{AsyncSink, Future, Poll, Sink, StartSend, sync::mpsc::{SendError, UnboundedSender}};
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

pub trait ReconnectQueue {
    type Error: 'static;
    fn push_back(&mut self, shard_id: usize) -> Box<Future<Item = (), Error = Self::Error> + Send>;
    fn pop_front(&mut self) -> Box<Future<Item = Option<usize>, Error = Self::Error> + Send>;
}

/*#[derive(Clone)]
pub struct ShardQueue {
    pub queue: VecDeque<usize>,
}

impl ShardQueue {
    pub fn new(shard_count: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(shard_count)
        }
    }
}

impl ReconnectQueue for ShardQueue {
    type Error = ();
    fn push_back(&mut self, shard_id: usize) -> Box<Future<Item = (), Error = Self::Error> + Send> {
        self.queue.push_back(shard_id);
        Box::new(future::ok(()))
    }

    fn pop_front(&mut self) -> Box<Future<Item = Option<usize>, Error = Self::Error> + Send> {
        Box::new(future::ok(self.queue.pop_front()))
    }
}
*/
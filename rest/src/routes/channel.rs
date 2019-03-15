use futures::future::Future;

use spectacles_model::channel::Channel;
use spectacles_model::message::{CreateMessage, Message};

use crate::Error;
use crate::routes::RouteManager;

/// Routes pertaining to channels in the Discord API.
pub struct ChannelsView {
    pub id: u64,
    pub router: RouteManager,
}

impl ChannelsView {
    pub fn get(&self) -> impl Future<Item=Channel, Error=Error> {
        let route = format!("/channels/{}", self.id);
        self.router.get::<Channel>(route)
    }

    pub fn create_message(&self, payload: CreateMessage) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.post::<Message, CreateMessage>(route, payload)
    }

    pub fn messages(&self) -> impl Future<Item=Vec<Message>, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.get::<Vec<Message>>(route)
    }

    pub fn get_message(&self, mid: u64) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages/{}", self.id, mid);
        self.router.get::<Message>(route)
    }
}
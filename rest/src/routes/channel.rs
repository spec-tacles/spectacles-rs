use futures::future::Future;

use spectacles_model::channel::{Channel, ModifyChannel};
use spectacles_model::message::{CreateMessage, Message, EditMessage};

use crate::Error;
use crate::routes::RouteManager;

/// Routes pertaining to channels in the Discord API.
pub struct ChannelsView {
    pub id: u64,
    // pub messages: ChannelMessagesView,
    pub router: RouteManager,
}

impl ChannelsView {
    /// Fetches a Channel object using the provided ID.
    pub fn get(&self) -> impl Future<Item=Channel, Error=Error> {
        let route = format!("/channels/{}", self.id);
        self.router.get::<Channel>(route)
    }

    /// Creates a message in the current channel.
    pub fn create_message(&self, payload: CreateMessage) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.post::<CreateMessage, Message>(route, payload)
    }

    /// Edits the message in this channel, with the given message ID.
    pub fn edit_message(&self, mid: u64, payload: EditMessage) -> impl Future<Item = Message, Error = Error> {
        let route = format!("/channels/{}/messages/{}", self.id, mid);
        self.router.patch::<EditMessage, Message>(route, payload)
    }

    /// Modifies this channel.
    pub fn modify(&self, payload: ModifyChannel) -> impl Future<Item = Channel, Error = Error> {
        let route = format!("/channels/{}", self.id);
        self.router.patch::<ModifyChannel, Channel>(route, payload)

    }
    /// Fetches all messages in this channel.
    pub fn messages(&self) -> impl Future<Item=Vec<Message>, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.get::<Vec<Message>>(route)
    }

    /// Gets a message in this channel, with the provided ID.
    pub fn get_message(&self, mid: u64) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages/{}", self.id, mid);
        self.router.get::<Message>(route)
    }

    /// Deletes this channel from Discord, or in the case of a direct messages, closes the channel.
    pub fn delete(&self) -> impl Future<Item = Channel, Error = Error> {
        let route = format!("/channels/{}", self.id);
        self.router.delete::<Channel>(route)
    }
}

pub struct ChannelMessagesView {

}
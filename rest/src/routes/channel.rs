use futures::future::Future;

use spectacles_model::channel::{Channel, ModifyChannel};
use spectacles_model::message::{CreateMessage, Message, MessageResponse};
use spectacles_model::snowflake::Snowflake;

use crate::Error;
use crate::routes::RouteManager;

/// Routes pertaining to channels in the Discord API.
pub struct ChannelView {
    pub id: u64,
    pub invites: ChannelInvitesView,
    pub messages: ChannelMessagesView,
    pub permissions: ChannelPermissionsView,
    pub pins: ChannelPinsView,
    pub recipients: ChannelRecipientsView,
    router: RouteManager,
}

impl ChannelView {
    pub fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            invites: ChannelInvitesView::new(id, router.clone()),
            messages: ChannelMessagesView::new(id, router.clone()),
            permissions: ChannelPermissionsView::new(id, router.clone()),
            pins: ChannelPinsView::new(id, router.clone()),
            recipients: ChannelRecipientsView::new(id, router.clone()),
            router,
        }
    }
    /// Creates a message in the current channel.
    pub fn create_message(&self, payload: impl MessageResponse) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.post::<CreateMessage, Message>(route, payload.to_message())
    }

    /// Creates an invite for the current channel.
    pub fn create_invite(&self) {}

    /// Fetches a Channel object using the provided ID.
    pub fn get(&self) -> impl Future<Item=Channel, Error=Error> {
        let route = format!("/channels/{}", self.id);
        self.router.get::<Channel>(route)
    }


    /// Deletes this channel from Discord, or in the case of a direct messages, closes the channel.
    pub fn delete(&self) -> impl Future<Item=Channel, Error=Error> {
        let route = format!("/channels/{}", self.id);
        self.router.delete::<Channel>(route)
    }

    /// Edits the message in this channel, with the given message ID.
    pub fn edit_message(&self, mid: impl Into<u64>, payload: impl MessageResponse) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages/{}", self.id, mid.into());
        self.router.patch::<CreateMessage, Message>(route, payload.to_message())
    }

    /// Modifies this channel.
    pub fn modify(&self, payload: ModifyChannel) -> impl Future<Item = Channel, Error = Error> {
        let route = format!("/channels/{}", self.id);
        self.router.patch::<ModifyChannel, Channel>(route, payload)
    }

    /// Gets the pins in this channel.
    pub fn pins(&self) {

    }
}

/// Methods for interfacing with a channel's invites.
pub struct ChannelInvitesView {
    pub id: u64,
    pub router: RouteManager,
}

impl ChannelInvitesView {
    fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            router,
        }
    }
    /// Creates an invite for this channel.
    pub fn create(&self) {}

    /// Gets all invites for this channel.
    pub fn get_all(&self) {}
}

/// A view for managing a channel's messages.
pub struct ChannelMessagesView {
    pub id: u64,
    router: RouteManager,
}

impl ChannelMessagesView {
    fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            router,
        }
    }

    /// Fetches all messages in this channel.
    pub fn fetch(&self) -> impl Future<Item=Vec<Message>, Error=Error> {
        let route = format!("/channels/{}/messages", self.id);
        self.router.get::<Vec<Message>>(route)
    }

    /// Gets a message in this channel, with the provided ID.
    pub fn get(&self, mid: Snowflake) -> impl Future<Item=Message, Error=Error> {
        let route = format!("/channels/{}/messages/{}", self.id, mid.0);
        self.router.get::<Message>(route)
    }

    /// Deletes a message from this channel.
    pub fn delete(&self, mid: Snowflake) {}

    /// Deletes multiple messages in one request for a single channel.
    pub fn bulk_delete(&self, ids: Vec<Snowflake>) {

    }

    /// A view for managing a message's reactions.
    pub fn reactions(&self, mid: Snowflake) -> ChannelMessageReactionsView {
        ChannelMessageReactionsView {
            id: self.id,
            message_id: mid.0,
            router: self.router.clone(),
        }
    }
}

/// A view for working with a channel's pinned messages.
pub struct ChannelPinsView {
    pub id: u64,
    router: RouteManager,
}

impl ChannelPinsView {
    fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            router,
        }
    }
    /// Gets all of this channel's pins.
    pub fn get(&self) {}

    /// Pins a message to this channel.
    pub fn add(&self, _mid: Snowflake) {}

    /// Deletes a pinned message from this channel.
    pub fn delete(&self, _mid: Snowflake) {}
}

/// A view for working with a a message's reactions.
pub struct ChannelMessageReactionsView {
    pub id: u64,
    pub message_id: u64,
    router: RouteManager,
}

impl ChannelMessageReactionsView {
    fn new(id: u64, message_id: u64, router: RouteManager) -> Self {
        Self {
            id,
            message_id,
            router,
        }
    }
    /// Get a list of users who have reacted to this message.
    pub fn get(&self, id: Snowflake) {}

    pub fn delete_all(&self) {}
}

/// A view for working with a channel's permissions.
pub struct ChannelPermissionsView {
    pub id: u64,
    router: RouteManager,
}

impl ChannelPermissionsView {
    fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            router,
        }
    }
    /// Edit the permissions for this channel.
    pub fn edit(&self) {}

    /// Deletes a permission overwrite for a user or role in this channel.
    pub fn delete(&self) {}
}

/// A view for working with a group DM's recipients.
pub struct ChannelRecipientsView {
    pub id: u64,
    router: RouteManager,
}

impl ChannelRecipientsView {
    fn new(id: u64, router: RouteManager) -> Self {
        Self {
            id,
            router,
        }
    }

    /// Adds a recipient to the group DM, using their access token.
    pub fn add(&self) {}

    /// Removes a recipient from the group DM.
    pub fn remove(&self) {}
}
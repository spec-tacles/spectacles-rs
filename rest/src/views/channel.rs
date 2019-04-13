use futures::future::Future;
use reqwest::Method;
use reqwest::r#async::multipart::{Form, Part};

use spectacles_model::channel::{Channel, ModifyChannelOptions};
use spectacles_model::invite::{CreateInviteOptions, Invite};
use spectacles_model::message::{GetReactionsOptions, Message, MessageResponse};
use spectacles_model::snowflake::Snowflake;
use spectacles_model::User;

use crate::Endpoint;
use crate::Error;
use crate::RestClient;

/// A view for interfacing with a Discord channel.
pub struct ChannelView {
    id: u64,
    /// A view for interfacing with a channel's messages.
    pub messages: ChannelMessagesView,
    client: RestClient,
}

impl ChannelView {
    pub(crate) fn new(id: u64, client: RestClient) -> Self {
        Self {
            id,
            messages: ChannelMessagesView::new(id, client.clone()),
            client,
        }
    }

    /// Creates a message in the current channel.
    /// This endpoint requires the Create Messages permission on Discord.
    pub fn create_message(&self, payload: impl MessageResponse) -> impl Future<Item=Message, Error=Error> {
        let endpt = Endpoint::new(
            Method::POST,
            format!("/channels/{}/messages", self.id),
        );
        let create = payload.as_message();
        let json = serde_json::to_string(&create).expect("Failed to serialize message");

        if let Some((name, file)) = create.file {
            self.client.request(endpt.multipart(
                Form::new()
                    .part("file", Part::bytes(file).file_name(name))
                    .part("payload_json", Part::text(json))
            ))
        } else {
            self.client.request(endpt.json(create))
        }
    }

    /// Returns a view representing the messages in this channel.
    pub fn messages(&self) -> ChannelMessagesView {
        ChannelMessagesView::new(self.id, self.client.clone())
    }

    /// Creates an invite for the current channel.
    /// Requires the Create Invite permission on Discord.
    pub fn create_invite(&self, inv: CreateInviteOptions) -> impl Future<Item=Invite, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/channels/{}/invites", self.id),
            ).json(inv)
        )
    }

    /// Edits a permission overwrite for a given channel.
    pub fn edit_overwrite(&self, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::PUT,
            format!("/channels/{}/permissions/{}", self.id, id.0),
        ))
    }

    /// Deletes a permission overwrite for the current channel.
    pub fn delete_overwrite(&self, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/channels/{}/permissions/{}", self.id, id.0),
        ))
    }

    /// Gets a collection of all invites created for this channel.
    pub fn get_invites(&self) -> impl Future<Item=Vec<Invite>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/channels/{}/invites", self.id),
        ))
    }

    /// Gets a list of all pinned messages in the channel.
    pub fn get_pins(&self) -> impl Future<Item=Vec<Message>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/channels/{}/pins", self.id))
        )
    }

    /// Fetches a Channel object using the provided ID.
    pub fn fetch(&self) -> impl Future<Item=Channel, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/channels/{}", self.id))
        )
    }

    /// Deletes this channel from Discord, or in the case of a direct messages, closes the channel.
    pub fn delete(&self) -> impl Future<Item=Channel, Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/channels/{}", self.id))
        )
    }

    /// Deletes a pinned message from this channel.
    pub fn delete_pin(&self, mid: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/channels/{}/pins/{}", self.id, mid),
        ))
    }

    /// Edits the message in this channel, with the given message ID.
    pub fn edit_message(&self, mid: &Snowflake, payload: impl MessageResponse) -> impl Future<Item=Message, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/channels/{}/messages/{}", self.id, mid),
            ).json(payload.as_message())
        )
    }

    /// Modifies this channel.
    pub fn modify(&self, payload: ModifyChannelOptions) -> impl Future<Item=Channel, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/channels/{}", self.id),
            ).json(payload)
        )
    }

    /// Adds a pinned message to this channel.
    pub fn pin_message(&self, mid: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::PUT,
            format!("/channels/{}/pins/{}", self.id, mid),
        ))
    }
}

/// A view for managing a channel's messages.
pub struct ChannelMessagesView {
    id: u64,
    client: RestClient,
}

impl ChannelMessagesView {
    fn new(id: u64, client: RestClient) -> Self {
        Self {
            id,
            client,
        }
    }

    /// Fetches all messages in this channel.
    pub fn get_all(&self) -> impl Future<Item=Vec<Message>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/channels/{}/messages", self.id),
        ))
    }

    /// Gets a single message in this channel, with the provided ID.
    pub fn get(&self, mid: &Snowflake) -> impl Future<Item=Message, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/channels/{}/messages/{}", self.id, mid.0),
        ))
    }

    /// Deletes a message from this channel.
    pub fn delete(&self, mid: Snowflake) -> impl Future<Item=Message, Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/channels/{}/messages/{}", self.id, mid.0),
        ))
    }

    /// Deletes multiple messages in one request for a single channel.
    pub fn bulk_delete(&self, ids: Vec<Snowflake>) -> impl Future<Item=(), Error=Error> {
        let json = json!({
            "messages": ids
        });

        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/channels/{}/messages/bulk-delete", self.id),
            ).json(json)
        )
    }

    /// A view for managing a message's reactions.
    pub fn reactions(&self, mid: Snowflake) -> ChannelMessageReactionsView {
        ChannelMessageReactionsView::new(self.id, mid.0, self.client.clone())
    }
}

/// A view for working with a a message's reactions.
pub struct ChannelMessageReactionsView {
    id: u64,
    message_id: u64,
    client: RestClient,
}

impl ChannelMessageReactionsView {
    fn new(id: u64, message_id: u64, client: RestClient) -> Self {
        Self {
            id,
            message_id,
            client,
        }
    }
    /// Get a list of users who have reacted to this message with the provided emoji.
    pub fn get(&self, id: &Snowflake, opts: GetReactionsOptions) -> impl Future<Item=Vec<User>, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::GET,
                format!("/channels/{}/messages/{}/reactions/{}", self.id, self.message_id, id.0),
            ).query(opts)
        )
    }

    pub fn delete_all(&self) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/channels/{}/messages/{}/reactions", self.id, self.message_id),
        ))
    }
}
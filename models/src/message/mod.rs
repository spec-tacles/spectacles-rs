//! Structs related to Discord messages in a guild channel.
use chrono::{DateTime, FixedOffset};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::guild::GuildMember;
use crate::snowflake::Snowflake;
use crate::User;

pub use self::embed::*;
pub use self::emoji::*;
pub use self::webhook::Webhook;

mod embed;
mod webhook;
mod emoji;

/// Represents different types that can be sent to the Discord API.
pub trait MessageResponse {
    fn to_message(self) -> CreateMessage;
}

impl MessageResponse for &str {
    fn to_message(self) -> CreateMessage {
        CreateMessage::default().with_content(self)
    }
}

impl MessageResponse for String {
    fn to_message(self) -> CreateMessage {
        CreateMessage::default().with_content(self)
    }
}

impl MessageResponse for EditMessage {
    fn to_message(self) -> CreateMessage {
        let m = CreateMessage::default();
        let m = m.clone().with_content(self.content.unwrap_or_default());

        if let Some(e) = self.embed {
            m.with_embed(e)
        } else {
            m
        }
    }
}

/// A message sent in a channel on Discord.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    /// The message ID of the message.
    pub id: Snowflake,
    /// The ID of the channel that the message was sent in.
    pub channel_id: Snowflake,
    /// The ID of the guild that the message was sent in.
    #[serde(default)]
    pub guild_id: Snowflake,
    /// The author of the message.
    pub author: User,
    /// The contents of this message.
    pub content: String,
    /// The guild member form of the message author.
    #[serde(default)]
    pub member: GuildMember,
    /// The time that this message was sent.
    pub timestamp: DateTime<FixedOffset>,
    /// When this message was edited, if applicable.
    pub edited_timestamp: Option<DateTime<FixedOffset>>,
    /// Whether or not this was a TTS message.
    pub tts: bool,
    /// Whether or not this message mentioned everyone.
    pub mention_everyone: bool,
    /// Roles that were mentioned in this message.
    pub mention_roles: Vec<Snowflake>,
    /// The message's attached files, if any.
    pub attachments: Vec<MessageAttachment>,
    /// Any embeds sent with this message.
    pub embeds: Vec<Embed>,
    /// The message's reactions.
    #[serde(default)]
    pub reactions: Vec<MessageReaction>,
    /// A snowflake used to validate that a message was sent.
    #[serde(default)]
    pub nonce: Option<Snowflake>,
    /// Whether or not the message is pinned.
    pub pinned: bool,
    /// The ID of the webhook if the message was sent by a webhook.
    #[serde(default)]
    pub webhook_id: Snowflake,
    /// The type of message sent.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// Message Activity sent with rich-presence embeds.
    #[serde(default)]
    pub activity: Option<MessageActivity>,
    /// Message Application ent with Rich Presence embeds.
    #[serde(default)]
    pub application: Option<MessageApplication>,
}

/// Represents a message that is being sent to Discord.
#[derive(Serialize, Clone, Debug, Default)]
pub struct CreateMessage {
    /// The content of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// The embed that this message has.
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Embed>,
    /// Whether or not this message is a TTS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
}

impl CreateMessage {
    /// Creates a new message with the specified content string.
    pub fn new() -> Self {
        CreateMessage {
            content: None,
            embed: None,
            tts: None
        }
    }

    /// Adds content to the message.
    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = Some(content.to_string());

        self
    }

    /// Adds an Embed object to the message.
    pub fn with_embed(mut self, embed: Embed) -> Self {
        self.embed = Some(embed);

        self
    }

    /// Whether or not this message will be a TTS message.
    pub fn tts(mut self, opt: bool) -> Self {
        self.tts = Some(opt);

        self
    }
}

/// Represents a message that is being edited in a Discord channel.
#[derive(Serialize, Clone, Debug, Default)]
pub struct EditMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Embed>
}

impl EditMessage {
    pub fn new() -> EditMessage {
        EditMessage {
            content: None,
            embed: None
        }
    }

    /// Adds the content to edit into this message.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());

        self
    }


    /// Adds an embed to be edited into this message.
    pub fn with_embed(mut self, embed: Embed) -> Self {
        self.embed = Some(embed);

        self
    }
}

/// Options for retrieving messages from a channel.
#[derive(Serialize, Clone, Debug)]
pub struct ChannelMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    around: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Snowflake>,
    limit: i32
}

impl ChannelMessagesQuery {
    pub fn new() -> ChannelMessagesQuery {
        ChannelMessagesQuery {
            around: None,
            before: None,
            after: None,
            limit: 50
        }
    }


    /// Fetch messages from this channel that are around this message ID.
    pub fn around(mut self, id: u64) -> Self {
        if self.after.is_some() || self.before.is_some() {
            return self;
        };
        self.around = Some(id.into());
        self
    }

    /// Fetch messages from this channel that are before this message ID.
    pub fn before(mut self, id: u64) -> Self {
        if self.around.is_some() || self.after.is_some() {
            return self;
        };
        self.before = Some(id.into());
        self
    }

    /// Fetch messages in this channels that are after this message ID.
    pub fn after(mut self, id: u64) -> Self {
        if self.around.is_some() || self.before.is_some() {
            return self;
        };
        self.after = Some(id.into());
        self
    }
}


/// Represents an attachment sent by a user.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageAttachment {
    /// The attachment ID.
    pub id: Snowflake,
    /// The name of the file attached.
    pub filename: String,
    /// The size of the file in bytes.
    pub size: i32,
    /// The source URL of the file.
    pub url: String,
    /// A proxied URL of the file.
    pub proxy_url: String,
    /// The height of the file, if it is an image.
    pub height: Option<i32>,
    /// The width of the file, if it is an image.
    pub width: Option<i32>

}
/// A Rich Presence Message activity.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MessageActivity {
    /// The type of message activity.
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    /// The party ID from a Rich Presence event.
    #[serde(default)]
    pub party_id: String
}

/// A Rich Presence Message Application.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MessageApplication {
    /// The ID of the application.
    pub id: Snowflake,
    /// The ID of the embeds's image.
    pub cover_image: String,
    /// The application description.
    pub description: String,
    /// The ID of the application icon.
    pub icon: String,
    /// The name of the application.
    pub name: String
}

/// A list of Message types.
#[derive(Deserialize_repr, Debug, Clone, Serialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    GuildMemberJoin
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Default
    }
}

/// A list of Message Activity types.
#[derive(Deserialize, Serialize, Debug, Clone)]

pub enum MessageActivityType {
    Join = 1,
    Spectate,
    Listen = 3,
    JoinRequest = 5
}

impl Default for MessageActivityType {
    fn default() -> Self {
        MessageActivityType::Join
    }
}

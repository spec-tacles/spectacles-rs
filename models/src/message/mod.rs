//! Structs related to Discord messages in a guild channel.
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::guild::GuildMember;
use crate::User;

pub use self::embed::*;
pub use self::emoji::*;
pub use self::webhook::Webhook;

mod embed;
mod webhook;
mod emoji;

/// A message sent in a channel on Discord.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    /// The message ID of the message.
    pub id: String,
    /// The ID of the channel that the message was sent in.
    pub channel_id: String,
    /// The ID of the guild that the message was sent in.
    #[serde(default)]
    pub guild_id: String,
    /// The author of the message.
    pub author: User,
    /// The contents of this message.
    pub content: String,
    /// The guild member form of the message author.
    #[serde(default)]
    pub member: GuildMember,
    /// The time that this message was sent.
    pub timestamp: String,
    /// When this message was edited, if applicable.
    pub edited_timestamp: Option<String>,
    /// Whether or not this was a TTS message.
    pub tts: bool,
    /// Whether or not this message mentioned everyone.
    pub mention_everyone: bool,
    /// Roles that were mentioned in this message.
    pub mention_roles: Vec<String>,
    /// The message's attached files, if any.
    pub attachments: Vec<MessageAttachment>,
    /// Any embeds sent with this message.
    pub embeds: Vec<Embed>,
    /// The message's reactions.
    #[serde(default)]
    pub reactions: Vec<MessageReaction>,
    /// A snowflake used to validate that a message was sent.
    #[serde(default)]
    pub nonce: Option<String>,
    /// Whether or not the message is pinned.
    pub pinned: bool,
    /// The ID of the webhook if the message was sent by a webhook.
    #[serde(default)]
    pub webhook_id: String,
    /// The type of message sent.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// Message Activity sent with rich-presence embeds.
    #[serde(default)]
    pub activity: MessageActivity,
    /// Message Application ent with Rich Presence embeds.
    #[serde(default)]
    pub application: MessageApplication,
}

/// Represents an attachment sent by a user.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageAttachment {
    /// The attachment ID.
    pub id: String,
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
    pub id: String,
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

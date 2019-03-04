//! Structs related to Discord messages in a guild channel.
use chrono::{DateTime, FixedOffset};

use crate::{parse_snowflake, parse_snowflake_array, User};
use crate::guild::GuildMember;

pub use self::embed::*;
pub use self::emoji::*;

mod embed;
mod emoji;

/// A message sent in a channel on Discord.
#[derive(Deserialize, Clone, Debug)]
pub struct Message {
    /// The message ID of the message.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// The ID of the channel that the message was sent in.
    #[serde(deserialize_with = "parse_snowflake")]
    pub channel_id: u64,
    /// The ID of the guild that the message was sent in.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
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
    #[serde(deserialize_with = "parse_snowflake_array")]
    pub mention_roles: Vec<u64>,
    /// The message's attached files, if any.
    pub attachments: Vec<u64>, // TODO: Type Attachment struct
    /// Any embeds sent with this message.
    pub embeds: Vec<u64>, // TODO: Type Embed struct
    /// The message's reactions.
    pub reactions: Vec<u64>, // TODO: Type MessageReaction struct,
    /// A snowflake used to validate that a message was sent.
    #[serde(default, deserialize_with = "parse_snowflake")]
    pub nonce: u64,
    /// Whether or not the message is pinned.
    pub pinned: bool,
    /// The ID of the webhook if the message was sent by a webhook.
    #[serde(default, deserialize_with = "parse_snowflake")]
    pub webhook_id: u64,
    /// The type of message sent.
    pub r#type: MessageType,
    /// Message Activity sent with rich-presence embeds.
    #[serde(default)]
    pub activity: MessageActivity,
    /// Message Application ent with Rich Presence embeds.
    #[serde(default)]
    pub application: MessageApplication,
}

/// A Rich Presence Message activity.
#[derive(Deserialize, Clone, Debug)]
pub struct MessageActivity {
    /// The type of message activity.
    pub r#type: MessageActivityType,
    /// The party ID from a Rich Presence event.
    #[serde(default)]
    pub party_id: String
}

/// A Rich Presence Message Application.
pub struct MessageApplication {
    /// The ID of the application.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
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
#[derive(Deserialize, Debug, Clone)]
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

/// A list of Message Activity types.
#[derive(Deserialize, Debug, Clone)]
pub enum MessageActivityType {
    Join = 1,
    Spectate,
    Listen = 3,
    JoinRequest = 5
}

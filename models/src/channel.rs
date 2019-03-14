//! Structures related to a Channel on Discord.
use chrono::{DateTime, FixedOffset};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{User, Snowflake};
/// A guild or DM channel on Discord.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Channel {
    /// The channel ID of this channel.
    pub id: String,
    /// The type of channel.
    #[serde(rename = "type")]
    pub kind: Option<ChannelType>,
    /// The guild ID of this channel.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// The position of this channel.
    #[serde(default)]
    pub position: i32,
    /// The explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: PermissionOverwrites,
    /// The name of the channel.
    #[serde(default)]
    pub name: String,
    /// The topic of this channel.
    #[serde(default)]
    pub topic: Option<String>,
    /// Whether or not this channel is an NSFW channel.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent in this channel.
    #[serde(default)]
    pub last_message_id: Option<Snowflake>,
    /// The bitrate of this channel.
    #[serde(default)]
    pub bitrate: i32,
    /// The user limit, if voice.
    #[serde(default)]
    pub user_limit: i32,
    /// The cooldown between sending messages in this channel, in seconds.
    #[serde(default)]
    pub rate_limit_per_user: i32,
    /// The recepients, if DM.
    #[serde(default)]
    pub recipients: Vec<User>,
    /// The channel's icon hash if any.
    #[serde(default)]
    pub icon: Option<String>,
    /// The ID of the creator, if a DM.
    #[serde(default)]
    pub owner_id: String,
    /// The application ID, if the channel was created by a bot.
    #[serde(default)]
    pub application_id: String,
    /// The ID of the parent category.
    #[serde(default)]
    pub parent_id: Option<String>,
    /// When the last message was pinned.
    #[serde(default)]
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>
}

/// A channel permission overwrite.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionOverwrites {
    /// The ID of the role or user.
    pub id: String,
    /// What this ID is for.
    #[serde(rename = "type")]
    pub kind: String,
    /// The allowed permission bitfield.
    pub allow: i32,
    /// The denied permissions bitfield.
    pub deny: i32
}

/// Represents the possible Channel types,
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum ChannelType {
    Text,
    DM,
    Voice,
    GroupDM,
    Category
}
//! Structures related to a Channel on Discord.
use chrono::{DateTime, FixedOffset};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::User;

/// A guild or DM channel on Discord.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    /// The channel ID of this channel.
    pub id: String,
    /// The type of channel.
    #[serde(rename = "type")]
    pub kind: Option<ChannelType>,
    /// The guild ID of this channel.
    pub guild_id: Option<String>,
    /// The position of this channel.
    pub position: Option<i32>,
    /// The explicit permission overwrites for members and roles.
    pub permission_overwrites: PermissionOverwrites,
    /// The name of the channel.
    pub name: Option<String>,
    /// The topic of this channel.
    pub topic: Option<String>,
    /// Whether or not this channel is an NSFW channel.
    pub nsfw: bool,
    /// The ID of the last message sent in this channel.
    pub last_message_id: Option<String>,
    /// The bitrate of this channel.
    pub bitrate: Option<i32>,
    /// The user limit, if voice.
    pub user_limit: Option<i32>,
    /// The cooldown between sending messages in this channel, in seconds.
    pub rate_limit_per_user: Option<i32>,
    /// The recepients, if DM.
    pub recipients: Option<User>,
    /// The channel's icon hash if any.
    pub icon: Option<String>,
    /// The ID of the creator, if a DM.
    pub owner_id: Option<String>,
    /// The application ID, if the channel was created by a bot.
    pub application_id: Option<String>,
    /// The ID of the parent category.
    pub parent_id: Option<String>,
    /// When the last message was pinned.
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>
}

/// A channel permission overwrite.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
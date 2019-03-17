//! Structures related to a Channel on Discord.
use chrono::{DateTime, FixedOffset};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{Snowflake, User};

/// A guild or DM channel on Discord.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Channel {
    /// The channel ID of this channel.
    pub id: Snowflake,
    /// The type of channel.
    #[serde(rename = "type")]
    pub kind: Option<ChannelType>,
    /// The guild ID of this channel.
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// The position of this channel.
    #[serde(default)]
    pub position: Option<i32>,
    /// The explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Option<Vec<PermissionOverwrites>>,
    /// The name of the channel.
    #[serde(default)]
    pub name: Option<String>,
    /// The topic of this channel.
    #[serde(default)]
    pub topic: Option<String>,
    /// Whether or not this channel is an NSFW channel.
    #[serde(default)]
    pub nsfw: Option<bool>,
    /// The ID of the last message sent in this channel.
    #[serde(default)]
    pub last_message_id: Option<Snowflake>,
    /// The bitrate of this channel.
    #[serde(default)]
    pub bitrate: Option<i32>,
    /// The user limit, if voice.
    #[serde(default)]
    pub user_limit: Option<i32>,
    /// The cooldown between sending messages in this channel, in seconds.
    #[serde(default)]
    pub rate_limit_per_user: Option<i32>,
    /// The recepients, if DM.
    #[serde(default)]
    pub recipients: Option<Vec<User>>,
    /// The channel's icon hash if any.
    #[serde(default)]
    pub icon: Option<String>,
    /// The ID of the creator, if a DM.
    #[serde(default)]
    pub owner_id: Option<Snowflake>,
    /// The application ID, if the channel was created by a bot.
    #[serde(default)]
    pub application_id: Option<Snowflake>,
    /// The ID of the parent category.
    #[serde(default)]
    pub parent_id: Option<Snowflake>,
    /// When the last message was pinned.
    #[serde(default)]
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>
}

/// Options for modifying a Discord channel.
#[derive(Serialize, Clone, Debug, Default)]
pub struct ModifyChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<i32>,
    /*#[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwrites>>,*/
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Snowflake>
}

impl ModifyChannel {
    pub fn new() -> Self {
        ModifyChannel::default()
    }

    /// Sets a new name for this channel.
    pub fn name(mut self, new_name: &str) -> Self {
        self.name = Some(new_name.to_string());
        self
    }

    /// Sets a new position for this channel.
    pub fn set_position(mut self, pos: i32) -> Self {
        self.position = Some(pos);
        self
    }

    /// Sets a new topic for this channel.
    pub fn topic(mut self, top: &str) -> Self {
        self.topic = Some(top.to_string());
        self
    }

    /// Changes the NSFW flag for this channel.
    pub fn nsfw(mut self, opt: bool) -> Self {
        self.nsfw = Some(opt);
        self
    }

    /// Modifies this channel's message rate limit per user.
    pub fn rate_limit_per_user(mut self, secs: i8) -> Self {
        self.rate_limit_per_user = Some(secs);
        self
    }

    /// Modifies this channel's user limit, if a voice channel.
    pub fn user_limit(mut self, limit: i32) -> Self {
        self.user_limit = Some(limit);
        self
    }

    /*
    /// Modifies this channel's permission overwrites.
    pub fn overwrites(mut self, ows: Vec<PermissionOverwrites>) -> Self {
        self.permission_overwrites = Some(ows);
        self
    }
    */

    /// Modifies this channel's parent category ID.
    pub fn parent_id(mut self, id: u64) -> Self {
        self.parent_id = Some(id.into());
        self
    }


}
/// A channel permission overwrite.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PermissionOverwrites {
    /// The ID of the role or user.
    pub id: Snowflake,
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

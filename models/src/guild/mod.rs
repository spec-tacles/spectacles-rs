//! Structures related to Discord guilds.
use chrono::{DateTime, FixedOffset};

use crate::{
    message::Emoji,
    User,
};

use super::parse_snowflake;

pub use self::{
    member::GuildMember,
    role::Role,
};

mod role;
mod member;

/// A Discord Guild, commonly referred to as a "server".
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Guild {
    /// The snowflake ID of this guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// The name of the guild.
    pub name: String,
    /// The guild's icon hash. Will be a None value if one is not set.
    pub icon: Option<String>,
    /// The guild's splash hash. Will be a None value if it does not exist.
    pub splash: Option<String>,
    /// The ID of the guild owner.
    pub owner_id: u64,
    /// The permissions that the user has in this guild.
    pub permissions: i32,
    /// The region in which this guild is located.
    pub region: String,
    /// The amount of members that are currently in this guild.
    pub member_count: String,
    /// A list of features that this guild currently has.
    pub features: Vec<String>,
    /// A collection of roles that belong to this guild.
    pub roles: Vec<Role>,
    /// A collection of emotes that belong to this guild.
    pub emojis: Vec<u64>, // TODO: Define Emoji struct
    /// The explicit content filter level for this guild.
    pub explicit_content_filter: ExplicitContentFilter,
    /// The AFK channel ID for this guild.
    #[serde(default)]
    pub afk_channel_id: Option<String>,
    /// The AFK channel timeout for this guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub afk_timeout: u64,
    /// The ID of the application which created the guild, if applicable.
    #[serde(default)]
    pub application_id: Option<u64>,
    /// The verification level, which determines which users can chat in a guild.
    pub verification_level: VerificationLevel,
    /// The MFA authentication level for this guild.
    pub mfa_level: MfaLevel,
    /// Whether or not the guild can be embedded in a widget.
    pub embed_enabled: bool,
    /// The channel ID that an embed widget will be generated for.
    #[serde(deserialize_with = "parse_snowflake")]
    pub embed_channel_id: u64,
    /// The ID of the channel in which system messages are sent to.
    #[serde(default)]
    pub system_channel_id: Option<u64>,
    /// The time that this guild was joined.
    pub joined_at: DateTime<FixedOffset>,
    /// Whether this guild is considered a large guild by Discord.
    pub large: bool,
    /// Whether or not this guild is available.
    pub unavailable: bool,
    /// Whether or not the server widget is enabled.
    pub widget_enabled: bool,
    /// The ID of the guild's widget channel, if one exists.
    #[serde(default)]
    pub widget_channel_id: Option<u64>,
    /// The default message notification setting for this guild.
    pub default_message_notifications: DefaultMessageNotifications,
    /// A collection of guild voice states.
    pub voice_states: Vec<u64>, // TODO: Add Voice State struct
    /// A collection of channels in this guild.
    pub channels: Vec<u64>, // TODO: Add channel struct
    /// A collection of presences in this guild.
    pub presences: Vec<u64> // TODO: Add Presence struct
}

/// A Partial guild object, usually an offline guild.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnavailableGuild {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// Whether or not the guild is available, usually set to true.
    pub unavailable: bool
}

/// Represents a packet received when a user is banned from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildBanAdd {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The user who was banned.
    pub user: User
}

/// Represents a packet received when a user is unbanned from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildBanRemove {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The user who was unbanned.
    pub user: User
}

/// Represents a packet received when a guild's emojis have been updated.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildEmojisUpdate {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// An array of Emoji objects.
    pub emojis: Vec<Emoji>,
}

/// Represents a packet sent when a guild integration is updated.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildIntegrationsUpdate {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64
}

/// Represents a packet sent when a user is removed from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildMemberRemove {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The user who was removed.
    pub user: User
}

/// Represents a packet sent when a guild member is updated.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberUpdate {
    /// The ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    pub roles: Vec<String>,
    /// The user who was updated.
    pub user: User,
    /// The nickname of the user in the guild.
    pub nick: String
}

/// Represents a response to a Guild Request Members packet.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildMembersChunk {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// An array of guild member objects.
    pub members: Vec<GuildMember>
}

/// Represents a packet sent when a role is created ina  guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleCreate {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The newly created role.
    pub role: Role
}

/// Represents a packet sent when a role is created in a guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleUpdate {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The updated role.
    pub role: Role
}

/// Represents a packet sent when a role is created in a guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleDelete {
    /// The guild ID of the guild.
    #[serde(deserialize_with = "parse_snowflake")]
    pub guild_id: u64,
    /// The ID of the deleted role.
    #[serde(deserialize_with = "parse_snowflake")]
    pub role: u64
}

/// A guild's explicit content filter levels.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum ExplicitContentFilter {
    /// The filter is not active.
    Disabled,
    /// The filter is only active for members without roles.
    MembersWithoutRoles,
    /// The filter is active for all members.
    AllMembers
}

/// A guild's MFA levels.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum MfaLevel {
    /// The guild does not require MFA for elevated actions.
    None,
    /// The guild requires MFA on a user account which has elevated permissions.
    Elevated,
}
/// A guild's default message notification setting.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum DefaultMessageNotifications {
    /// A user will be notified whenever a new message is sent in the guild.
    AllMessages,
    /// A user will only be notified when they are mentioned.
    OnlyMentions
}

/// A guild's verification levels.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum VerificationLevel {
    /// The guild is unrestricted.
    None,
    /// The guild requires a verified email on the user's account.
    Low,
    /// The guild requires that the user be registered on Discord for longer than 5 minutes.
    Medium,
    /// The guild requires that the user be on the guild for longer than 10 minutes.
    High,
    /// The guild requires that the user have a verified phone number on their account.
    Insane
}

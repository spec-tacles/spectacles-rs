//! Structures related to Discord guilds.
use chrono::{DateTime, FixedOffset};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    channel::Channel,
    message::Emoji,
    presence::Presence,
    User,
    voice::VoiceState,
};
use crate::snowflake::Snowflake;

pub use self::{
    member::*,
    role::*,
};

mod role;
mod audit_log;
mod member;

/// A Discord Guild, commonly referred to as a "server".
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Guild {
    /// The snowflake ID of this guild.
    pub id: Snowflake,
    /// The name of the guild.
    pub name: String,
    /// The guild's icon hash. Will be a None value if one is not set.
    pub icon: Option<String>,
    /// The guild's splash hash. Will be a None value if it does not exist.
    pub splash: Option<String>,
    /// Whether or not the user is an owner of the guild.
    pub owner: Option<bool>,
    /// The ID of the guild owner.
    pub owner_id: String,
    /// The permissions that the user has in this guild.
    #[serde(default)]
    pub permissions: i32,
    /// The region in which this guild is located.
    pub region: String,
    /// The AFK channel ID for this guild.
    pub afk_channel_id: Option<String>,
    /// The AFK channel timeout for this guild.
    pub afk_timeout: Option<i32>,
    /// Whether or not the guild can be embedded in a widget.
    #[serde(default)]
    pub embed_enabled: bool,
    /// The channel ID that an embed widget will be generated for.
    #[serde(default)]
    pub embed_channel_id: String,
    /// The amount of members that are currently in this guild.
    #[serde(default)]
    pub member_count: i32,
    /// A list of features that this guild currently has.
    pub features: Vec<String>,
    /// A collection of roles that belong to this guild.
    pub roles: Vec<Role>,
    /// A collection of emotes that belong to this guild.
    pub emojis: Vec<Emoji>,
    /// The explicit content filter level for this guild.
    pub explicit_content_filter: ExplicitContentFilter,
    /// The ID of the application which created the guild, if applicable.
    pub application_id: Option<String>,
    /// The verification level, which determines which users can chat in a guild.
    pub verification_level: VerificationLevel,
    /// The MFA authentication level for this guild.
    pub mfa_level: MfaLevel,
    /// The ID of the channel in which system messages are sent to.
    pub system_channel_id: Option<String>,
    /// The time that this guild was joined.
    #[serde(default)]
    pub joined_at: String,
    /// Whether this guild is considered a large guild by Discord.
    #[serde(default)]
    pub large: bool,
    /// Whether or not this guild is available.
    #[serde(default)]
    pub unavailable: bool,
    /// Whether or not the server widget is enabled.
    #[serde(default)]
    pub widget_enabled: bool,
    /// The ID of the guild's widget channel, if one exists.
    #[serde(default)]
    pub widget_channel_id: String,
    /// The default message notification setting for this guild.
    pub default_message_notifications: DefaultMessageNotifications,
    /// A collection of guild voice states.
    pub voice_states: Vec<VoiceState>,
    /// A collection of channels in this guild.
    #[serde(default)]
    pub channels: Vec<Channel>,
    /// A collection of members in this guild.
    #[serde(default)]
    pub members: Vec<GuildMember>,
    /// A collection of presences in this guild.
    #[serde(default)]
    pub presences: Option<Vec<Presence>>
}

/// A Partial guild object, usually an offline guild.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnavailableGuild {
    /// The guild ID of the guild.
    pub id: Snowflake,
    /// Whether or not the guild is available, usually set to true.
    pub unavailable: bool
}

/// The embed object of a guild.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildEmbed {
    /// Whether the embed is enabled.
    pub enabled: bool,
    /// The channel ID of the embed.
    pub channel_id: Snowflake,
}

/// Options for modifying a guild embed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifyGuildEmbedOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Snowflake>,
}

impl ModifyGuildEmbedOptions {
    /// Sets the enabled status of this embed.
    pub fn enabled(mut self, opt: bool) -> Self {
        self.enabled = Some(opt);
        self
    }

    /// Sets the channel ID of the guild embed.
    pub fn channel_id(mut self, id: Snowflake) -> Self {
        self.channel_id = Some(id);
        self
    }
}

/// A body that can be sent to the Modify Guild endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifyGuildOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<VerificationLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<DefaultMessageNotifications>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<ExplicitContentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_channel_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_id: Option<Snowflake>,
}

impl ModifyGuildOptions {
    /// Sets a new name for this guild.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets a new region for this guild.
    pub fn region(mut self, reg: &str) -> Self {
        self.region = Some(reg.to_string());
        self
    }

    /// Sets a new verification level for this guild.
    pub fn verification_level(mut self, level: VerificationLevel) -> Self {
        self.verification_level = Some(level);
        self
    }

    /// Sets the default message notifications level for this guild.
    pub fn default_message_notifications(mut self, level: DefaultMessageNotifications) -> Self {
        self.default_message_notifications = Some(level);
        self
    }

    /// Sets the explicit content filter for this guild.
    pub fn explicit_content_filter(mut self, filter: ExplicitContentFilter) -> Self {
        self.explicit_content_filter = Some(filter);
        self
    }

    /// Sets the AFK channel for this guild.
    pub fn afk_channel(mut self, id: Snowflake) -> Self {
        self.afk_channel_id = Some(id);
        self
    }

    /// Sets the AFK timeout for this guild.
    pub fn afk_timeout(mut self, timeout: i32) -> Self {
        self.afk_timeout = Some(timeout);
        self
    }

    /// Sets the icon of this guild.
    pub fn icon(mut self, url: &str) -> Self {
        self.icon = Some(url.to_string());
        self
    }

    /// Sets a new owner for this guild.
    pub fn owner(mut self, id: Snowflake) -> Self {
        self.owner = Some(id);
        self
    }

    /// Sets the splash icon for this guild.
    pub fn splash(mut self, url: &str) -> Self {
        self.splash = Some(url.to_string());
        self
    }

    /// Sets the system channel of this guild.
    pub fn system_channel(mut self, chan: Snowflake) -> Self {
        self.system_channel_id = Some(chan);
        self
    }
}

/// Information about a guild's prune status.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildPrune {
    /// The number of members that have been pruned.
    pub pruned: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct GuildIntegration {
    /// The snowflake ID of this integration.
    pub id: Snowflake,
    /// The name of this integration.
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    /// Whether or not this integration is enabled.
    pub enabled: bool,
    /// Whether or not this integration is syncing.
    pub syncing: bool,
    /// The "subscribers" role for this integration.
    pub role_id: Snowflake,
    /// The behavior of expiring subscribers.
    pub expire_behavior: i32,
    /// The grace period before expiring subscribers.
    pub expire_grace_period: i32,
    /// The user of this integration.
    pub user: User,
    /// The integration account information.
    pub account: IntegrationAccount,
    /// When this integration was last synced.
    pub synced_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IntegrationAccount {
    /// The ID of the account.
    pub id: String,
    /// The name of the account.
    pub name: String,
}

/// Options for modifying a guild integration.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ModifyGuildIntegrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_behavior: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_grace_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_emoticons: Option<bool>,
}

impl ModifyGuildIntegrationOptions {
    /// Sets the new expire behavior for this integration.
    pub fn expire_behavior(mut self, beh: i32) -> Self {
        self.expire_behavior = Some(beh);
        self
    }

    /// Sets a new expire grace period for this integration.
    pub fn expire_grace_period(mut self, per: i32) -> Self {
        self.expire_grace_period = Some(per);
        self
    }

    /// Sets whether emoticons should be synced for this integration.
    pub fn enable_emoticons(mut self, opt: bool) -> Self {
        self.enable_emoticons = Some(opt);
        self
    }
}

/// A guild ban object.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildBan {
    /// The reason for the ban, if applicable.
    pub reason: Option<String>,
    /// The user who was banned.
    pub user: User,
}

/// Represents a packet received when a user is banned from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildBanAdd {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The user who was banned.
    pub user: User
}

/// Represents a packet received when a user is unbanned from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildBanRemove {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The user who was unbanned.
    pub user: User
}

/// Represents a packet received when a guild's emojis have been updated.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildEmojisUpdate {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// An array of Emoji objects.
    pub emojis: Vec<Emoji>,
}

/// Represents a packet sent when a guild integration is updated.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildIntegrationsUpdate {
    /// The guild ID of the guild.
    pub guild_id: Snowflake
}

/// Represents a packet sent when a user is removed from a guild.
#[derive(Deserialize, Debug, Clone)]
pub struct GuildMemberRemove {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The user who was removed.
    pub user: User
}

/// Represents a packet sent when a guild member is updated.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildMemberUpdate {
    /// The ID of the guild.
    pub guild_id: Snowflake,
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
    pub guild_id: Snowflake,
    /// An array of guild member objects.
    pub members: Vec<GuildMember>
}

/// Represents a packet sent when a role is created ina  guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleCreate {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The newly created role.
    pub role: Role
}

/// Represents a packet sent when a role is created in a guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleUpdate {
    /// The guild ID of the guild.
    pub guild_id: String,
    /// The updated role.
    pub role: Role
}

/// Represents a packet sent when a role is created in a guild.
#[derive(Deserialize, Clone, Debug)]
pub struct GuildRoleDelete {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The ID of the deleted role.
    pub role: Snowflake
}

/// A guild's explicit content filter levels.
#[derive(Deserialize_repr, Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    /// The filter is not active.
    Disabled,
    /// The filter is only active for members without roles.
    MembersWithoutRoles,
    /// The filter is active for all members.
    AllMembers
}

/// A guild's MFA levels.
#[derive(Deserialize_repr, Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum MfaLevel {
    /// The guild does not require MFA for elevated actions.
    None,
    /// The guild requires MFA on a user account which has elevated permissions.
    Elevated,
}

/// A guild's default message notification setting.
#[derive(Deserialize_repr, Debug, Serialize_repr, Clone)]
#[repr(u8)]
pub enum DefaultMessageNotifications {
    /// A user will be notified whenever a new message is sent in the guild.
    AllMessages,
    /// A user will only be notified when they are mentioned.
    OnlyMentions
}

/// A guild's verification levels.
#[derive(Deserialize_repr, Debug, Clone, Serialize_repr)]
#[repr(u8)]
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

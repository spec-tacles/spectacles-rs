//! Structures related to Discord guilds.
use chrono::{DateTime, FixedOffset};

pub use self::member::GuildMember;
pub use self::role::Role;

mod role;
mod member;

/// A Discord Guild, commonly referred to as a "server".
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Guild {
    /// The snowflake ID of this guild.
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
    pub afk_channel_id: Option<u64>,
    /// The AFK channel timeout for this guild.
    pub afk_timeout: u64,
    /// The ID of the application which created the guild, if applicable.
    pub application_id: Option<u64>,
    /// The verification level, which determines which users can chat in a guild.
    pub verification_level: VerificationLevel,
    /// The MFA authentication level for this guild.
    pub mfa_level: MfaLevel,
    /// Whether or not the guild can be embedded in a widget.
    pub embed_enabled: bool,
    /// The channel ID that an embed widget will be generated for.
    pub embed_channel_id: u64,
    /// The ID of the channel in which system messages are sent to.
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

/// A guild's explicit content filter levels.
#[derive(Clone, Debug, Copy)]
pub enum ExplicitContentFilter {
    /// The filter is not active.
    Disabled = 0,
    /// The filter is only active for members without roles.
    MembersWithoutRoles = 1,
    /// The filter is active for all members.
    AllMembers = 2
}

impl ExplicitContentFilter {
    pub fn num(&self) -> u8 {
        match *self {
            ExplicitContentFilter::Disabled => 0,
            ExplicitContentFilter::MembersWithoutRoles => 1,
            ExplicitContentFilter::AllMembers => 2
        }
    }

}

/// A guild's MFA levels.
#[derive(Clone, Debug, Copy)]
pub enum MfaLevel {
    /// The guild does not require MFA for elevated actions.
    None = 0,
    /// The guild requires MFA on a user account which has elevated permissions.
    Elevated = 1,
}

impl MfaLevel {
    pub fn num(&self) -> u8 {
        match *self {
            MfaLevel::None => 0,
            MfaLevel::Elevated => 1
        }
    }
}

/// A guild's default message notification setting.
#[derive(Clone, Debug, Copy)]
pub enum DefaultMessageNotifications {
    /// A user will be notified whenever a new message is sent in the guild.
    AllMessages = 0,
    /// A user will only be notified when they are mentioned.
    OnlyMentions = 1
}

impl DefaultMessageNotifications {
    pub fn num(&self) -> u8 {
        match *self {
            DefaultMessageNotifications::AllMessages => 0,
            DefaultMessageNotifications::OnlyMentions => 1
        }
    }
}

/// A guild's verification levels.
#[derive(Clone, Debug, Copy)]
pub enum VerificationLevel {
    /// The guild is unrestricted.
    None = 0,
    /// The guild requires a verified email on the user's account.
    Low = 1,
    /// The guild requires that the user be registered on Discord for longer than 5 minutes.
    Medium = 2,
    /// The guild requires that the user be on the guild for longer than 10 minutes.
    High = 3,
    /// The guild requires that the user have a verified phone number on their account.
    Insane = 4
}

impl VerificationLevel {
    pub fn num(&self) -> u8 {
        match *self {
            VerificationLevel::None => 0,
            VerificationLevel::Low => 1,
            VerificationLevel::Medium => 2,
            VerificationLevel::High => 3,
            VerificationLevel::Insane => 4
        }
    }
}
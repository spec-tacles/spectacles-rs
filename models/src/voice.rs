//! Structures for interfacing with Discord's voice related features.
use crate::guild::GuildMember;

/// Represents a user's voice connection status.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VoiceState {
    /// The guild ID of the guild this voice state belongs to.
    #[serde(default)]
    pub guild_id: String,
    /// The channel ID of the channel the user is connected to.
    pub channel_id: Option<String>,
    /// The user ID of the user this voice state belongs to.
    pub user_id: String,
    /// The guild member that this voice state belongs to.
    #[serde(default)]
    pub member: GuildMember,
    /// The session ID of this voice state.
    pub session_id: String,
    /// Whether or not the user is deafened on the server.
    pub deaf: bool,
    /// Whether or not the user is muted on the server.
    pub muted: bool,
    /// Whether or not the user is locally deaf.
    pub self_deaf: bool,
    /// Whether or not the user is locally muted.
    pub self_mute: bool,
    /// Whether or not the user was muted by the current user.
    pub suppress: bool
}

/// Represents a Discord voice region.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceRegion {
    /// The ID of this voice region.
    pub id: String,
    /// The name of this voice region.
    pub name: String,
    /// Whether or not this server is a VIP-only server.
    pub vip: bool,
    /// Whetehr or not this region is the closest to the user's client.
    pub optimal: bool,
    /// Whether or not this voice region has been deprecated.
    pub deprecated: bool,
    /// Whether or not this is a custom voice region.
    pub custom: bool
}
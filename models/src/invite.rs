//! Structures related to a Discord invite.
use chrono::{DateTime, FixedOffset};

use crate::channel::Channel;
use crate::guild::Guild;
use crate::User;

/// Represents a code that when used, adds a user to a guild or group DM channel.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Invite {
    /// The ID of the invite code.
    pub code: String,
    /// The guild that the invite belongs to.
    pub guild: Option<Guild>,
    /// The channel that the invite belongs to.
    pub channel: Option<Channel>,
    /// The approximate count of online members.
    pub approximate_presence_count: i32,
    /// The approximate count of total members.
    pub approximate_member_count: i32
}

/// Detailed information about an invite.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct InviteMetadata {
    /// The user who created the invite.
    pub inviter: User,
    /// The amount of times that this invite has been used.
    pub uses: i32,
    /// The maximum amount of uses allowed for this invite.
    pub max_uses: i32,
    /// The duration after which the invite expires, in seconds.
    pub max_age: i32,
    /// Whether or not this invite grants temporary membership.
    pub temporary: bool,
    /// The date that this invite was created.
    pub created_at: DateTime<FixedOffset>,
    /// Whether or not this invite has been revoked.
    pub revoked: bool
}
//! Structures related to a Discord invite.
use std::time::Duration;

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
    #[serde(default)]
    pub guild: Option<Guild>,
    /// The channel that the invite belongs to.
    pub channel: Channel,
    /// The approximate count of online members.
    #[serde(default)]
    pub approximate_presence_count: Option<i32>,
    /// The approximate count of total members.
    #[serde(default)]
    pub approximate_member_count: Option<i32>
}

/// Represents a partial channel invite.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PartialInvite {
    /// The code of the invite, if one is set.
    pub code: String
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

/// Represents data that is sent to the create invite endpoint.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateInviteOptions {
    max_age: u64,
    max_uses: i32,
    temporary: bool,
    unique: bool,
}

impl CreateInviteOptions {
    pub fn new() -> Self {
        CreateInviteOptions {
            max_age: 86400,
            max_uses: 0,
            temporary: false,
            unique: false,
        }
    }

    /// Sets the maximum age for this invite.
    pub fn set_max_age(mut self, age: Duration) -> Self {
        self.max_age = age.as_secs();
        self
    }

    /// Sets the maximum # of uses for this invite.
    pub fn set_max_uses(mut self, uses: i32) -> Self {
        self.max_uses = uses;
        self
    }

    /// Sets whether or not this invite should be temporary.
    pub fn set_temporary(mut self, temp: bool) -> Self {
        self.temporary = temp;
        self
    }

    /// Sets whether or not this invite should be unique.
    pub fn set_unique(mut self, unique: bool) -> Self {
        self.unique = unique;
        self
    }
}
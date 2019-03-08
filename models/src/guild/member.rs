use chrono::{DateTime, FixedOffset};

use crate::{
    guild::Role,
    user::User,
};

/// A User that is part of a guild.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMember {
    /// The Discord user of this guild member.
    pub user: User,
    #[serde(rename = "nick")]
    /// The member's nickname, if applicable.
    pub nickname: Option<String>,
    /// The date that they joined the server.
    pub joined_at: DateTime<FixedOffset>,
    /// Whether or not the member is muted.
    pub mute: bool,
    /// Whether or not the member has been deafened.
    pub deaf: bool,
    /// A collection of roles that this member has.
    pub roles: Vec<Role>,
    pub guild_id: String
}
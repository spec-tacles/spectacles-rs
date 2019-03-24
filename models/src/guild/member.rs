use crate::{
    user::User,
    Snowflake
};
use chrono::{DateTime, FixedOffset};

/// A User that is part of a guild.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GuildMember {
    /// The guild ID of this guild member. (Guild Member Add)
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    /// The Discord user of this guild member.
    pub user: Option<User>,
    /// The member's nickname, if applicable.
    #[serde(default, rename = "nick")]
    pub nickname: Option<String>,
    /// The date that they joined the server.
    pub joined_at: Option<DateTime<FixedOffset>>,
    /// Whether or not the member is muted.
    #[serde(default)]
    pub mute: bool,
    /// Whether or not the member has been deafened.
    #[serde(default)]
    pub deaf: bool,
    /// A collection of roles that this member has.
    pub roles: Vec<String>,
}

/// A payload sent by the gateway when a guild member is removed from a guild.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GuildMemberRemove {
    /// The guild ID of the guild that this member belongs to.
    pub guild_id: Snowflake,
    /// The Discord User of this guild member.
    pub user: User,
}

/// A payload sent by the gateway upon a RequestGuildMembers packet.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GuildMembersChunk {
    /// The guild ID of the guild that the members belong to.
    pub guild_id: Snowflake,
    /// The array of guild members.
    pub members: Vec<GuildMember>
}
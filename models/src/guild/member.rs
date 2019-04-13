use chrono::{DateTime, FixedOffset};

use crate::{
    Snowflake,
    user::User,
};

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

/// Options for adding a member to a guild.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct AddMemberOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
}

impl AddMemberOptions {
    /// Sets the access token of the user which you want to add.
    pub fn access_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    /// Sets the nickname for the newly created user.
    pub fn nickname(mut self, name: &str) -> Self {
        self.nick = Some(name.to_string());
        self
    }

    /// Sets the roles that the user should have upon joining the guild.
    pub fn roles(mut self, rls: Vec<Snowflake>) -> Self {
        self.roles = Some(rls);
        self
    }

    /// Sets the user's muted status when joining the guild.
    pub fn muted(mut self, mute: bool) -> Self {
        self.mute = Some(mute);
        self
    }

    /// Sets the deaf status of the user when joining the guild.
    pub fn deaf(mut self, opt: bool) -> Self {
        self.deaf = Some(opt);
        self
    }
}

/// Options for modifying a guild member.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ModifyMemberOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Snowflake>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Snowflake>,
}

impl ModifyMemberOptions {
    /// Sets the new nickname of this user.
    pub fn nick(mut self, name: &str) -> Self {
        self.nick = Some(name.to_string());
        self
    }

    /// Sets the roles that should be assigned to the user.
    pub fn roles(mut self, rls: Vec<Snowflake>) -> Self {
        self.roles = Some(rls);
        self
    }

    /// Sets the muted status of this guild member.
    pub fn muted(mut self, opt: bool) -> Self {
        self.mute = Some(opt);
        self
    }

    /// Sets the deafened status of this guild member.
    pub fn deaf(mut self, opt: bool) -> Self {
        self.deaf = Some(opt);
        self
    }

    /// Sets the voice channel that this member should be moved to.
    pub fn channel_id(mut self, id: Snowflake) -> Self {
        self.channel_id = Some(id);
        self
    }
}

/// Options for requesting a list of guild members from the API.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ListMembersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Snowflake>,
}

impl ListMembersOptions {
    /// Sets the maximum amount of members for this request.
    pub fn limit(mut self, num: i32) -> Self {
        self.limit = Some(num);
        self
    }

    /// Sets the the highest user id in the previous page.
    pub fn after(mut self, id: Snowflake) -> Self {
        self.after = Some(id);
        self
    }
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
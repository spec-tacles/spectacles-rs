use crate::snowflake::Snowflake;

/// Represents a Discord Role.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    /// The snowflake ID of this role.
    pub id: Snowflake,
    /// The name of this role.
    pub name: String,
    /// The hexadecimal color code for this role.
    pub color: i32,
    /// whether or not this role is hoisted.
    #[serde(rename = "hoist")]
    pub hoisted: bool,
    /// The position of this role.
    pub position: i32,
    /// Whether or not this role is managed by an integration.
    pub managed: bool,
    /// Whether or not this role is mentionable.
    pub mentionable: bool
}

/// Represents a packet sent by the gateway when a guild role is created/updated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildRoleCreateOrUpdate {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The role that was created.
    pub role: Role
}

/// Represents a packet sent by the gateway when a guild role is deleted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildRoleDelete {
    /// The guild ID of the guild.
    pub guild_id: Snowflake,
    /// The role ID of the role that was deleted.
    pub role_id: Snowflake
}
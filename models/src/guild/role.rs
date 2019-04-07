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

/// Options for creating a role in a guild.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct CreateRoleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
}

impl CreateRoleOptions {
    /// Sets the name of the role.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the permissions that this role should have.
    pub fn permissions(mut self, perms: i32) -> Self {
        self.permissions = Some(perms);
        self
    }

    /// Sets the color of the role.
    pub fn color(mut self, clr: i32) -> Self {
        self.color = Some(clr);
        self
    }

    /// Sets the hoisted status of this role.
    pub fn hoisted(mut self, opt: bool) -> Self {
        self.hoist = Some(opt);
        self
    }

    /// Sets the mentionable status of this user.
    pub fn mentionable(mut self, opt: bool) -> Self {
        self.mentionable = Some(opt);
        self
    }
}

/// Options for modifying a role in a guild.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ModifyRoleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
}

impl ModifyRoleOptions {
    /// Sets a new name for the role.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the new permissions for the role.
    pub fn permissions(mut self, perms: i32) -> Self {
        self.permissions = Some(perms);
        self
    }

    /// Sets the new color of the role.
    pub fn color(mut self, clr: i32) -> Self {
        self.color = Some(clr);
        self
    }

    /// Sets the hoist status of this role.
    pub fn hoisted(mut self, opt: bool) -> Self {
        self.hoist = Some(opt);
        self
    }

    /// Sets the mentionable status of this role.
    pub fn mentionable(mut self, opt: bool) -> Self {
        self.mentionable = Some(opt);
        self
    }
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
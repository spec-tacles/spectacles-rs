use super::parse_snowflake;

/// Represents a Discord Role.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    /// The snowflake ID of this role.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// The name of this role.
    pub name: String,
    /// The hexadecimal color code for this role.
    pub color: i32,
    /// whether or not this role is hoisted.
    #[serde(rename = "hoisted")]
    pub hoist: bool,
    /// The position of this role.
    pub position: i32,
    /// Whether or not this role is managed by an integration.
    pub managed: bool,
    /// Whether or not this role is mentionable.
    pub mentionable: bool
}
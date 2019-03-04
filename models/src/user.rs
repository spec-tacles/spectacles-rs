use super::parse_snowflake;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a User on Discord.
pub struct User {
    /// The Snowflake ID of this user.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// The username of this user.
    pub username: String,
    /// The four-digit number following the user's username.
    pub discriminator: u16,
    /// The user's avatar hash, if they have one.
    pub avatar: Option<String>,
    /// Whether or not this user is a bot.
    pub bot: bool,
    /// Whether or not this user has two factor authentication on their account.
    pub mfa_enabled: bool,
    /// The user's email. Only available on user accounts.
    pub email: Option<String>
}

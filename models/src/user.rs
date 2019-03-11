#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a User on Discord.
pub struct User {
    /// The Snowflake ID of this user.
    pub id: String,
    /// The username of this user.
    pub username: String,
    /// The four-digit number following the user's username.
    pub discriminator: String,
    /// The user's avatar hash, if they have one.
    pub avatar: Option<String>,
    /// Whether or not this user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// Whether or not this user has two factor authentication on their account.
    #[serde(default)]
    pub mfa_enabled: bool,
    /// The user's email. Only available on user accounts.
    #[serde(default)]
    pub email: Option<String>
}

use crate::Snowflake;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// Represents a User on Discord.
pub struct User {
    /// The Snowflake ID of this user.
    pub id: Snowflake,
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

impl User {
    pub fn get_avatar_url(&self, format: &str) -> String {
        if let Some(h) = &self.avatar {
            format!("https://cdn.discordapp.com/avatars/{}/{}.{}", self.id.0, h, format)
        } else {
            let avatars = vec![
                "6debd47ed13483642cf09e832ed0bc1b",
                "322c936a8c8be1b803cd94861bdfa868",
                "dd4dbc0016779df1378e7812eabaa04d",
                "0e291f67c9274a1abdddeb3fd919cbaa",
                "1cbd08c76f8af6dddce02c5138971129",
            ];
            let hash = avatars[self.discriminator.parse::<usize>().unwrap() % avatars.len()];

            format!("https://cdn.discordapp.com/avatars/{}/{}.{}", self.id.0, hash, format)
        }
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("<@{}>", self.id.0)
    }
}
use crate::User;

/// A simple solution to post messages in Discord channels from external sources.
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Webhook {
    /// The webhook ID of this webhook.
    pub id: String,
    /// The guild ID of the guild which the webhook belongs to.
    pub guild_id: String,
    /// The channel ID of the channel which the webhook belongs to.
    pub channel_id: String,
    /// The user who created this webhook.
    #[serde(default)]
    pub user: User,
    /// The default name of this webhook.
    pub name: Option<String>,
    /// The default avatar hash of this webhook.
    pub avatar_hash: Option<String>,
    /// The secure token of this webhook.
    pub token: String
}

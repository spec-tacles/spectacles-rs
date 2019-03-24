use crate::{Snowflake, User};

/// A Discord emote than can be used to react to messages.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Emoji {
    /// The ID of the emoji.
    pub id: Option<Snowflake>,
    /// The name of the emoji.
    pub name: String,
    /// The roles that the emoji is whitelisted to.
    #[serde(default)]
    pub roles: Vec<Snowflake>,
    /// The user who created this emoji.
    #[serde(default)]
    pub user: Option<User>,
    /// Whether or not this emoji must be wrapped in colons.
    #[serde(default)]
    pub require_colons: Option<bool>,
    /// Whether or not this emoji is managed.
    #[serde(default)]
    pub managed: Option<bool>,
    /// Whether or not this emoji is animated.
    #[serde(default)]
    pub animated: Option<bool>
}

/// A reaction on a message.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageReaction {
    /// The times that this reaction has been clicked.
    pub count: i32,
    /// Whether or not the current user has reacted on this message.
    pub me: bool,
    /// Emoji information.
    pub emoji: Emoji
}

/// The gateway event emitted when a guild's emojis are updated.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GuildEmojisUpdate {
    /// The guild ID that the emojis belong to.
    pub guild_id: Snowflake,
    /// The collion of guild emojis.
    pub emojis: Vec<Emoji>
}
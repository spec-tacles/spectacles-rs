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

/// Query for getting the users who reacted to a message.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetReactions {
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetReactions {
    /// Get users before this message ID.
    pub fn before(mut self, id: Snowflake) -> Self {
        self.before = Some(id);
        self
    }

    /// Get users after this message ID.
    pub fn after(mut self, id: Snowflake) -> Self {
        self.after = Some(id);
        self
    }

    /// Sets the maximum # of users to return.
    pub fn limit(mut self, num: i32) -> Self {
        self.limit = Some(num);
        self
    }
}

/// The gateway event emitted when a guild's emojis are updated.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GuildEmojisUpdate {
    /// The guild ID that the emojis belong to.
    pub guild_id: Snowflake,
    /// The collion of guild emojis.
    pub emojis: Vec<Emoji>
}
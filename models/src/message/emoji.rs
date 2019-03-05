use crate::parse_snowflake;

/// A Discord emote than can be used to react to messages.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    /// The ID of the emoji.
    #[serde(deserialize_with = "parse_snowflake")]
    pub id: u64,
    /// The name of the emoji.
    pub name: String,
    /// The roles that the emoji is whitelisted to.
    pub roles: Option<Vec<String>>,
    /// Whether or not this emoji must be wrapped in colons.
    pub require_colons: Option<bool>,
    /// Whether or not this emoji is managed.
    pub managed: Option<bool>,
    /// Whether or not this emoji is animated.
    pub animated: Option<bool>
}

/// A reaction on a message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReaction {
    /// The times that this reaction has been clicked.
    pub count: i32,
    /// Whether or not the current user has reacted on this message.
    pub me: bool,
    /// Emoji information.
    pub emoji: Emoji
}
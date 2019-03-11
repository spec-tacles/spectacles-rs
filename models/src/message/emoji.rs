use crate::User;

/// A Discord emote than can be used to react to messages.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Emoji {
    /// The ID of the emoji.
    pub id: String,
    /// The name of the emoji.
    pub name: String,
    /// The roles that the emoji is whitelisted to.
    #[serde(default)]
    pub roles: Vec<String>,
    /// The user who created this emoji.
    #[serde(default)]
    pub user: User,
    /// Whether or not this emoji must be wrapped in colons.
    #[serde(default)]
    pub require_colons: bool,
    /// Whether or not this emoji is managed.
    #[serde(default)]
    pub managed: bool,
    /// Whether or not this emoji is animated.
    #[serde(default)]
    pub animated: bool
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
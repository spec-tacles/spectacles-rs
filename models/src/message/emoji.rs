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
pub struct GetReactionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetReactionsOptions {
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateEmojiOptions {
    name: String,
    image: String,
    roles: Vec<Snowflake>,
}

impl CreateEmojiOptions {
    /// Sets the name of this emoji.
    pub fn name(mut self, text: &str) -> Self {
        self.name = text.to_string();
        self
    }

    /// Sets the image for this emoji.
    /// Discord requires that the image is base64 encoded, in the format listed [here.](https://discordapp.com/developers/docs/resources/user#avatar-data)
    pub fn image(mut self, text: &str) -> Self {
        self.image = text.to_string();
        self
    }

    /// Sets the roles for which will this emoji will be whitelisted.
    pub fn roles(mut self, rls: Vec<Snowflake>) -> Self {
        self.roles = rls;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ModifyEmojiOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Snowflake>>,
}

impl ModifyEmojiOptions {
    /// Sets the new name for this emoji.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the roles for which will this emoji will be whitelisted.
    pub fn roles(mut self, rls: Vec<Snowflake>) -> Self {
        self.roles = Some(rls);
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
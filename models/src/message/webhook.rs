use tokio_fs::File;

use crate::{Snowflake, User};
use crate::message::embed::Embed;

/// A simple solution to post messages in Discord channels from external sources.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Webhook {
    /// The webhook ID of this webhook.
    pub id: Snowflake,
    /// The guild ID of the guild which the webhook belongs to.
    #[serde(default)]
    pub guild_id: Option<String>,
    /// The channel ID of the channel which the webhook belongs to.
    pub channel_id: Snowflake,
    /// The user who created this webhook.
    #[serde(default)]
    pub user: Option<User>,
    /// The default name of this webhook.
    pub name: Option<String>,
    /// The default avatar hash of this webhook.
    pub avatar: Option<String>,
    /// The secure token of this webhook.
    pub token: String
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ModifyWebhookOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Snowflake>,
}

impl ModifyWebhookOptions {
    /// Sets a new name for the webhook.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets a new avatar for this webhook.
    /// This url must be base64 encodes according to Discord specifications, which can be found [here.](https://discordapp.com/developers/docs/resources/user#avatar-data)
    pub fn avatar(mut self, url: &str) -> Self {
        self.avatar = Some(url.to_string());
        self
    }

    /// Sets the new channel ID for this webhook.
    pub fn channel_id(mut self, id: Snowflake) -> Self {
        self.channel_id = Some(id);
        self
    }
}

#[derive(Serialize, Debug, Default)]
pub struct ExecuteWebhookOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing)]
    pub file: Option<(String, File)>,
    embeds: Vec<Embed>,
}

impl ExecuteWebhookOptions {
    /// Adds a content for this webhook's message.
    pub fn content(mut self, text: &str) -> Self {
        self.content = Some(text.to_string());
        self
    }

    /// Overrides the default username of this webhook.
    pub fn username(mut self, name: &str) -> Self {
        self.username = Some(name.to_string());
        self
    }

    /// Sets the avatar url for this webhook.
    pub fn avatar_url(mut self, url: &str) -> Self {
        self.avatar_url = Some(url.to_string());
        self
    }

    /// Sets the TTS flag for this message.
    pub fn tts(mut self, opt: bool) -> Self {
        self.tts = Some(opt);
        self
    }

    /// Adds a file to be sent with this webhook's message.
    pub fn file(mut self, name: &str, file: File) -> Self {
        self.file = Some((name.to_string(), file));
        self
    }

    /// Adds an embed to the collection of embeds being sent with this embed.
    pub fn embed(mut self, embe: Embed) -> Self {
        self.embeds.push(embe);
        self
    }
}
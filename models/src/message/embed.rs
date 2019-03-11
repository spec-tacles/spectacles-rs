/// Represents a Message Embed being sent.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Embed {
    /// The title of the embed.
    #[serde(default)]
    pub title: String,
    /// The type of embed.
    #[serde(default, rename = "type")]
    pub kind: String,
    /// The description of the embed.
    #[serde(default)]
    pub description: String,
    /// The URL of the embed.
    #[serde(default)]
    pub url: String,
    /// The timestamp of the embed.
    #[serde(default)]
    pub timestamp: String,
    /// The color of the embed.
    #[serde(default)]
    pub color: i32,
    /// Information about the embed's footer.
    #[serde(default)]
    pub footer: EmbedFooter,
    /// Information about the embed's image.
    #[serde(default)]
    pub image: EmbedImage,
    /// Information about the embed's thumbnail.
    #[serde(default)]
    pub thumbnail: EmbedThumbnail,
    /// Information about an embed's video, if applicable.
    #[serde(default)]
    pub video: EmbedVideo,
    /// Information about an embed's provider if applicable.
    #[serde(default)]
    pub provider: EmbedProvider,
    /// Information about the embed's author.
    #[serde(default)]
    pub author: EmbedAuthor,
    /// Information about the embed's fields.
    pub fields: EmbedField
}

/// An Embed Footer data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedFooter {
    /// The text of this footer.
    pub text: String,
    /// The Icon URL of this footer.
    #[serde(default)]
    pub icon_url: String,
    /// The proxied URL of the icon.
    #[serde(default)]
    pub proxy_icon_url: String

}

/// An Embed Image data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedImage {
    /// The source URL of the image.
    #[serde(default)]
    pub url: String,
    /// A proxied URL of the image.
    #[serde(default)]
    pub proxy_url: String,
    /// The height of the image.
    #[serde(default)]
    pub height: i32,
    /// The width of the image.
    #[serde(default)]
    pub width: i32
}

/// An Embed Thumbnail data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedThumbnail {
    /// The source URL of the thumbnail.
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    /// A proxied URL of the thumbnail.
    pub proxy_url: String,
    /// The height of the thumbnail.
    #[serde(default)]
    pub height: i32,
    /// The width of the thumbnail.
    #[serde(default)]
    pub width: i32
}

/// An Embed Video data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedVideo {
    /// The source URL of the video.
    #[serde(default)]
    pub url: String,
    /// The height of the video.
    #[serde(default)]
    pub height: i32,
    /// The width of the thumbnail.
    #[serde(default)]
    pub width: i32
}

/// Information about the embed's provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedProvider {
    /// The name of the provider.
    #[serde(default)]
    pub name: String,
    /// The url of the provider.
    #[serde(default)]
    pub url: String
}

/// Information about the embed's author.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedAuthor {
    /// The name of the author.
    #[serde(default)]
    pub name: String,
    /// The URL of the author.
    #[serde(default)]
    pub url: String,
    /// The URL of the author's icon.
    #[serde(default)]
    pub icon_url: String,
    /// A proxied version of the author's icon.
    #[serde(default)]
    pub proxy_icon_url: String
}

/// Represents an Embed Field object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: String,
    /// Whether or not this field should display as inline.
    #[serde(default)]
    pub inline: bool
}
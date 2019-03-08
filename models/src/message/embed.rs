use chrono::{DateTime, FixedOffset};

/// Represents a Message Embed being sent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embed {
    /// The title of the embed.
    pub title: Option<String>,
    /// The type of embed.
    pub r#type: Option<String>,
    /// The description of the embed.
    pub description: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The timestamp of the embed.
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// The color of the embed.
    pub color: Option<i32>,
    /// Information about the embed's footer.
    pub footer: Option<EmbedFooter>,
    /// Information about the embed's image.
    pub image: Option<EmbedImage>,
    /// Information about the embed's thumbnail.
    pub thumbnail: Option<EmbedThumbnail>,
    /// Information about an embed's video, if applicable.,
    pub video: Option<EmbedVideo>,
    /// Information about an embed's provider if applicable.
    pub provider: Option<EmbedProvider>,
    /// Information about the embed's author.
    pub author: Option<EmbedAuthor>,
    /// Information about the embed's fields.
    pub fields: Option<EmbedField>
}

/// An Embed Footer data object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedFooter {
    /// The text of this footer.
    pub footer: String,
    /// The Icon URL of this footer.
    pub icon_url: Option<String>

}

/// An Embed Image data object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedImage {
    /// The source URL of the image.
    pub url: Option<String>,
    /// A proxied URL of the image.
    pub proxy_url: Option<String>,
    /// The height of the image.
    pub height: Option<i32>,
    /// The width of the image.
    pub width: Option<i32>
}

/// An Embed Thumbnail data object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedThumbnail {
    /// The source URL of the thumbnail.
    pub thumbnail: Option<String>,
    /// A proxied URL of the thumbnail.
    pub proxy_url: Option<String>,
    /// The height of the thumbnail.
    pub height: i32,
    /// The width of the thumbnail.
    pub width: i32
}

/// An Embed Video data object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedVideo {
    /// The source URL of the video.
    pub url: Option<String>,
    /// The height of the video.
    pub height: i32,
    /// The width of the thumbnail.
    pub width: i32
}

/// Information about the embed's provider.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: Option<String>,
    /// The url of the provider.
    pub url: Option<String>
}

/// Information about the embed's author.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: Option<String>,
    /// The URL of the author.
    pub url: Option<String>,
    /// The URL of the author's icon.
    pub icon_url: Option<String>,
    /// A proxied version of the author's icon.
    pub proxy_icon_url: Option<String>
}

/// Represents an Embed Field object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbedField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: String,
    /// Whether or not this field should display as inline.
    pub inline: Option<bool>
}
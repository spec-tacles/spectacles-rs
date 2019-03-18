/// Represents a Message Embed being sent.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Embed {
    /// The title of the embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of embed.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The description of the embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL of the embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The timestamp of the embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The color of the embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    /// Information about the embed's footer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Information about the embed's image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    /// Information about the embed's thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    /// Information about an embed's video, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    /// Information about an embed's provider if applicable.
    #[serde(default)]
    pub provider: Option<EmbedProvider>,
    /// Information about the embed's author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Information about the embed's fields.
    #[serde(default)]
    pub fields: Vec<EmbedField>
}

impl Embed {
    pub fn new() -> Self {
        Embed::default()
    }

    /// Sets the title for this embed.
    pub fn set_title(mut self, text: impl Into<String>) -> Self {
        self.title = Some(text.into());
        self
    }

    /// Sets the description of this embed.
    pub fn set_description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    /// Sets the color of this embed.
    pub fn set_color(mut self, code: i32) -> Self {
        self.color = Some(code);
        self
    }

    /// Adds a field to this embed.
    pub fn add_field<F: FnOnce(EmbedField) -> EmbedField>(mut self, builder: F) -> Self {
        let new_field = builder(EmbedField::default());
        self.fields.push(new_field);
        self
    }

    /// Sets the author of this embed.
    pub fn set_author<F>(mut self, author: F) -> Self
        where F: FnOnce(EmbedAuthor) -> EmbedAuthor
    {
        self.author = Some(author(EmbedAuthor::default()));
        self
    }

    /// Sets the footer of this embed.
    pub fn set_footer<F>(mut self, footer: F) -> Self
        where F: FnOnce(EmbedFooter) -> EmbedFooter
    {
        self.footer = Some(footer(EmbedFooter::default()));
        self
    }

    /// Adds a thumbnail to this embed.
    pub fn set_thumbnail<T>(mut self, thumb: T) -> Self
        where T: FnOnce(EmbedThumbnail) -> EmbedThumbnail
    {
        self.thumbnail = Some(thumb(EmbedThumbnail::default()));
        self
    }

    /// Adds an image to this embed.
    pub fn set_image<F>(mut self, img: F) -> Self
        where F: FnOnce(EmbedImage) -> EmbedImage
    {
        self.image = Some(img(EmbedImage::default()));
        self
    }
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

impl EmbedFooter {
    /// Sets the text for this footer.
    pub fn set_text(mut self, txt: impl Into<String>) -> Self {
        self.text = txt.into();
        self
    }

    /// Set the icon URL for this footer.
    pub fn set_icon_url(mut self, url: impl Into<String>) -> Self {
        self.icon_url = url.into();
        self
    }
}
/// An Embed Image data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedImage {
    /// The source URL of the image.
    #[serde(default)]
    pub url: Option<String>,
    /// A proxied URL of the image.
    #[serde(default)]
    pub proxy_url: Option<String>,
    /// The height of the image.
    #[serde(default)]
    pub height: Option<i32>,
    /// The width of the image.
    #[serde(default)]
    pub width: Option<i32>
}

impl EmbedImage {
    /// Set the URL for this image.
    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// An Embed Thumbnail data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedThumbnail {
    /// The source URL of the thumbnail.
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    /// A proxied URL of the thumbnail.
    pub proxy_url: Option<String>,
    /// The height of the thumbnail.
    #[serde(default)]
    pub height: Option<i32>,
    /// The width of the thumbnail.
    #[serde(default)]
    pub width: Option<i32>
}

impl EmbedThumbnail {
    /// Set the URL of this thumbnail.
    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}
/// An Embed Video data object.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedVideo {
    /// The source URL of the video.
    #[serde(default)]
    pub url: Option<String>,
    /// The height of the video.
    #[serde(default)]
    pub height: Option<i32>,
    /// The width of the thumbnail.
    #[serde(default)]
    pub width: Option<i32>
}

/// Information about the embed's provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedProvider {
    /// The name of the provider.
    #[serde(default)]
    pub name: Option<String>,
    /// The url of the provider.
    #[serde(default)]
    pub url: Option<String>
}

/// Information about the embed's author.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmbedAuthor {
    /// The name of the author.
    #[serde(default)]
    pub name: Option<String>,
    /// The URL of the author.
    #[serde(default)]
    pub url: Option<String>,
    /// The URL of the author's icon.
    #[serde(default)]
    pub icon_url: Option<String>,
    /// A proxied version of the author's icon.
    #[serde(default)]
    pub proxy_icon_url: Option<String>
}

impl EmbedAuthor {
    /// Set the name of the author.
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the URL for this author.
    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the author's icon URL.
    pub fn set_icon_url(mut self, url: impl Into<String>) -> Self {
        self.icon_url = Some(url.into());
        self
    }
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
    pub inline: Option<bool>
}

impl EmbedField {
    /// Sets the name of this field.
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn set_value(mut self, val: impl Into<String>) -> Self {
        self.value = val.into();
        self
    }
}
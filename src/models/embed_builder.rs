use crate::consts::color::Color;

use super::embed;

/// A builder for creating embeds.
pub struct EmbedBuilder {
    embed: embed::Embed,
}

impl EmbedBuilder {
    /// Creates a new `EmbedBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            embed: Default::default(),
        }
    }

    /// Builds the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// let embed = EmbedBuilder::new().title("Title").build();
    /// ```
    pub fn build(self) -> embed::Embed {
        self.embed
    }

    /// Sets the title of the embed.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().title("Title");
    /// ```
    pub fn title(mut self, title: &str) -> Self {
        self.embed.title = Some(title.to_owned());
        self
    }

    /// Sets the description of the embed.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().description("Description");
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.embed.description = Some(description.to_owned());
        self
    }

    /// Sets the color of the embed.
    ///
    /// # Arguments
    ///
    /// * `color` - The color of the embed.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().color(Color::from_rgb(255, 0, 0));
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.embed.color = Some(color.into());
        self
    }

    /// Sets the footer of the embed.
    ///
    /// # Arguments
    ///
    /// * `text` - The text of the footer.
    /// * `icon_url` - The URL of the footer icon.
    /// * `proxy_icon_url` - The proxy URL of the footer icon.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().footer("Footer text", Some("https://example.com/icon.png".to_string()), None);
    /// ```
    pub fn footer(
        mut self,
        text: &str,
        icon_url: Option<String>,
        proxy_icon_url: Option<String>,
    ) -> Self {
        self.embed.footer = Some(embed::EmbedFooter {
            text: text.to_owned(),
            icon_url,
            proxy_icon_url,
        });

        self
    }

    /// Sets the image of the embed.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the image.
    /// * `height` - The height of the image.
    /// * `width` - The width of the image.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().image("https://example.com/image.png".to_string(), Some(100), Some(100));
    /// ```
    pub fn image(mut self, url: String, height: Option<u32>, width: Option<u32>) -> Self {
        self.embed.image = Some(embed::EmbedImage {
            url,
            proxy_url: None,
            height,
            width,
        });

        self
    }

    /// Sets the thumbnail of the embed.
    ///
    /// # Arguments
    ///
    /// * `thumbnail` - The thumbnail object.
    ///
    /// # Examples
    ///
    /// ```
    /// let thumbnail = embed::EmbedThumbnail {
    ///     url: "https://example.com/thumbnail.png".to_string(),
    ///     proxy_url: None,
    ///     height: None,
    ///     width: None,
    /// };
    /// let builder = EmbedBuilder::new().thumbnail(thumbnail);
    /// ```
    pub fn thumbnail(mut self, thumbnail: embed::EmbedThumbnail) -> Self {
        self.embed.thumbnail = Some(thumbnail);
        self
    }

    /// Sets the video of the embed.
    ///
    /// # Arguments
    ///
    /// * `video` - The video object.
    ///
    /// # Examples
    ///
    /// ```
    /// let video = embed::EmbedVideo {
    ///     url: "https://example.com/video.mp4".to_string(),
    ///     proxy_url: None,
    ///     height: None,
    ///     width: None,
    /// };
    /// let builder = EmbedBuilder::new().video(video);
    /// ```
    pub fn video(mut self, video: embed::EmbedVideo) -> Self {
        self.embed.video = Some(video);
        self
    }

    /// Sets the author of the embed.
    ///
    /// # Arguments
    ///
    /// * `author` - The author object.
    ///
    /// # Examples
    ///
    /// ```
    /// let author = embed::EmbedAuthor {
    ///     name: "Author".to_string(),
    ///     url: Some("https://example.com".to_string()),
    ///     icon_url: Some("https://example.com/icon.png".to_string()),
    ///     proxy_icon_url: None,
    /// };
    /// let builder = EmbedBuilder::new().author(author);
    /// ```
    pub fn author(mut self, author: embed::EmbedAuthor) -> Self {
        self.embed.author = Some(author);
        self
    }

    /// Adds a field to the embed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    /// * `inline` - Whether the field is inline.
    ///
    /// # Panics
    ///
    /// Panics if the embed already contains 25 fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EmbedBuilder::new().field("Field name", "Field value", true);
    /// ```
    pub fn field(mut self, name: &str, value: &str, inline: bool) -> Self {
        if self.embed.fields.len() == 25 {
            panic!("Embeds can only contain max of 25 fields");
        }

        self.embed.fields.push(embed::EmbedField {
            name: name.to_owned(),
            value: value.to_owned(),
            inline,
        });

        self
    }

    /// Adds multiple fields to the embed.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of fields.
    ///
    /// # Panics
    ///
    /// Panics if the embed already contains 25 fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let fields = vec![
    ///     embed::EmbedField {
    ///         name: "Field 1".to_string(),
    ///         value: "Value 1".to_string(),
    ///         inline: true,
    ///     },
    ///     embed::EmbedField {
    ///         name: "Field 2".to_string(),
    ///         value: "Value 2".to_string(),
    ///         inline: false,
    ///     },
    /// ];
    /// let builder = EmbedBuilder::new().fields(fields);
    /// ```
    pub fn fields(mut self, fields: Vec<embed::EmbedField>) -> Self {
        if self.embed.fields.len() + fields.len() > 25 {
            panic!("Embeds can only contain max of 25 fields");
        }

        self.embed.fields.extend(fields.into_iter());
        self
    }
}

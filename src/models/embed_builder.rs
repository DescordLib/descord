use crate::consts::color::Color;

use super::embed;

pub struct EmbedBuilder {
    embed: embed::Embed,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            embed: Default::default(),
        }
    }

    pub fn build(self) -> embed::Embed {
        self.embed
    }

    pub fn title(mut self, title: &str) -> Self {
        self.embed.title = Some(title.to_owned());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.embed.description = Some(description.to_owned());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.embed.color = Some(color.into());
        self
    }

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

    pub fn image(mut self, image: embed::EmbedImage) -> Self {
        self.embed.image = Some(image);
        self
    }

    pub fn thumbnail(mut self, thumbnail: embed::EmbedThumbnail) -> Self {
        self.embed.thumbnail = Some(thumbnail);
        self
    }

    pub fn video(mut self, video: embed::EmbedVideo) -> Self {
        self.embed.video = Some(video);
        self
    }

    pub fn author(mut self, author: embed::EmbedAuthor) -> Self {
        self.embed.author = Some(author);
        self
    }

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

    pub fn fields(mut self, fields: Vec<embed::EmbedField>) -> Self {
        if self.embed.fields.len() + fields.len() > 25 {
            panic!("Embeds can only contain max of 25 fields");
        }

        self.embed.fields.extend(fields.into_iter());
        self
    }
}

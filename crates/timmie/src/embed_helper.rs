use poise::serenity_prelude::CreateEmbed;
use serde::{Deserialize, Serialize};

use crate::{Context, Error};

#[derive(Deserialize, Serialize)]
pub struct Embed {
    title: String,
    description: Option<String>,
    fields: Option<Vec<Field>>,
}

#[derive(Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub body: String,
    pub inline: bool,
}

#[derive(Clone, Copy)]
pub enum EmbedTextType {
    Title,
    Description,
    FieldName,
    FieldBody,
}

pub enum Colors {
    Resin = 0x3f90e0,
}

impl Embed {
    pub fn from_file(file: &str) -> Self {
        match crate::read_resource(file) {
            Ok(embed) => embed,
            Err(_) => Embed::error_embed(),
        }
    }

    pub fn fill_placeholder<S: ToString>(
        &mut self,
        placeholder: &str,
        text: &S,
        text_type: EmbedTextType,
    ) {
        match text_type {
            EmbedTextType::Title => {
                self.title = self.title.replace(placeholder, &text.to_string());
            }
            EmbedTextType::Description => {
                if let Some(desc) = &self.description {
                    self.description = Some(desc.replace(placeholder, &text.to_string()));
                };
            }
            EmbedTextType::FieldName => todo!(),
            EmbedTextType::FieldBody => todo!(),
        };
    }

    pub fn fill_placeholders<S: ToString>(
        &mut self,
        placeholders: Vec<&str>,
        text: Vec<&S>,
        text_type: EmbedTextType,
    ) {
        for (placeholder, text) in placeholders.iter().zip(text) {
            self.fill_placeholder(placeholder, text, text_type);
        }
    }

    pub fn build(self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title(self.title);
        if let Some(desc) = self.description {
            embed.description(desc);
        }
        if let Some(fields) = self.fields {
            for field in fields {
                embed.field(field.name, field.body, field.inline); // Should properly implement IntoIterator so embed.fields can be used.
            }
        }

        embed
    }

    pub async fn send(self, ctx: Context<'_>) -> Result<(), Error> {
        ctx.send(|r| {
            r.embed(|e| {
                *e = self.build();
                e
            })
        })
        .await?;
        Ok(())
    }

    fn error_embed() -> Self {
        Embed {
            title: String::from("Couldn't read embed from the file."),
            description: None,
            fields: None,
        }
    }
}

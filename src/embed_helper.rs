use poise::serenity_prelude::CreateEmbed;
use serde::{Deserialize, Serialize};

use crate::{Context, Error, Resources};

#[derive(Deserialize, Serialize)]
pub struct Embed {
    title: String,
    fields: Option<Vec<Field>>,
}

#[derive(Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub body: String,
    pub inline: bool,
}

pub enum Colors {
    Resin = 0x3f90e0,
}

impl Embed {
    pub fn from_file(file: &str) -> Self {
        let file_str: String = match Resources::get(file) {
            Some(file) => match std::str::from_utf8(&file.data) {
                Ok(string) => string.to_owned(),
                Err(_) => return Embed::error_embed(),
            },
            None => return Embed::error_embed(),
        };

        match serde_json::from_str(&file_str) {
            Ok(embed) => embed,
            Err(_) => Embed::error_embed(),
        }
    }

    pub fn build(self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.title(self.title);
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
            fields: None,
        }
    }
}

use serde::Deserialize;
use strum::{Display, EnumIter, IntoEnumIterator};

use super::api;

#[derive(Deserialize)]
pub struct CharacterInfo {
    name: String,
    title: Option<String>,
    vision: String, // Could be an enum later, for utility.
    weapon: String, // Could be an enum later, again, for utitliy.
    nation: String, // Could be an enum later, again, for utitliy.
    affiliation: String,
    rarity: u8,
    constellation: String,
    birthday: Option<String>,
    description: String,
}

#[derive(EnumIter, Display)]
pub enum Character {
    Albedo,
    Aloy,
    Amber,
    #[strum(serialize = "Arataki Itto")]
    AratakiItto,
    Ayaka,
    Ayato,
    Barbara,
    Beidou,
    Bennett,
    Chongyun,
    Collei,
    Diluc,
    Diona,
    Eula,
    Fischl,
    Ganyu,
    Gorou,
    HuTao,
    Jean,
    Kaeya,
    Kazuha,
    Keqing,
    Klee,
    Kokomi,
    #[strum(serialize = "Kuki Shinobu")]
    KukiShinobu,
    Lisa,
    Mona,
    Ningguang,
    Noelle,
    Qiqi,
    Raiden,
    Razor,
    Rosaria,
    Sara,
    Sayu,
    Shenhe,
    #[strum(serialize = "Shikanoin Heizou")]
    ShikanoinHeizou,
    Sucrose,
    Tartaglia,
    Thoma,
    Tighnari,
    #[strum(serialize = "Traveler - Anemo")]
    TravelerAnemo,
    #[strum(serialize = "Traveler - Dendro")]
    TravelerDendro,
    #[strum(serialize = "Traveler - Electro")]
    TravelerElectro,
    #[strum(serialize = "Traveler - Geo")]
    TravelerGeo,
    Venti,
    Xiangling,
    Xiao,
    Xingqui,
    #[strum(serialize = "Yae Miko")]
    YaeMiko,
    Yanfei,
    Yelan,
    Yoimiya,
    #[strum(serialize = "Yun Jin")]
    YunJin,
    Zhongli,
}

impl CharacterInfo {
    pub async fn get(name: &str) -> Result<CharacterInfo, reqwest::Error> {
        let info = api::get_character_info(name).await?;
        Ok(info)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_title(&self) -> String {
        match &self.title {
            Some(title) if !title.is_empty() => format!("\"{}\"", title),
            _ => String::new(),
        }
    }

    pub fn get_vision(&self) -> &str {
        &self.vision
    }

    pub fn get_weapon(&self) -> &str {
        &self.weapon
    }

    pub fn get_rarity(&self) -> u8 {
        self.rarity
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

impl Character {
    pub fn starts_with(string: &str) -> Vec<String> {
        let mut characters = Vec::new();
        for character in Character::iter() {
            if character.to_string().starts_with(string) {
                characters.push(character.to_string());
            }
        }
        characters
    }
}

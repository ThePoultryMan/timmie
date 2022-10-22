use serde::Deserialize;
use strum::{Display, EnumIter, IntoEnumIterator};

use super::api;

#[derive(Deserialize)]
pub struct CharacterInfo {
    name: String,
}

#[derive(EnumIter, Display)]
pub enum Character {
    Albedo,
    Aloy,
    Amber,
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
    ShikanoinHeizou,
    Sucrose,
    Tartaglia,
    Thoma,
    Tighnari,
    TravelerAnemo,
    TravelerDendro,
    TravelerElectro,
    TravelerGeo,
    Venti,
    Xiangling,
    Xiao,
    Xingqui,
    YaeMiko,
    Yanfei,
    Yelan,
    Yoimiya,
    YunJin,
    Zhongli,
}

impl CharacterInfo {
    pub async fn get(name: &str) -> Result<CharacterInfo, reqwest::Error> {
        let info = api::get_character_info(name).await?;
        Ok(info)
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

use super::character::CharacterInfo;

pub async fn get_character_info(name: &str) -> Result<CharacterInfo, reqwest::Error> {
    let info = reqwest::get("https://api.genshin.dev/characters/".to_owned() + name)
        .await?
        .json::<CharacterInfo>()
        .await?;
    Ok(info)
}

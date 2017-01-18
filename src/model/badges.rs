use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct BadgeSets
{
    pub badge_sets: HashMap<String, BadgeVersions>,
}

#[derive(Deserialize, Debug)]
pub struct BadgeVersions
{
    pub versions: HashMap<String, Badge>,
}

#[derive(Deserialize, Debug)]
pub struct Badge
{
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub description: String,
    pub title: String,
    pub click_action: String,
    pub click_url: String,
}

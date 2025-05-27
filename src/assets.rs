use macroquad::prelude::{Font, Texture2D};
use macroquad::text::load_ttf_font;
use macroquad::texture::load_texture;
use std::collections::HashMap;

pub struct Fonts {
    pub mono: Option<Font>,
    pub text: Option<Font>,
    pub text_bold: Option<Font>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum AssetId {
    WoodAnim1,
    WoodAnim2,
    CookingAnim1,
    CookingAnim2,
    Mining1,
    Mining2,
    Hunting1,
    Hunting2,
    Smithing1,
    Smithing2,
    Wood,
    MeatGame,
    Coin,
    BagOfCoins,
    Bread,
    Herbs,
    Sandwich,
    Tree,
    Deer,
}

impl AssetId {
    pub fn texture(&self, assets: &Assets) -> Texture2D {
        assets.textures.get(self).expect("Couldn't find texture").clone()
    }
}

fn asset_paths() -> Vec<(AssetId, &'static str)> {
    vec![
        (AssetId::WoodAnim1, "ChopChop_1_.png"),
        (AssetId::WoodAnim2, "ChopChop_2_.png"),
        (AssetId::CookingAnim1, "pan_1.png"),
        (AssetId::CookingAnim2, "pan_2.png"),
        (AssetId::Mining1, "ClingCling_1.png"),
        (AssetId::Mining2, "ClingCling_2.png"),
        (AssetId::Hunting1, "PewPew_1.png"),
        (AssetId::Hunting2, "PewPew_2.png"),
        (AssetId::Smithing1, "BomBom_1.png"),
        (AssetId::Smithing2, "BomBom_2.png"),
        (AssetId::Wood, "chatgpt/wood.png"),
        (AssetId::MeatGame, "chatgpt/game.png"),
        (AssetId::Coin, "coin.png"),
        (AssetId::BagOfCoins, "chatgpt/bag_of_coins.png"),
        (AssetId::Bread, "chatgpt/bread.png"),
        (AssetId::Herbs, "chatgpt/herbs.png"),
        (AssetId::Sandwich, "chatgpt/sandwich.png"),
        (AssetId::Tree, "chatgpt/tree.png"),
        (AssetId::Deer, "chatgpt/deer.png"),
    ]
}

pub async fn load_assets() -> Assets {
    use futures::future::join_all;

    let paths = asset_paths();

    let futures = paths.iter().map(|(_, path)| load_texture(path));
    let results = join_all(futures).await;

    // Zip results into a HashMap<AssetId, Texture2D>
    let texture_map: HashMap<AssetId, Texture2D> = paths
        .into_iter()
        .zip(results)
        .map(|((assetId, _), res)| {
            let texture = res.expect("Failed to load asset");
            (assetId, texture)
        })
        .collect();

    let fonts = Fonts {
        mono: Some(load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font")),
        text: Some(load_ttf_font("WorkSans-Regular.ttf").await.expect("Couldn't load font")),
        text_bold: Some(load_ttf_font("WorkSans-SemiBold.ttf").await.expect("Couldn't load font")),
    };

    Assets { fonts, textures: texture_map }
}

pub struct Assets {
    pub fonts: Fonts,
    pub textures: HashMap<AssetId, Texture2D>
}

impl Assets {
    //
}
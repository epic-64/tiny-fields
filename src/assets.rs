use futures::future::join_all;
use macroquad::prelude::{Font, Texture2D};
use macroquad::text::load_ttf_font;
use macroquad::texture::load_texture;
use std::collections::HashMap;
use futures::join;

pub struct Fonts {
    pub mono: Font,
    pub text: Font,
    pub text_bold: Font,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum FontId {
    MonoBold,
    TextRegular,
    TextBold,
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
    FontMonoBold,
    FontTextRegular,
    FontTextBold,
}

impl AssetId {
    pub fn texture(&self, assets: &Assets) -> Texture2D {
        assets.textures.get(self).expect("Couldn't find texture").clone()
    }
}

fn texture_paths() -> Vec<(AssetId, &'static str)> {
    vec![
        (AssetId::WoodAnim1, "ChopChop_1_.png"),
        (AssetId::WoodAnim2, "chop2_lanczos.png"),
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

pub async fn load_textures() -> HashMap<AssetId, Texture2D> {
    let paths = texture_paths();
    let futures = paths.iter().map(|(_, path)| load_texture(path));
    let results = join_all(futures).await; // only wait once

    paths
        .into_iter()
        .zip(results)
        .map(|((assetId, _), res)| {
            let texture = res.expect("Failed to load asset");
            (assetId, texture)
        })
        .collect()
}

async fn load_fonts() -> HashMap<FontId, Font> {
    let paths = vec![
        (FontId::MonoBold, "Lekton-Bold.ttf"),
        (FontId::TextRegular, "WorkSans-Regular.ttf"),
        (FontId::TextBold, "WorkSans-SemiBold.ttf"),
    ];

    let futures = paths.iter().map(|(_, path)| load_ttf_font(path));
    let results = join_all(futures).await;

    paths.into_iter().zip(results)
        .map(|((font_id, _), res)| {
            let font = res.expect("Failed to load font");
            (font_id, font)
        })
        .collect()
}

pub async fn load_assets() -> Assets {
    let (texture_map, font_map) = join!(load_textures(), load_fonts());

    let fonts = Fonts {
        mono: font_map.get(&FontId::MonoBold).expect("Couldn't find Mono font").clone(),
        text: font_map.get(&FontId::TextRegular).expect("Couldn't find Text font").clone(),
        text_bold: font_map.get(&FontId::TextBold).expect("Couldn't find Text bold font").clone(),
    };

    Assets {
        fonts,
        textures: texture_map,
    }
}

pub struct Assets {
    pub fonts: Fonts,
    pub textures: HashMap<AssetId, Texture2D>,
}
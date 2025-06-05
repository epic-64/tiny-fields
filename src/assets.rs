use futures::future::join_all;
use futures::join;
use macroquad::prelude::{Font, Texture2D};
use macroquad::text::load_ttf_font;
use macroquad::texture::load_texture;
use std::collections::HashMap;

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
    // Backgrounds
    BackgroundParchment,

    // Icons
    LockIcon,

    // Skill Icons
    LumberingIcon,
    MiningIcon,

    // Animations
    WoodAnim1,
    WoodAnim2,
    CookingAnim1,
    CookingAnim2,
    HerbalismAnim1,
    HerbalismAnim2,
    AlchemyAnim1,
    AlchemyAnim2,
    MiningAnim1,
    MiningAnim2,
    HuntingAnim1,
    HuntingAnim2,
    SmithingAnim1,
    SmithingAnim2,

    // Icons and Items
    // Wood
    Kindlewood,
    Craftwood,
    Graintree,

    // Ores
    IronOre,

    MeatGame,
    Coin,
    BagOfCoins,
    Bread,
    Herbs,
    Sandwich,
    Tree,
    Deer,
    ManaPotion,
}

impl AssetId {
    pub fn texture(&self, assets: &Assets) -> Texture2D {
        assets.textures.get(self).unwrap_or(&Texture2D::empty()).clone()
    }
}

fn texture_paths() -> Vec<(AssetId, &'static str)> {
    vec![
        // Backgrounds
        (AssetId::BackgroundParchment, "chatgpt/parchment.png"),

        // Icons
        (AssetId::LockIcon, "chatgpt/icons/lock.png"),

        // Skill Icons
        (AssetId::LumberingIcon, "chatgpt/skills/woodcutting.png"),
        (AssetId::MiningIcon, "chatgpt/skills/mining.png"),
        
        // Animations
        (AssetId::WoodAnim1, "ChopChop_1_.png"),
        (AssetId::WoodAnim2, "chop2_lanczos.png"),
        (AssetId::CookingAnim1, "pan_1.png"),
        (AssetId::CookingAnim2, "pan_2.png"),
        (AssetId::HerbalismAnim1, "ary/HerbHerb_1.png"),
        (AssetId::HerbalismAnim2, "ary/HerbHerb_2.png"),
        (AssetId::AlchemyAnim1, "ary/Alchemy_1.png"),
        (AssetId::AlchemyAnim2, "ary/Alchemy_2.png"),
        (AssetId::MiningAnim1, "ClingCling_1.png"),
        (AssetId::MiningAnim2, "ClingCling_2.png"),
        (AssetId::HuntingAnim1, "PewPew_1.png"),
        (AssetId::HuntingAnim2, "PewPew_2.png"),
        (AssetId::SmithingAnim1, "BomBom_1.png"),
        (AssetId::SmithingAnim2, "BomBom_2.png"),
        
        // Wood
        (AssetId::Kindlewood, "chatgpt/kindlewood.png"),
        (AssetId::Craftwood, "chatgpt/craftwood.png"),
        (AssetId::Graintree, "chatgpt/graintree.png"),

        // Ores
        (AssetId::IronOre, "chatgpt/iron_ore.png"),
        
        (AssetId::MeatGame, "chatgpt/game.png"),
        (AssetId::Coin, "coin.png"),
        (AssetId::BagOfCoins, "chatgpt/bag_of_coins.png"),
        (AssetId::Bread, "chatgpt/bread.png"),
        (AssetId::Herbs, "chatgpt/herbs.png"),
        (AssetId::Sandwich, "chatgpt/sandwich.png"),
        (AssetId::Tree, "chatgpt/tree.png"),
        (AssetId::Deer, "chatgpt/deer.png"),
        (AssetId::ManaPotion, "chatgpt/mana_potion.png"),
    ]
}

pub async fn load_textures() -> HashMap<AssetId, Texture2D> {
    let paths = texture_paths();
    let futures = paths.iter().map(|(_, path)| load_texture(path));
    let results = join_all(futures).await; // only wait once

    paths
        .into_iter()
        .zip(results)
        .map(|((asset_id, _), res)| {
            let texture = res.expect("Failed to load asset");
            (asset_id, texture)
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
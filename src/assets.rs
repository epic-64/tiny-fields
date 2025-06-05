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
    HuntingIcon,
    HerbalismIcon,
    CookingIcon,

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
        (AssetId::HuntingIcon, "chatgpt/skills/hunting.png"),
        (AssetId::HerbalismIcon, "chatgpt/skills/herbalism.png"),
        (AssetId::CookingIcon, "chatgpt/skills/cooking.png"),
        
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

    let mut textures = vec![];

    for (asset_id, path) in paths {
        let texture = load_texture(path).await.expect(&format!("Failed to load texture: {}", path));
        textures.push((asset_id, texture));
    }

    HashMap::from_iter(textures)
}

pub async fn load_assets() -> Assets {
    let texture_map = load_textures().await;

    let fonts = Fonts {
        mono: load_ttf_font("Lekton-Bold.ttf").await.expect("Couldn't find Mono font"),
        text: load_ttf_font("WorkSans-Regular.ttf").await.expect("Couldn't find Text font"),
        text_bold: load_ttf_font("WorkSans-SemiBold.ttf").await.expect("Couldn't find Text bold font"),
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
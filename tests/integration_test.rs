use macroquad::file::set_pc_assets_folder;
use macroquad::prelude::{load_texture, load_ttf_font, Texture2D};
use tiny_fields::game::{Assets, Fonts, GameState, Intent, Textures};

#[macroquad::test]
async fn it_works() {
    set_pc_assets_folder("assets");

    let hut1: Texture2D = load_texture("hut1.png").await.expect("Couldn't load file");
    let hut2: Texture2D = load_texture("hut2.png").await.expect("Couldn't load file");
    let wood_1: Texture2D = load_texture("ChopChop_1_.png").await.expect("Couldn't load file");
    let wood_2: Texture2D = load_texture("ChopChop_2_.png").await.expect("Couldn't load file");
    let frame1: Texture2D = load_texture("frame2.png").await.expect("Couldn't load file");
    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1 };

    let main_font = load_ttf_font("Menlo-Regular.ttf").await.expect("Couldn't load font");
    let fonts = Fonts { main: main_font };

    let assets = Assets { fonts, textures };

    let mut game_state = GameState::new(assets);
    let intents: Vec<Intent> = vec![];

    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.total_money, 0);
}
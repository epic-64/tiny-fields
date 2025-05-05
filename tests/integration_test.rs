use macroquad::prelude::Texture2D;
use tiny_fields::game::{Assets, Fonts, GameState, Intent, Textures};

#[macroquad::test]
async fn it_works() {
    let hut1: Texture2D = Texture2D::empty();
    let hut2: Texture2D = Texture2D::empty();
    let wood_1: Texture2D = Texture2D::empty();
    let wood_2: Texture2D = Texture2D::empty();
    let frame1: Texture2D = Texture2D::empty();
    let textures = Textures { hut1, hut2, wood_1, wood_2, frame1 };

    let fonts = Fonts { main: None };
    let assets = Assets { fonts, textures };

    let mut game_state = GameState::new(assets);
    let intents: Vec<Intent> = vec![];

    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.total_money, 0);
}
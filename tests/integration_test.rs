use tiny_fields::game::{GameState, Intent, Item};

#[test]
fn it_works() {
    let mut game_state = GameState::new();
    let intents: Vec<Intent> = vec![];

    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.inventory.get_item_amount(&Item::Coin), 0);
}
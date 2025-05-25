use tiny_fields::game::{GameState, Intent, Item};
use tiny_fields::game::Intent::ToggleJob;

#[test]
fn it_works() {
    let mut game_state = GameState::new();
    let intents: Vec<Intent> = vec![];

    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.inventory.get_item_amount(Item::Coin), 0);
}

#[test]
fn toggle_job() {
    let mut game_state = GameState::new();

    assert_eq!(game_state.jobs[0].running, false);

    let intents = vec![ToggleJob(0)];
    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.jobs[0].running, true);
}
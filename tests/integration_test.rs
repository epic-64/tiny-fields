use std::collections::HashSet;
use strum::IntoEnumIterator;
use tiny_fields::game::{GameState, Intent, Item};
use tiny_fields::job::{JobArchetype, JobArchetypeInstances};

#[test]
fn it_works() {
    let mut game_state = GameState::new();
    let intents: Vec<Intent> = vec![];

    game_state.step(intents.as_slice(), 0.016);

    assert_eq!(game_state.inventory.get_item_amount(&Item::Coin), 0);
}

#[test]
fn all_job_archetypes_are_instantiated() {
    let job_instances = JobArchetypeInstances::new();

    let actual: HashSet<_> = job_instances
        .instances
        .iter()
        .map(|inst| inst.job_archetype)
        .collect();

    let expected: HashSet<_> = JobArchetype::iter().collect();

    assert_eq!(
        actual, expected,
        "Mismatch between expected and actual job archetypes.\nMissing: {:?}\nExtra: {:?}",
        expected.difference(&actual).collect::<Vec<_>>(),
        actual.difference(&expected).collect::<Vec<_>>()
    );
}
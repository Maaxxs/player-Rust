use crate::models::{game_state::GameState, player_action::PlayerAction};

pub fn decide(game_state: GameState) -> Vec<PlayerAction> {
    // TODO: place your player logic here.
    vec![PlayerAction {
        src: 0,
        dest: 0,
        amount: 0,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn decide_test() {
        let want = vec![PlayerAction::default()];

        let result = decide(GameState::default());

        assert!(want == result)
    }

    #[test]
    fn parse_game_state() {
        let json_str = read_to_string("./example_game_state.json").unwrap();

        let _json: GameState = serde_json::from_str(&json_str).unwrap();
    }
}

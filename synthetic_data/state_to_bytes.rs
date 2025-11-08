use splendor::game_state::GameState;
use splendor::card::card::Card;
use splendor::resource::Resource;
use splendor::state_encoder::StateEncoder;

fn add_card_to_state(state: &mut Vec<u8>, card: Option<&&Card>) {
    if let Some(card) = card {
        state.push(card.n_points());
        state.push(card.cost().n_green());
        state.push(card.cost().n_red());
        state.push(card.cost().n_blue());
        state.push(card.cost().n_black());
        state.push(card.cost().n_white());
        match card.production() {
            Resource::Green => state.append(&mut vec![1, 0, 0, 0, 0]),
            Resource::Red => state.append(&mut vec![0, 1, 0, 0, 0]),
            Resource::Blue => state.append(&mut vec![0, 0, 1, 0, 0]),
            Resource::White => state.append(&mut vec![0, 0, 0, 1, 0]),
            Resource::Black => state.append(&mut vec![0, 0, 0, 0, 1]),
        }
    } else {
        state.push(0);
        state.push(0);
        state.push(0);
        state.push(0);
        state.push(0);
        state.push(0);
        state.append(&mut vec![0, 0, 0, 0, 0]);
    }
}

pub fn game_state_to_bytes(game_state: &GameState, encoder: &dyn StateEncoder) -> Vec<u8> {
    let mut output = Vec::new();

    // Add players in turn order (current player first)
    let players_in_order: Vec<_> = game_state
        .get_players()
        .iter()
        .chain(game_state.get_players().iter())
        .collect();

    let current_idx = game_state.get_current_player_index();
    let n_players = game_state.get_players().len();

    for player in &players_in_order[current_idx..current_idx + n_players] {
        output.push(player.get_points());

        let resources = player.get_resources();
        output.push(resources.n_green());
        output.push(resources.n_red());
        output.push(resources.n_blue());
        output.push(resources.n_black());
        output.push(resources.n_white());
        output.push(resources.n_gold());

        let production = player.get_production();
        output.push(production.n_green());
        output.push(production.n_red());
        output.push(production.n_blue());
        output.push(production.n_black());
        output.push(production.n_white());

        // Add reserved cards (up to 3)
        for i in 0..3 {
            add_card_to_state(&mut output, player.get_reserve().get(i));
        }
    }

    for row_index in 0..3 {
        let row = game_state.get_board().get_rows().get_row(row_index);
        output.extend(encoder.encode_row(row));
    }
    output
}

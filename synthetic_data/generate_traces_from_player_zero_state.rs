use splendor::game_state::GameState;
use splendor::moves::all_moves::get_all_moves;

/// Generate traces from a player 0 state by exploring all valid moves
/// Returns a list of tuples (child_state, traces) where:
///   - child_state: immediate result of a move from player_zero_state
///   - traces: all possible traces (sequences of states) starting from that child_state
pub fn generate_traces_from_player_zero_state(
    player_zero_state: &GameState,
    n_players: u8,
) -> Vec<(GameState, Vec<Vec<GameState>>)> {
    let all_moves = get_all_moves();
    let mut result: Vec<(GameState, Vec<Vec<GameState>>)> = Vec::new();

    // Generate all child states from player_zero_state
    for valid_move in all_moves
        .iter()
        .filter(|m| m.is_valid(player_zero_state)) {
        let child_state = valid_move.perform(player_zero_state);

        // For each child, generate all traces of length (n_players - 1)
        let mut traces_from_child: Vec<Vec<GameState>> = vec![vec![]];

        for _ in 1..n_players {
            let mut new_traces = Vec::new();

            for trace in traces_from_child {
                // Determine current state: if trace is empty, use child_state
                let current_state = if trace.is_empty() {
                    &child_state
                } else {
                    trace.last().unwrap()
                };

                for valid_move in all_moves
                    .iter()
                    .filter(|m| m.is_valid(current_state)) {
                    let new_state = valid_move.perform(current_state);
                    let mut new_trace = trace.clone();
                    new_trace.push(new_state);
                    new_traces.push(new_trace);
                }
            }

            traces_from_child = new_traces;
        }

        result.push((child_state, traces_from_child));
    }

    result
}


use splendor::game_state::GameState;

use crate::constants::WINNING_POINTS;
use crate::generate_traces_from_player_zero_state::generate_traces_from_player_zero_state;
use crate::getters::{get_last_player_points, get_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationResult {
    Winning,
    Losing,
    Draw,
}

impl EvaluationResult {
    /// Convert evaluation result to label: Winning=1, Losing=-1, Draw=0
    pub fn to_label(self) -> i8 {
        match self {
            EvaluationResult::Winning => 1,
            EvaluationResult::Losing => -1,
            EvaluationResult::Draw => 0,
        }
    }
}

/// Evaluate if a player 0 state is winning, losing, or draw
pub fn evaluate_player_zero_state(player_zero_state: &GameState, n_players: u8, max_depth: u8) -> EvaluationResult {
    if max_depth == 0 {
        return EvaluationResult::Draw;
    }
    let traces = generate_traces_from_player_zero_state(player_zero_state, n_players);

    let mut all_children_losing = true;
    let mut has_draw_child = false;
    let mut player_zero_states_to_recurse: Vec<GameState> = Vec::new();

    // Evaluate each child state and its traces
    for (child_state, trace_list) in traces {
        let child_value = get_value(&child_state, n_players);

        // Check if child has winning points and is better than all states in its traces
        if child_value.0 >= WINNING_POINTS {
            let is_better_than_all = trace_list.iter().all(|trace| {
                trace.iter().all(|state| {
                    let state_value = get_value(state, n_players);
                    child_value > state_value
                })
            });

            if is_better_than_all {
                return EvaluationResult::Winning;
            }

            // Check if child has equal value to any state in traces (only when game ends)
            let has_equal_state = trace_list.iter().any(|trace| {
                trace.iter().any(|state| {
                    let state_value = get_value(state, n_players);
                    child_value == state_value
                })
            });

            if has_equal_state {
                has_draw_child = true;
            }
        }

        // Check if child is losing (has a state in its traces that's better)
        let has_better_state = trace_list.iter().any(|trace| {
            trace.iter().any(|state| {
                get_last_player_points(state, n_players) >= WINNING_POINTS
            })
        });

        if !has_better_state {
            all_children_losing = false;

            // Collect last states from traces for recursion
            for trace in trace_list {
                if let Some(last_state) = trace.last() {
                    if last_state.get_current_player_index() == 0 {
                        player_zero_states_to_recurse.push(last_state.clone());
                    }
                }
            }
        }
    }

    if all_children_losing {
        return EvaluationResult::Losing;
    }

    // Recurse on collected player 0 states
    for state in player_zero_states_to_recurse {
        let evaluation_result = evaluate_player_zero_state(&state, n_players, max_depth - 1);
        if evaluation_result == EvaluationResult::Winning {
            return EvaluationResult::Winning;
        }
        if evaluation_result == EvaluationResult::Draw {
            has_draw_child = true;
        }
    }
    if has_draw_child {
        return EvaluationResult::Draw;
    }
    EvaluationResult::Losing
}

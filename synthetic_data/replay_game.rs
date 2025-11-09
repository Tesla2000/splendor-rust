use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use splendor::game_state::{create_initial_game_state, GameState};
use splendor::moves::all_moves::get_all_moves;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn decode_move_index(index: usize) -> String {
    match index {
        0 => "Get 3: Green, Blue, Red".to_string(),
        1 => "Get 3: Green, Blue, White".to_string(),
        2 => "Get 3: Green, Blue, Black".to_string(),
        3 => "Get 3: Green, Red, White".to_string(),
        4 => "Get 3: Green, Red, Black".to_string(),
        5 => "Get 3: Green, White, Black".to_string(),
        6 => "Get 3: Blue, Red, White".to_string(),
        7 => "Get 3: Blue, Red, Black".to_string(),
        8 => "Get 3: Blue, White, Black".to_string(),
        9 => "Get 3: Red, White, Black".to_string(),
        10 => "Get 2: Green".to_string(),
        11 => "Get 2: Blue".to_string(),
        12 => "Get 2: Red".to_string(),
        13 => "Get 2: White".to_string(),
        14 => "Get 2: Black".to_string(),
        15..=38 => {
            let offset = index - 15;
            let tier_index = offset / 8;
            let within_tier = offset % 8;
            let card_index = within_tier / 2;
            let is_build = within_tier % 2 == 1;
            if is_build {
                format!("Build card from board (Tier {}, Position {})", tier_index + 1, card_index)
            } else {
                format!("Reserve card from board (Tier {}, Position {})", tier_index + 1, card_index)
            }
        }
        39 => "Reserve from hidden deck (Tier 1)".to_string(),
        40 => "Reserve from hidden deck (Tier 2)".to_string(),
        41 => "Reserve from hidden deck (Tier 3)".to_string(),
        42 => "Build from reserve (Slot 0)".to_string(),
        43 => "Build from reserve (Slot 1)".to_string(),
        44 => "Build from reserve (Slot 2)".to_string(),
        _ => format!("Unknown move index: {}", index),
    }
}

fn load_rng_states_batch(path: &str) -> Result<Vec<ChaCha8Rng>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let rng_states: Vec<ChaCha8Rng> = bincode::deserialize(&buffer)?;
    Ok(rng_states)
}

fn get_rng_from_index(base_seed: u64, index: usize) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(base_seed.wrapping_add(index as u64))
}

fn format_player_state(state: &GameState, player_index: usize) -> String {
    let player = &state.get_players()[player_index];
    let resources = player.get_resources();
    let production = player.get_production();
    format!(
        "Resources=[G:{} B:{} R:{} W:{} B:{} Gold:{}] Production=[G:{} B:{} R:{} W:{} B:{}] Points={}",
        resources.n_green(),
        resources.n_blue(),
        resources.n_red(),
        resources.n_white(),
        resources.n_black(),
        resources.n_gold(),
        production.n_green(),
        production.n_blue(),
        production.n_red(),
        production.n_white(),
        production.n_black(),
        player.get_points()
    )
}

fn play_and_print_moves<R: Rng>(n_players: u8, rng: &mut R, winner_only: bool) -> GameState {
    let mut current_state = create_initial_game_state(n_players, rng);
    let mut move_num = 0;
    let all_moves = get_all_moves();
    let mut move_history: Vec<(usize, usize, String, String)> = Vec::new();
    if !winner_only {
        println!("\n=== Starting Game ===");
        println!("Initial state: {} players", n_players);
    }
    loop {
        move_num += 1;
        let valid_move_indices: Vec<usize> = all_moves
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_valid(&current_state))
            .map(|(i, _)| i)
            .collect();
        if valid_move_indices.is_empty() {
            if !winner_only {
                println!("\nNo valid moves available! Game restarted.");
            }
            move_history.clear();
            return play_and_print_moves(n_players, rng, winner_only);
        }
        let random_index = valid_move_indices[rng.gen_range(0..valid_move_indices.len())];
        let chosen_move = &all_moves[random_index];
        let current_player = current_state.get_current_player_index();
        let move_description = decode_move_index(random_index);
        current_state = chosen_move.perform(&current_state);
        let acting_player = if current_state.get_current_player_index() == 0 {
            n_players - 1
        } else {
            (current_state.get_current_player_index() - 1) as u8
        };
        let player_state = format_player_state(&current_state, acting_player as usize);
        if winner_only {
            move_history.push((move_num, current_player as usize, move_description.clone(), player_state));
        } else {
            println!(
                "Move {}: Player {} - {}",
                move_num, current_player, move_description
            );
            println!("  -> Player {} state: {}", acting_player, player_state);
        }
        let last_player = if current_state.get_current_player_index() == 0 {
            n_players - 1
        } else {
            (current_state.get_current_player_index() - 1) as u8
        };
        let last_player_points = current_state.get_players()[last_player as usize].get_points();
        if last_player_points >= 15 {
            if winner_only {
                println!("\n=== Winner: Player {} ===", last_player);
                println!("Total moves: {}", move_num);
                println!("\nWinner's moves:");
                for (move_n, player, description, state) in move_history.iter() {
                    if *player == last_player as usize {
                        println!("  Move {}: {}", move_n, description);
                        println!("    -> {}", state);
                    }
                }
            } else {
                println!(
                    "\n=== Game Over ===\nPlayer {} wins with {} points after {} moves!",
                    last_player, last_player_points, move_num
                );
                println!("\nFinal state of Player {}:", last_player);
                println!("  {}", format_player_state(&current_state, last_player as usize));
            }
            return current_state;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} <state_index> [base_seed] [--winner-only]", args[0]);
        eprintln!("  {} <rng_states_file> <state_index> [--winner-only]", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} 454 42                                      # Use seed-based RNG (seed 42+454)", args[0]);
        eprintln!("  {} rng_states/rng_states_160000.bin 454        # Load from batch file", args[0]);
        eprintln!("  {} rng_states/rng_states_160000.bin 454 --winner-only  # Show only winner's moves", args[0]);
        std::process::exit(1);
    }
    let winner_only = args.iter().any(|arg| arg == "--winner-only");
    let n_players = 2;
    let mut rng = if args.len() >= 2 && args[1].parse::<usize>().is_ok() {
        let state_index: usize = args[1].parse().expect("state_index must be a valid number");
        let base_seed: u64 = if args.len() >= 3 && args[2] != "--winner-only" {
            args[2].parse().expect("base_seed must be a valid number")
        } else {
            42
        };
        if !winner_only {
            println!("Using seed-based RNG: base_seed={}, index={}", base_seed, state_index);
            println!("Effective seed: {}", base_seed.wrapping_add(state_index as u64));
        }
        get_rng_from_index(base_seed, state_index)
    } else {
        let rng_states_file = &args[1];
        let state_index: usize = args[2].parse().expect("state_index must be a valid number");
        if !Path::new(rng_states_file).exists() {
            eprintln!("Error: File {} does not exist", rng_states_file);
            eprintln!("\nTip: If you don't have the RNG states file, use seed-based mode:");
            eprintln!("  {} {} 42", args[0], state_index);
            std::process::exit(1);
        }
        if !winner_only {
            println!("Loading RNG states from: {}", rng_states_file);
        }
        let rng_states = match load_rng_states_batch(rng_states_file) {
            Ok(states) => states,
            Err(e) => {
                eprintln!("Failed to load RNG states: {}", e);
                std::process::exit(1);
            }
        };
        if !winner_only {
            println!("Loaded {} RNG states", rng_states.len());
        }
        if state_index >= rng_states.len() {
            eprintln!(
                "Error: state_index {} is out of bounds (max: {})",
                state_index,
                rng_states.len() - 1
            );
            std::process::exit(1);
        }
        if !winner_only {
            println!("Using RNG state at index {}", state_index);
        }
        rng_states[state_index].clone()
    };
    play_and_print_moves(n_players, &mut rng, winner_only);
}
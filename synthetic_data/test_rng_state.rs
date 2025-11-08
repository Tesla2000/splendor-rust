use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use splendor::state_encoder::OneHotCardEncoder;
use super::generator::generate_synthetic_data;
use super::rng_state::{save_rng_state, load_rng_state};
use std::fs;

#[test]
fn test_rng_state_determinism() {
    let n_players = 2;
    let n_moves_limit = 69;
    let max_depth = 1;
    let encoder = OneHotCardEncoder::new();
    let seed = 12345u64;
    let mut rng1 = ChaCha8Rng::seed_from_u64(seed);
    let mut rng2 = ChaCha8Rng::seed_from_u64(seed);
    let (states1, labels1, n_moves1) = generate_synthetic_data(
        5,
        n_players,
        &mut rng1,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    let (states2, labels2, n_moves2) = generate_synthetic_data(
        5,
        n_players,
        &mut rng2,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    assert_eq!(states1, states2, "Same seed should produce identical states");
    assert_eq!(labels1, labels2, "Same seed should produce identical labels");
    assert_eq!(n_moves1, n_moves2, "Same seed should produce identical n_moves");
}

#[test]
fn test_rng_state_save_and_load() {
    let test_dir = "test_rng_output";
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    let state_path = format!("{}/test_rng_state.bin", test_dir);
    let n_players = 2;
    let n_moves_limit = 69;
    let max_depth = 1;
    let encoder = OneHotCardEncoder::new();
    let seed = 54321u64;
    let mut rng_original = ChaCha8Rng::seed_from_u64(seed);
    let (_states_first_batch, _labels_first_batch, _n_moves_first_batch) = generate_synthetic_data(
        3,
        n_players,
        &mut rng_original,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    save_rng_state(&rng_original, &state_path)
        .expect("Failed to save RNG state");
    let (states_second_batch, labels_second_batch, n_moves_second_batch) = generate_synthetic_data(
        2,
        n_players,
        &mut rng_original,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    let mut rng_loaded = load_rng_state(&state_path)
        .expect("Failed to load RNG state");
    let (states_loaded_batch, labels_loaded_batch, n_moves_loaded_batch) = generate_synthetic_data(
        2,
        n_players,
        &mut rng_loaded,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    assert_eq!(states_second_batch, states_loaded_batch, "Loaded RNG should produce same states");
    assert_eq!(labels_second_batch, labels_loaded_batch, "Loaded RNG should produce same labels");
    assert_eq!(n_moves_second_batch, n_moves_loaded_batch, "Loaded RNG should produce same n_moves");
    fs::remove_dir_all(test_dir).expect("Failed to cleanup test directory");
}

#[test]
fn test_rng_state_full_sequence() {
    let test_dir = "test_rng_full_sequence";
    fs::create_dir_all(test_dir).expect("Failed to create test directory");
    let n_players = 2;
    let n_moves_limit = 69;
    let max_depth = 1;
    let encoder = OneHotCardEncoder::new();
    let seed = 99999u64;
    let mut rng_continuous = ChaCha8Rng::seed_from_u64(seed);
    let (states_all, labels_all, n_moves_all) = generate_synthetic_data(
        10,
        n_players,
        &mut rng_continuous,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    let mut rng_checkpointed = ChaCha8Rng::seed_from_u64(seed);
    let mut states_checkpointed = Vec::new();
    let mut labels_checkpointed = Vec::new();
    let mut n_moves_checkpointed = Vec::new();
    for i in 0..10 {
        let checkpoint_path = format!("{}/checkpoint_{}.bin", test_dir, i);
        save_rng_state(&rng_checkpointed, &checkpoint_path)
            .expect("Failed to save checkpoint");
        let (states, labels, n_moves) = generate_synthetic_data(
            1,
            n_players,
            &mut rng_checkpointed,
            n_moves_limit,
            max_depth,
            &encoder,
        );
        states_checkpointed.extend(states);
        labels_checkpointed.extend(labels);
        n_moves_checkpointed.extend(n_moves);
    }
    assert_eq!(states_all, states_checkpointed, "Checkpointed generation should match continuous");
    assert_eq!(labels_all, labels_checkpointed, "Checkpointed labels should match continuous");
    assert_eq!(n_moves_all, n_moves_checkpointed, "Checkpointed n_moves should match continuous");
    let mut rng_from_checkpoint_5 = load_rng_state(&format!("{}/checkpoint_5.bin", test_dir))
        .expect("Failed to load checkpoint 5");
    let (states_resume, labels_resume, n_moves_resume) = generate_synthetic_data(
        5,
        n_players,
        &mut rng_from_checkpoint_5,
        n_moves_limit,
        max_depth,
        &encoder,
    );
    assert_eq!(&states_resume, &states_all[5..10], "Resuming from checkpoint 5 should match games 5-9");
    assert_eq!(&labels_resume, &labels_all[5..10], "Resuming labels should match");
    assert_eq!(&n_moves_resume, &n_moves_all[5..10], "Resuming n_moves should match");
    fs::remove_dir_all(test_dir).expect("Failed to cleanup test directory");
}

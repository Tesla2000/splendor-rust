use ndarray::Array2;
use ndarray_npy::WriteNpyExt;
use std::fs::File;

/// Save game states, labels, and number of moves to NumPy format files
///
/// # Arguments
/// * `states` - Vector of game states as Vec<u8>
/// * `labels` - Vector of labels (1 for win, -1 for loss, 0 for tie)
/// * `n_moves` - Vector of number of moves for each state
/// * `states_path` - Path to save states.npy
/// * `labels_path` - Path to save labels.npy
/// * `n_moves_path` - Path to save n_moves.npy
pub fn save_states_with_labels(
    states: Vec<Vec<u8>>,
    labels: Vec<i8>,
    n_moves: Vec<i32>,
    states_path: &str,
    labels_path: &str,
    n_moves_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if states.is_empty() {
        return Ok(());
    }

    assert_eq!(
        states.len(),
        labels.len(),
        "Number of states must equal number of labels"
    );
    assert_eq!(
        states.len(),
        n_moves.len(),
        "Number of states must equal number of n_moves"
    );

    // Save states as 2D array (n_states × state_size)
    let n_states = states.len();
    let state_size = states[0].len();

    // Flatten states into single Vec
    let flat_states: Vec<u8> = states.into_iter().flatten().collect();

    let states_array = Array2::from_shape_vec((n_states, state_size), flat_states)
        .expect("Failed to create states array");

    let mut states_file = File::create(states_path)?;
    states_array.write_npy(&mut states_file)?;

    // Save labels as 1D array
    let labels_array = ndarray::Array1::from_vec(labels);
    let mut labels_file = File::create(labels_path)?;
    labels_array.write_npy(&mut labels_file)?;

    // Save n_moves as 1D array
    let n_moves_array = ndarray::Array1::from_vec(n_moves);
    let mut n_moves_file = File::create(n_moves_path)?;
    n_moves_array.write_npy(&mut n_moves_file)?;

    println!("Saved {} states to {}, {}, and {}", n_states, states_path, labels_path, n_moves_path);

    Ok(())
}

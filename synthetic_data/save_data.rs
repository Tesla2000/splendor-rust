use ndarray::Array2;
use ndarray_npy::WriteNpyExt;
use std::fs::File;
use std::io::Result;

/// Save game states and labels to NumPy format files
///
/// # Arguments
/// * `states` - Vector of game states as Vec<u8>
/// * `labels` - Vector of labels (1 for win, -1 for loss, 0 for tie)
/// * `states_path` - Path to save states.npy
/// * `labels_path` - Path to save labels.npy
pub fn save_states_with_labels(
    states: Vec<Vec<u8>>,
    labels: Vec<i8>,
    states_path: &str,
    labels_path: &str,
) -> Result<()> {
    if states.is_empty() {
        return Ok(());
    }

    assert_eq!(
        states.len(),
        labels.len(),
        "Number of states must equal number of labels"
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

    println!("Saved {} states to {} and {}", n_states, states_path, labels_path);

    Ok(())
}

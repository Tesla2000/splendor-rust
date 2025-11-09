use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn save_rng_state(rng: &ChaCha8Rng, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = bincode::serialize(rng)?;
    let mut file = File::create(path)?;
    file.write_all(&serialized)?;
    Ok(())
}

pub fn load_rng_state(path: &str) -> Result<ChaCha8Rng, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let rng: ChaCha8Rng = bincode::deserialize(&buffer)?;
    println!("Loaded RNG state from {}", path);
    Ok(rng)
}

pub fn save_rng_states_batch(rng_states: &[ChaCha8Rng], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = bincode::serialize(rng_states)?;
    let mut file = File::create(path)?;
    file.write_all(&serialized)?;
    Ok(())
}
pub fn create_or_load_rng(seed: u64, state_path: &str) -> ChaCha8Rng {
    if Path::new(state_path).exists() {
        match load_rng_state(state_path) {
            Ok(rng) => {
                println!("Resuming from saved RNG state");
                rng
            }
            Err(e) => {
                println!("Failed to load RNG state: {}. Creating new RNG from seed {}", e, seed);
                ChaCha8Rng::seed_from_u64(seed)
            }
        }
    } else {
        println!("No saved RNG state found. Creating new RNG from seed {}", seed);
        ChaCha8Rng::seed_from_u64(seed)
    }
}

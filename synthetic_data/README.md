# Synthetic Data Generator

Generates synthetic game data for training machine learning models on Splendor gameplay.

## Usage

```bash
cargo run --bin synthetic_data -- [NUM_GAMES] [SEED] [N_MOVES_LIMIT] [USE_ONE_HOT] [OUTPUT_DIR] [RNG_STATES_DIR]
```

## Arguments

| Position | Name | Type | Default | Description |
|----------|------|------|---------|-------------|
| 1 | `NUM_GAMES` | u32 | 1 | Number of games to generate |
| 2 | `SEED` | u64 | 42 | Random seed for initial RNG state |
| 3 | `N_MOVES_LIMIT` | i32 | 69 | Maximum number of moves per game (games exceeding this are skipped) |
| 4 | `USE_ONE_HOT` | bool | true | Use one-hot encoding (true) or parameter encoding (false) |
| 5 | `OUTPUT_DIR` | String | "." | Directory where .npy data files are saved |
| 6 | `RNG_STATES_DIR` | String | "rng_states" | Directory where RNG state checkpoints are saved |

## Examples

### Basic usage (all defaults)
```bash
cargo run --bin synthetic_data
```
Generates 1 game with seed 42, saves to current directory.

### Generate 1000 games with custom seed
```bash
cargo run --bin synthetic_data -- 1000 12345
```

### Specify output directories
```bash
cargo run --bin synthetic_data -- 5000 42 69 true ./training_data ./checkpoints
```

### Use parameter encoding instead of one-hot
```bash
cargo run --bin synthetic_data -- 1000 42 69 false ./output ./states
```

## Output Files

### Data Files (saved every 1000 games and at completion)
- `{OUTPUT_DIR}/states_{N}.npy` - Game states as byte arrays
- `{OUTPUT_DIR}/labels_{N}.npy` - Labels (-1, 0, 1) for loss/tie/win
- `{OUTPUT_DIR}/n_moves_{N}.npy` - Number of moves per game

Where `{N}` is the checkpoint number (1000, 2000, etc.)

### RNG State Files (saved after each successful game)
- `{RNG_STATES_DIR}/rng_state_{N}.bin` - RNG state after game N

## Resuming Generation

To resume from a specific checkpoint:

1. Copy the desired RNG state to `rng_state_0.bin`:
   ```bash
   cp rng_states/rng_state_1000.bin rng_states/rng_state_0.bin
   ```

2. Run the generator:
   ```bash
   cargo run --bin synthetic_data -- 1000 42 69 true ./output ./rng_states
   ```

The generator will load `rng_state_0.bin` and continue from that point.

## Features

- **Deterministic**: Same seed produces identical results
- **Resumable**: Can resume from any saved RNG checkpoint
- **Memory efficient**: Clears data every 1000 games
- **Progress tracking**: Shows progress every 100 games

## Testing

Run the test suite to verify RNG state functionality:

```bash
cargo test --bin synthetic_data
```

The tests verify:
- Deterministic generation with same seed
- Correct save/load of RNG states
- Checkpoint-based generation matches continuous generation
- Resuming from checkpoints produces identical results

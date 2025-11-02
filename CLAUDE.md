# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust implementation of the board game Splendor with Python bindings via PyO3. The project is designed as a library (`cdylib`) that exposes a Python module for use in Python-based AI/ML applications (e.g., reinforcement learning).

## Build and Test Commands

```bash
# Build the project (creates Python extension)
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Run a single test
cargo test test_name

# Run the standalone Rust binary (for development/testing)
cargo run
```

## Architecture Overview

### Core Design Pattern: Immutable Game State with Builder Pattern

The game uses an **immutable state pattern** where:
- `GameState` is immutable and read-only
- All moves create new `GameState` instances rather than modifying existing ones
- `GameStateBuilder` (and similar builder structs) provide a mutable intermediate representation for constructing new states

This pattern ensures safe state transitions and makes the game suitable for tree search algorithms.

### Key Architectural Components

**1. Move System (`src/moves/`)**
- All possible moves are pre-generated in `all_moves.rs` using the `AllMoves` struct
- There are exactly 42 possible move types across all game situations:
  - 10 "get three different resources" combinations
  - 5 "get two of same resource" moves
  - 12 reserve moves (3 tiers × 4 visible cards)
  - 12 build moves (3 tiers × 4 visible cards)
  - 3 reserve-from-hidden moves (one per tier)
- The `Move` trait (in `move_trait.rs`) defines the interface:
  - `is_valid(&self, game_state: &GameState) -> bool`: Check if move is legal
  - `perform(&self, game_state: &GameState) -> GameState`: Execute move and return new state
  - `finalize()`: Advances to next player (default implementation)

**2. State Management**
- `GameState` (in `game_state.rs`): Core immutable state containing players, board, and current player index
- `GameStateBuilder`: Mutable builder for constructing new states during move execution
- Similar builder pattern used for `Board`, `Player`, `Resources`, and `Rows`

**3. Board Structure**
- `Board` (in `board/board.rs`): Contains resources pool, card rows, and aristocrats
- `Rows` (in `board/rows/`): Manages three tiers of visible cards (4 cards per tier)
- Cards are referenced by `CardReference` with tier and position indices

**4. Player State**
- Players track owned cards (deck), resources, reserved cards (max 3), and acquired aristocrats
- `get_production()`: Calculates resource production from owned cards
- Resources limited to 10 tokens total per player

**5. Python Interface (`src/lib.rs`)**

The `SplendorGame` PyO3 class exposes:
- `new(n_players, seed=None)`: Initialize game with deterministic seed
- `get_valid_moves()`: Returns indices of valid moves
- `apply_move(move_index, seed=None)`: Apply move by index, returns new game instance
- `get_game_state()`: Returns flattened u8 vector representation for neural networks
- Game state methods: `is_game_over()`, `get_winner()`, `get_all_player_points()`

**6. Static Data Storage**

Cards and aristocrats use static storage patterns:
- `CARD_STORAGE` and `ARISTOCRAT_STORAGE` contain all game data as static references
- Players/board store indices or references (`&'static Card`) rather than copying data
- This reduces memory overhead and allows cheap cloning of game states

### State Representation for ML

The `get_game_state()` method in `lib.rs:185` returns a flattened byte vector suitable for neural network input. The state is structured with:
- Players in turn order (current player first)
- For each player: points, resources (6 types), production (5 types), reserved cards (3 slots)
- Board state: 3 rows × 4 cards with encoded card data

## Development Notes

### Randomness and Seeding
- Uses `ChaCha8Rng` for deterministic randomness
- Seeds can be passed to `SplendorGame::new()` and `apply_move()` for reproducibility
- Random number generator used during board initialization for card shuffling

### Module Organization
- Core game logic in root modules: `card`, `resource`, `player`, `resources`, `aristocrat`
- Move implementations in `moves/` directory
- Board-related code in `board/` with sub-modules for rows
- Both `lib.rs` (Python binding) and `main.rs` (Rust binary) entry points exist

### Working with Moves

When adding or modifying moves:
1. Implement the `Move` trait in a new file under `src/moves/`
2. Add instances to `AllMoves::new()` in `all_moves.rs`
3. Use `GameStateBuilder` pattern to construct new states
4. Remember to call `finalize()` to advance to next player

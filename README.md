# Rust Chess Engine

A chess engine written from scratch in Rust as a learning project.

## Features

- Board representation using a 64-square array
- FEN parsing
- Move execution
- Legal move generation
- Attack and check detection
- Checkmate and stalemate detection
- Castling
- En passant
- Pawn promotion
- Perft and divide utilities for move generation validation
- Perft validated through depth 6
- Tapered position evaluation
  - Material evaluation
  - Middlegame and endgame piece-square tables
  - Game-phase interpolation
- Negamax search
- Checkmate and stalemate handling during search
- Unit test coverage

## Planned Features

- Alpha-beta pruning
- Move ordering
- 50-move rule
- Threefold repetition
- Quiescence search
- Transposition tables
- UCI support
- GUI

## Running

```bash
cargo run
```

## Testing

Run the test suite with:

```bash
cargo test
```

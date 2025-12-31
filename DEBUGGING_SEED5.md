# Seed 5 PRNG Divergence Investigation

## Problem
Seed 5 has 59 PRNG calls (Rust) vs 93 PRNG calls (JavaScript) across 20 turns.
- Difference: Rust is missing 34 PRNG calls
- Both battles complete 20 turns without ending

## Divergence Pattern

### Per-Turn Comparison:

| Turn | Rust | JS | Diff | Rust Total | JS Total |
|------|------|----|----|------------|----------|
| 1    | 4    | 7  | -3 | 4          | 7        |
| 2    | 4    | 7  | -3 | 8          | 14       |
| 3    | 4    | 7  | -3 | 12         | 21       |
| 4    | 4    | 7  | -3 | 16         | 28       |
| 5    | 4    | 7  | -3 | 20         | 35       |
| 6    | 4    | 7  | -3 | 24         | 42       |
| 7    | 4    | 7  | -3 | 28         | 49       |
| 8    | 4    | 7  | -3 | 32         | 56       |
| 9    | 5    | 5  | 0  | 37         | 61       |
| 10   | 3    | 6  | -3 | 40         | 67       |
| 11   | 0    | 6  | -6 | 40         | 73       |
| 12   | 3    | 3  | 0  | 43         | 76       |
| 13   | 0    | 0  | 0  | 43         | 76       |
| 14   | 4    | 3  | +1 | 47         | 79       |
| 15   | 0    | 3  | -3 | 47         | 82       |
| 16   | 3    | 0  | +3 | 50         | 82       |
| 17   | 0    | 3  | -3 | 50         | 85       |
| 18   | 6    | 3  | +3 | 56         | 88       |
| 19   | 0    | 0  | 0  | 56         | 88       |
| 20   | 3    | 5  | -2 | 59         | 93       |

## Analysis

**Key Findings:**
- Turns 1-8: Consistently missing 3 PRNG calls per turn
- Turn 9: Matches perfectly
- Turns 10-20: Erratic pattern, sometimes missing, sometimes extra

**Hypothesis:** Something is happening in turns 1-8 that makes 3 extra PRNG calls in JavaScript but not in Rust.

## Investigation Steps

1. Check which moves are being used in turns 1-8
2. Look for abilities/items that might trigger
3. Compare PRNG value sequences to find exact divergence point

# conservation-composer

[![crates.io](https://img.shields.io/crates/v/conservation-composer.svg)](https://crates.io/crates/conservation-composer)
[![docs.rs](https://docs.rs/conservation-composer/badge.svg)](https://docs.rs/conservation-composer)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Idea

A ii-V-I progression sounds "right" because the voice leading minimizes spectral energy loss between chords. The transitions preserve the maximum amount of harmonic information. This isn't aesthetic — it's geometric.

`conservation-composer` treats chord progressions as walks on a graph where edge weights are spectral similarity. It generates progressions by **simulated annealing**: start random, then iteratively swap chords to maximize a conservation constraint (eigenvalue matching, spectral gap, voice-leading distance). The result is progressions that sound like they "resolve" because they mathematically do.

## How It Works

### 1. Build the chord graph

Every chord is a node. Edges connect chords that are "reachable" (within voice-leading distance). The edge weight is the spectral distance — how different the chords sound.

### 2. Define a conservation constraint

What makes a "good" progression? You choose:
- **Spectral gap**: the difference between the largest and second-largest eigenvalues of the transition matrix. Large gap = strong tonal center.
- **Algebraic connectivity** (Fiedler value): how well-connected the progression graph is. High = every chord feels related.
- **Voice-leading conservation**: total semitone distance minimized.

### 3. Simulated annealing

Start with a random 4- or 8-chord progression. At each step:
1. Swap one chord for another
2. Compute the new conservation score
3. If it's better, accept. If worse, accept with probability e^(-ΔE/T) where T is the temperature
4. Decrease T (cool down)
5. Repeat until frozen

```rust
use conservation_composer::{Composer, ComposeStyle};

let mut composer = Composer::new(ComposeStyle::Jazz, 4);
let progression = composer.compose(); // 4-chord progression

for chord in &progression.chords {
    println!("{}", chord); // e.g., "Dm7 → G7 → Cmaj7 → Fmaj7"
}
```

### Quick compose shortcut

```rust
use conservation_composer::{compose::quick_compose, ComposeStyle};

let prog = quick_compose(ComposeStyle::Jazz, 4).unwrap();
println!("Spectral conservation: {:.3}", prog.conservation_score());
```

## The Graph Laplacian Connection

The transition matrix of a chord progression is a Markov chain. Its eigenvalues tell you about the structure:
- **λ₁ = 1** (always, it's a Markov chain)
- **λ₂ (spectral gap)**: how fast the chain "forgets" its starting chord. Large gap = the progression has a clear tonic.
- **Fiedler value** of the graph Laplacian: measures how easily the progression splits into two keys. Small value = ambiguous tonality (could be major or minor). Large value = strong key center.

## Style Presets

| Style | What it constrains |
|---|---|
| `Jazz` | Seventh chords, wide harmonic palette, moderate voice leading |
| `Classical` | Triads only, strict voice leading, no tritone moves |
| `Minimalist` | Limited chord palette, lots of repetition, slow harmonic rhythm |
| `Free` | Any chord, any voice leading — pure spectral optimization |

## Module Map

| Module | What it does |
|---|---|
| `chord` | `Chord`, `ChordQuality` — chord representation (triads through 13ths) |
| `progression` | `ChordProgression`, `KeySignature`, `KeyMode` — a sequence of chords with key context |
| `laplacian` | `build_laplacian`, `build_transition_matrix` — graph construction for the chord network |
| `spectral` | `eigenvalues`, `spectral_gap`, `algebraic_connectivity` — eigen-analysis of the transition matrix |
| `annealer` | `Annealer`, `ConservationConstraint` — the simulated annealing engine |
| `compose` | `Composer`, `ComposeStyle` — high-level composition API |
| `style` | Style presets and quick-compose shortcuts |

## Design Decisions

- **Why simulated annealing over gradient descent?** The chord space is discrete (you can't have 2.7 chords). SA naturally handles discrete spaces with a proven theoretical guarantee of convergence to near-optimal solutions.
- **Why eigenvalue matching?** Eigenvalues of the transition matrix capture global structure that individual chord pairs don't. A progression where each adjacent pair is smooth but the overall arc doesn't resolve has a different eigenvalue profile than one with a clear tonic.
- **Why not use PLR directly?** groovemesh-plr generates valid *adjacent* chord pairs. But a 4-chord progression needs global coherence, not just local smoothness. Conservation-composer optimizes the whole sequence.

## Links

- [Documentation](https://docs.rs/conservation-composer)
- [Repository](https://github.com/SuperInstance/conservation-composer)
- [crates.io](https://crates.io/crates/conservation-composer)
- See also: [groovemesh-plr](https://crates.io/crates/groovemesh-plr) for local PLR transformations, [heat-spectral](https://crates.io/crates/heat-spectral) for the spectral graph theory

## License

MIT

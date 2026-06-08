# conservation-composer

[![crates.io](https://img.shields.io/crates/v/conservation-composer.svg)](https://crates.io/crates/conservation-composer)
[![docs.rs](https://docs.rs/conservation-composer/badge.svg)](https://docs.rs/conservation-composer)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Algorithmic composition maximizing spectral conservation.**

A jazz ii-V-I progression is mathematically optimal because the voice-leading
distance between chords minimizes spectral energy loss. `conservation-composer`
generates chord progressions whose transition matrix has eigenvalues matching
conservation constraints, using simulated annealing over a graph-Laplacian
representation.

The result: progressions that feel "inevitable" — each chord flows naturally
to the next because the spectral energy is conserved across transitions.

## Features

- **Spectral analysis** — eigenvalue decomposition of chord transition matrices,
  algebraic connectivity, and spectral gap computation
- **Graph Laplacian representation** — chord progressions modeled as weighted
  graphs with `build_laplacian()` and `build_transition_matrix()`
- **Simulated annealing** — `Annealer` with configurable temperature schedule,
  acceptance criteria, and `ConservationConstraint` settings
- **Style presets** — `ComposeStyle::Jazz`, `Classical`, `Minimalist`,
  `Atonal` with style-specific voice-leading and harmonic rules
- **Rich chord model** — `Chord` with `ChordQuality` (Major, Minor, Dominant7,
  Minor7, Major7, Diminished, Augmented, Sus2, Sus4)
- **Progression API** — `ChordProgression` with key signatures, modes, and
  chord sequence management
- **Quick compose** — one-call `quick_compose()` for immediate results

## Quick Start

```rust
use conservation_composer::{compose::ComposeStyle, style::quick_compose};

// Generate a 4-chord jazz progression
let progression = quick_compose(ComposeStyle::Jazz, 4).unwrap();

for chord in &progression.chords {
    println!("{}", chord);
}
```

## Custom Composition

```rust
use conservation_composer::{Composer, ComposeStyle, ConservationConstraint, Annealer};

let mut composer = Composer::new(ComposeStyle::Jazz)
    .length(8)
    .key(0)           // C
    .constraint(ConservationConstraint::SpectralGap { min: 0.5 });

let result = composer.compose().unwrap();
println!("Spectral gap: {:.3}", result.spectral_gap());
```

## Module Overview

| Module | Description |
|---|---|
| `chord` | `Chord`, `ChordQuality` — chord representation |
| `progression` | `ChordProgression`, `KeySignature`, `KeyMode` — progression model |
| `spectral` | `eigenvalues()`, `spectral_gap()`, `algebraic_connectivity()` |
| `laplacian` | `build_laplacian()`, `build_transition_matrix()` — graph construction |
| `annealer` | `Annealer`, `ConservationConstraint` — optimization engine |
| `compose` | `Composer`, `ComposeStyle` — high-level composition API |
| `style` | `quick_compose()` — one-call composition presets |
| `error` | `ComposeError` — error types |

## Links

- [Documentation](https://docs.rs/conservation-composer)
- [Repository](https://github.com/nightshift-crates/conservation-composer)
- [Crates.io](https://crates.io/crates/conservation-composer)

## License

MIT

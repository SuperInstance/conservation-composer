# conservation-composer

**Music composition governed by the spectral conservation laws of graph theory — graph Laplacian eigenvalues mapped to harmony, rhythm, and dynamics.**

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Problem

Music theory describes harmony in terms of intervals, scales, and chord progressions — but these are ultimately *mathematical structures* expressed in a particular domain. What if you could compose music directly from *any* mathematical structure? A social network, a road map, a protein interaction graph — anything with nodes and edges. The question is: what is the *natural* mapping from graph topology to musical parameters that preserves structural meaning?

## The Key Insight

The eigenvalues of the graph Laplacian are a natural musical scale:

- **λ₁ = 0** is the drone — the constant vector that never changes, the fundamental pitch that anchors everything.
- **λ₂** (algebraic connectivity) is the tonic — the tonal center. The Fiedler vector (its eigenvector) partitions the graph into two communities, which become the "strong" and "weak" halves of the beat.
- **λ₃, λ₄, λ₅** are chord tones — they define the harmonic backbone, mapped to scale degrees based on their magnitude relative to the spectral radius.
- **λ₆+** are overtones — higher partials that add color and tension, mapped to higher octaves.

The **Conservation Ratio** CR = λ₂/λₙ controls the entire musical style:
- **CR > 0.7**: Well-connected graph → consonant major key → slow tempo (100 BPM)
- **0.3 < CR < 0.7**: Moderate connectivity → jazz with extended chords → medium tempo (130 BPM)
- **CR < 0.3**: Sparse graph with bottlenecks → atonal chromatic → fast tempo (160 BPM)

This isn't arbitrary. The CR measures how "spread out" the eigenvalue spectrum is. A concentrated spectrum (high CR) means all modes interact similarly — this is musical *consonance*. A spread spectrum (low CR) means some modes dominate while others are suppressed — this is *dissonance* and *tension*.

## Architecture

```
┌──────────────────────────┐
│   Graph Topology          │    Nodes + edges → adjacency matrix
│   (any graph structure)   │
└─────────────┬────────────┘
              │ Build Laplacian L = D - A
┌─────────────▼────────────┐
│   Jacobi Eigenvalue       │    λ₁ = 0, λ₂ = algebraic connectivity,
│   Decomposition           │    Fiedler vector partitions graph
└─────────────┬────────────┘
              │
     ┌────────┼────────────┐
     │        │            │
┌────▼───┐ ┌──▼────┐ ┌─────▼──────┐
│ CR     │ │ Pitch │ │ Rhythm     │
│ → Style│ │ λ→MIDI│ │ Fiedler    │
│ → Scale│ │ →scale│ │ →duration  │
│ → BPM  │ │ →note │ │ →prob      │
└────┬───┘ └──┬────┘ └─────┬──────┘
     │        │            │
┌────▼────────▼────────────▼──────┐
│   Musical Score                  │
│   Vec<NoteEvent>                 │
│   beat / MIDI / duration / vel   │
└─────────────────────────────────┘
              │
     ┌────────┼────────────┐
     │        │            │
┌────▼──┐ ┌───▼───┐ ┌─────▼────┐
│Rust   │ │Web App│ │MIDI/WAV  │
│API    │ │(HTML) │ │Export    │
└───────┘ └───────┘ └──────────┘
```

## Quick Start

```rust
use conservation_composer::{Graph, compose, generate_score};

// Build a graph — any topology works
let graph = Graph::cycle(8);

// Compose: eigenvalues → musical parameters
let params = compose(&graph);

println!("CR = {:.4} → {}", params.cr, params.style_name());
println!("Key: {} at {} BPM", params.key_name(), params.bpm);

// Generate a score: 16 beats of music
let events = generate_score(&params, 16);
for event in &events {
    println!("  beat {} | MIDI {} | dur {:.2}", event.beat, event.midi, event.duration_beats);
}
```

## Tutorial

### Graph Topologies → Musical Styles

```rust
use conservation_composer::{Graph, compose};

// Complete graph: maximum connectivity → consonant, major
let params = compose(&Graph::complete(6));
assert!(params.cr > 0.5);
// → "Consonant / Major" at 100 BPM

// Path graph: minimal connectivity → atonal, experimental
let params = compose(&Graph::path(8));
assert!(params.cr < 0.3);
// → "Atonal / Experimental" at 160 BPM

// Barbell: two cliques + bridge → dramatic tension
let params = compose(&Graph::barbell(4));
// Bridge creates bottleneck → low CR → chromatic scale

// Jazz ii-V-I: three chord clusters with transitions
let params = compose(&Graph::jazz_ii_v_i());
// Moderate CR → jazz scale, extended harmonies
```

### Understanding the Notes

```rust
let params = compose(&Graph::cycle(6));

for note in &params.notes {
    println!(
        "{:?}: {} (MIDI {}, {:.1} Hz) amp={:.2}",
        note.role, note.name, note.midi, note.freq, note.amplitude
    );
}
// Drone: C3 (MIDI 48, 130.8 Hz) amp=0.15
// Root:  C4 (MIDI 60, 261.6 Hz) amp=0.25
// Chord: E4 (MIDI 64, 329.6 Hz) amp=0.12
// ...
```

### The Fiedler Vector and Rhythm

```rust
let params = compose(&Graph::barbell(5));

// The Fiedler vector splits the graph into two halves.
// Positive values → sustained notes; Negative → staccato
if let Some(fiedler) = params.decomp.fiedler_vector() {
    for (i, &val) in fiedler.iter().enumerate() {
        let dur = params.notes[i].duration_mult;
        println!("node {}: Fiedler={:+.3} → duration={:.2}", i, val, dur);
    }
}
```

Run the full tutorial: `cargo run --example tutorial`

## Interactive Version

Open `index.html` in a browser for the interactive visual composer:
- Click canvas to add nodes, shift+drag to add edges
- Watch eigenvalues update in real-time
- Play compositions directly via Web Audio API
- Export to WAV

## API Reference

| Type / Function | Description |
|---|---|
| `Graph::new(n)` | Empty graph with n nodes |
| `Graph::path(n)` | Chain topology |
| `Graph::cycle(n)` | Ring topology |
| `Graph::complete(n)` | Fully connected |
| `Graph::star(n)` | Hub-spoke |
| `Graph::barbell(m)` | Two m-cliques + bridge |
| `Graph::jazz_ii_v_i()` | ii-V-I chord structure |
| `Graph::add_edge(a, b, w)` | Add weighted edge |
| `Graph::laplacian()` | Laplacian matrix L = D - A |
| `compose(&graph)` | Graph → MusicalParams |
| `generate_score(&params, beats)` | Params → Vec<NoteEvent> |
| `MusicalParams` | CR, BPM, key, scale, notes |
| `MusicalNote` | MIDI, freq, amplitude, role, eigenvalue |
| `NoteEvent` | beat, MIDI, duration, velocity |
| `NoteRole` | Drone / Root / Chord / Overtone |
| `EigenDecomposition` | eigenvalues, eigenvectors, CR, Fiedler |
| `midi_to_freq(midi)` | MIDI → Hz |
| `midi_to_name(midi)` | MIDI → "C4" |
| `freq_to_midi(freq)` | Hz → MIDI |

## Why Graph Topology Determines Harmony

The graph Laplacian L captures how "energy" flows through the network. Its eigenvalues encode the resonant frequencies of the graph:

1. **λ₁ = 0**: The DC component — uniform distribution. In music, this is the drone: a continuous bass note that never changes.

2. **λ₂ (Fiedler value)**: The fundamental mode. The Fiedler vector divides the graph into two optimal partitions. In music, this is the tonic/home key — the most stable state.

3. **Higher eigenvalues**: Increasingly oscillatory modes. Each one adds a new "frequency" to the spectrum. In music, these are the overtones that define timbre and harmony.

4. **CR = λ₂/λₙ**: The spectral spread. When CR is high (close to 1), all modes have similar energy — the graph is well-mixed and the music is consonant. When CR is low, some modes dominate — the graph has bottlenecks and the music is tense and dissonant.

This mapping is not arbitrary — it's the **natural frequency response of the graph**, translated into the domain of musical pitch.

## Ecosystem Role

**conservation-composer** is the creative application layer in the SuperInstance spectral analysis framework:

- **[heat-spectral](https://github.com/SuperInstance/heat-spectral)** — Heat diffusion (parabolic PDE) on graphs
- **conservation-composer** — Spectral composition *(this crate)*
- **[spectral-fingerprint](https://github.com/SuperInstance/spectral-fingerprint)** — Spectral code similarity
- **[conservation-protocol](https://github.com/SuperInstance/conservation-protocol)** — Conservation law protocol
- **[conservation-geometry](https://github.com/SuperInstance/conservation-geometry)** — Geometric analysis

## License

MIT

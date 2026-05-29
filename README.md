# Conservation Composer

**🎵 Graph Laplacian → Jazz Composition**

An interactive browser application that transforms graph structures into music using spectral graph theory. Draw a graph, and its Laplacian eigenvalues become notes, the Fiedler vector shapes rhythm, and the conservation ratio determines the harmonic style — from consonant major to chromatic atonality.

## How It Works

### Spectral → Musical Mapping

| Spectral Feature | Musical Parameter |
|-----------------|-------------------|
| λ₁ (zero eigenvalue) | Bass drone / root |
| λ₂ (spectral gap) | Tonal center |
| λ₃–λ₅ (mid eigenvalues) | Chord tones |
| λ₆+ (higher eigenvalues) | Overtones / tension |
| Conservation ratio (λ₂/λₙ) | Harmonic style |
| Fiedler vector values | Note duration |
| Eigenvalue magnitudes | Note amplitudes |

### Conservation Ratio → Style

- **CR > 0.7** → Consonant, major scale (well-connected graph)
- **0.3 < CR ≤ 0.7** → Jazz, extended chords (moderate structure)
- **CR ≤ 0.3** → Atonal, chromatic (sparse/chaotic graph)

## Features

### Graph Editor
- **Click canvas** → add node
- **Drag node** → reposition
- **Shift+drag** node→node → add edge
- **Right-click** node → remove
- **Scroll** → adjust BPM

### Presets
- ⭐ **Star Graph** — Central hub with satellite nodes
- 🔗 **Complete Graph** — Every node connected to every other
- 🎷 **Jazz ii-V-I** — Musical chord progression as graph
- 🌪️ **Random Chaos** — Random graph for atonal exploration

### Audio
- **Web Audio API** — real-time synthesis, no dependencies
- **WAV export** — save compositions as audio files
- **Live playback** with visual spectrum bar
- **Dynamic BPM** — conservation ratio controls tempo

### Real-Time Analysis
- Conservation ratio meter with color-coded fill
- Eigenvalue list with note mapping
- Fiedler vector display
- Style classification badge

## Technical Architecture

### Linear Algebra
- **Jacobi eigenvalue algorithm** for symmetric matrix decomposition
- Laplacian construction: L = D - A
- Fiedler vector: eigenvector corresponding to λ₂
- All computation runs client-side in the browser

### Audio Engine
- Built on Web Audio API (OscillatorNode, GainNode)
- Each eigenvalue → one oscillator with mapped frequency
- Fiedler vector controls gain envelope duration
- Play/stop with scheduled note events
- WAV export via OfflineAudioContext

### Visualization
- HTML5 Canvas for graph rendering
- Animated spectrum bar showing eigenvalue distribution
- Real-time UI updates on graph changes
- Responsive layout with sidebar controls

## Quick Start

1. Open `index.html` in any modern browser
2. Click one of the presets (try "🎷 Jazz ii-V-I")
3. Press **▶ Play** to hear the graph as music
4. Add/remove nodes and edges to hear how topology changes the composition
5. Click **💾 Export WAV** to save

No server, no build step, no dependencies.

## Connection to Conservation Spectral Framework

Conservation Composer is the **artistic application** of the conservation spectral framework. Where conservation-geometry makes the math visible, this repo makes it *audible*. The conservation ratio that predicts ecosystem resilience and code stability now predicts harmonic consonance. The spectral gap that measures graph connectivity now sets the tempo. This isn't sonification — it's structural music, where the graph *is* the score.

## Testing

This is a pure client-side HTML/JS application. To verify functionality:

1. Open `index.html` in a browser
2. Load the "Complete Graph" preset with 4 nodes
3. Verify eigenvalues display: [0, 4, 4, 4]
4. Verify CR ≈ 1.0 (λ₂/λₙ = 4/4)
5. Click Play and verify audio output
6. Try Export WAV and verify file downloads

Core math functions (`buildLaplacian`, `eigenDecomp`, `computeConservationRatio`, `getMusicalParams`) are pure functions that could be extracted for unit testing.

## License

MIT

---

Part of the [OpenConstruct](https://github.com/SuperInstance) ecosystem — where spectral graph theory becomes music.

## Related Repos

- [conservation-spectral-v2](https://github.com/SuperInstance/conservation-spectral-v2) — Reference Python implementation
- [conservation-geometry](https://github.com/SuperInstance/conservation-geometry) — Geometric visualizations
- [spectral-spreadsheet](https://github.com/SuperInstance/spectral-spreadsheet) — Interactive spectral spreadsheet
- [conservation-art](https://github.com/SuperInstance/conservation-art) — Artistic visualizations

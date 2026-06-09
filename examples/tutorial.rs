//! Tutorial: Understanding Conservation Composition
//!
//! A guided walkthrough of how graph topology becomes music.
//!
//! Run with: `cargo run --example tutorial`

use conservation_composer::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   Conservation Composer Tutorial: Graphs → Music            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // ── Lesson 1: The Conservation Ratio ────────────────────────
    println!("━━━ Lesson 1: The Conservation Ratio (CR = λ₂/λₙ) ━━━\n");
    println!("CR measures how 'uniformly connected' a graph is.");
    println!("It controls the entire musical style:\n");
    println!("  CR > 0.7 → Consonant / Major    (complete graphs, expanders)");
    println!("  0.3-0.7 → Jazz / Extended       (cycles, regular graphs)");
    println!("  CR < 0.3 → Atonal / Experimental (paths, barbells)\n");

    let topologies: Vec<(&str, Graph)> = vec![
        ("Path(8)", Graph::path(8)),
        ("Cycle(8)", Graph::cycle(8)),
        ("Star(8)", Graph::star(8)),
        ("Complete(8)", Graph::complete(8)),
        ("Barbell(4)", Graph::barbell(4)),
    ];

    println!("  {:<15} {:>8} {:>20}", "Graph", "CR", "Style");
    println!("  {}", "─".repeat(46));
    for (name, g) in &topologies {
        let params = compose(g);
        println!("  {:<15} {:>8.4} {:>20}", name, params.cr, params.style_name());
    }
    println!();

    // ── Lesson 2: Eigenvalues → Pitch ──────────────────────────
    println!("━━━ Lesson 2: Eigenvalues Become Pitches ━━━\n");
    println!("Each eigenvalue maps to a note:\n");
    println!("  λ₁ = 0    → Drone (bass pedal, always present)");
    println!("  λ₂        → Root / Tonic (the tonal center)");
    println!("  λ₃, λ₄, λ₅ → Chord tones (harmonic backbone)");
    println!("  λ₆, λ₇, … → Overtones (color and tension)\n");

    let g = Graph::cycle(6);
    let params = compose(&g);
    println!("  Cycle(6) spectral composition:\n");
    for note in &params.notes {
        let bar = "█".repeat((note.amplitude * 50.0) as usize);
        println!(
            "  λ_{}={:6.3} → {:>3} ({:>6} Hz) {}",
            note.eigen_index + 1,
            note.eigen_value,
            note.name,
            format!("{:.1}", note.freq),
            bar
        );
    }
    println!();

    // ── Lesson 3: Fiedler Vector → Rhythm ──────────────────────
    println!("━━━ Lesson 3: The Fiedler Vector Controls Rhythm ━━━\n");
    println!("The Fiedler vector (eigenvector of λ₂) partitions the graph\n");
    println!("into two communities. In music, this becomes:\n");
    println!("  Positive Fiedler → long, sustained notes");
    println!("  Negative Fiedler → short, staccato notes\n");

    let g = Graph::barbell(4);
    let params = compose(&g);
    println!("  Barbell(4) — note durations from Fiedler vector:\n");
    if let Some(fiedler) = params.decomp.fiedler_vector() {
        for (i, &val) in fiedler.iter().enumerate() {
            let dur = params.notes.get(i).map(|n| n.duration_mult).unwrap_or(0.0);
            let bar = if val >= 0.0 {
                "░".repeat((val * 30.0) as usize + 1)
            } else {
                "▓".repeat((-val * 30.0) as usize + 1)
            };
            println!(
                "  node {}: Fiedler={:+.3} → dur={:.2} {}",
                i, val, dur, bar
            );
        }
    }
    println!();

    // ── Lesson 4: Graph → Complete Composition ─────────────────
    println!("━━━ Lesson 4: Complete Composition Pipeline ━━━\n");

    let g = Graph::jazz_ii_v_i();
    let params = compose(&g);
    println!("  Jazz ii-V-I Graph:\n");
    println!("  CR = {:.4} → {}", params.cr, params.style_name());
    println!("  Key: {} at {} BPM", params.key_name(), params.bpm);
    println!();

    let events = generate_score(&params, 16);
    println!("  Generated {} events over 16 beats:\n", events.len());

    // Group events by beat
    let mut beat_groups: std::collections::BTreeMap<u32, Vec<&NoteEvent>> =
        std::collections::BTreeMap::new();
    for event in &events {
        beat_groups.entry(event.beat).or_default().push(event);
    }

    for (beat, notes) in beat_groups.iter().take(16) {
        let names: Vec<String> = notes
            .iter()
            .map(|e| {
                let idx = e.note_index;
                let name = params.notes.get(idx).map(|n| n.name.clone()).unwrap_or("?".into());
                name
            })
            .collect();
        println!("  beat {:>2}: {}", beat, names.join(", "));
    }
    println!();

    // ── Lesson 5: Compare Topologies ───────────────────────────
    println!("━━━ Lesson 5: Same Process, Different Graphs ━━━\n");
    println!("  Compare the harmonic content of different topologies:\n");

    for (name, g) in &[
        ("Path(6)", Graph::path(6)),
        ("Cycle(6)", Graph::cycle(6)),
        ("Complete(6)", Graph::complete(6)),
    ] {
        let params = compose(g);
        let chord: Vec<String> = params
            .notes
            .iter()
            .filter(|n| matches!(n.role, NoteRole::Chord | NoteRole::Root))
            .map(|n| n.name.clone())
            .collect();
        println!(
            "  {:<12} CR={:.3} → {} (chord: {})",
            name,
            params.cr,
            params.style_name(),
            chord.join("-")
        );
    }

    println!("\n✅ Tutorial complete! Explore the interactive version at index.html");
}

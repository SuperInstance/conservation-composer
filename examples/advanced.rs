//! Advanced Example: Real-world composition workflows
//!
//! Demonstrates generating complete scores, analyzing harmonic content,
//! and exploring the relationship between topology and musical style.
//!
//! Run with: `cargo run --example advanced`

use conservation_composer::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   Advanced Conservation Composer                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ── Application 1: Harmonic Analysis ────────────────────────
    println!("━━━ Application 1: Harmonic Analysis of Graph Families ━━━\n");

    for (name, graph) in [
        ("Path(8)", Graph::path(8)),
        ("Cycle(8)", Graph::cycle(8)),
        ("Star(8)", Graph::star(8)),
        ("Complete(8)", Graph::complete(8)),
        ("Barbell(4)", Graph::barbell(4)),
        ("Jazz ii-V-I", Graph::jazz_ii_v_i()),
    ] {
        let params = compose(&graph);

        // Build a "chord" from the non-drone, non-overtone notes
        let chord_notes: Vec<&MusicalNote> = params
            .notes
            .iter()
            .filter(|n| matches!(n.role, NoteRole::Root | NoteRole::Chord))
            .collect();

        let chord_names: Vec<&str> = chord_notes.iter().map(|n| n.name.as_str()).collect();
        let freqs: Vec<f64> = chord_notes.iter().map(|n| n.freq).collect();

        // Compute frequency ratios relative to root
        let root_freq = params.notes[1].freq;
        let ratios: Vec<f64> = freqs.iter().map(|f| f / root_freq).collect();

        println!("  {} {}", name, "─".repeat(40 - name.len()));
        println!("    CR: {:.4} | BPM: {} | {}", params.cr, params.bpm, params.style_name());
        println!("    Chord: {}", chord_names.join(" - "));
        println!("    Freq ratios from root: {:?}", ratios.iter().map(|r| format!("{:.3}", r)).collect::<Vec<_>>());
        println!();
    }

    // ── Application 2: Full Score Generation ────────────────────
    println!("━━━ Application 2: Full Score Generation (32 beats) ━━━\n");

    let graph = Graph::jazz_ii_v_i();
    let params = compose(&graph);
    let events = generate_score(&params, 32);

    println!("  Graph: Jazz ii-V-I | CR: {:.4} | Key: {} | BPM: {}",
             params.cr, params.key_name(), params.bpm);
    println!("  Total events: {}\n", events.len());

    // Piano roll view
    let mut min_midi = 127u8;
    let mut max_midi = 0u8;
    for event in &events {
        min_midi = min_midi.min(event.midi);
        max_midi = max_midi.max(event.midi);
    }

    println!("  MIDI range: {} to {} ({} semitones)", min_midi, max_midi, max_midi - min_midi);

    // Count notes per beat
    let mut notes_per_beat: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for event in &events {
        *notes_per_beat.entry(event.beat).or_insert(0) += 1;
    }
    let max_density = notes_per_beat.values().copied().max().unwrap_or(0);

    println!("\n  Density (notes/beat):");
    for beat in 0..32 {
        let count = notes_per_beat.get(&beat).copied().unwrap_or(0);
        let bar = "█".repeat(count);
        let empty = "░".repeat(max_density.saturating_sub(count));
        if beat % 4 == 0 {
            print!("\n  {:>2}|{}{}", beat, bar, empty);
        } else {
            print!("{}{}", bar, empty);
        }
    }
    println!("\n");

    // ── Application 3: Eigenvalue Spectrum as Frequency Spectrum ─
    println!("━━━ Application 3: Eigenvalue Spectrum ━━━\n");
    println!("  The eigenvalue spectrum IS the frequency spectrum of the graph.\n");

    for (name, graph) in [
        ("Path(8)", Graph::path(8)),
        ("Cycle(8)", Graph::cycle(8)),
        ("Complete(8)", Graph::complete(8)),
    ] {
        let params = compose(&graph);
        let max_eig = params.decomp.eigenvalues.last().copied().unwrap_or(1.0);
        println!("  {} eigenvalues:", name);
        for (i, &val) in params.decomp.eigenvalues.iter().enumerate() {
            let bar_len = if max_eig > 0.0 { (val / max_eig * 40.0) as usize } else { 0 };
            let bar: String = if i == 0 { "·".repeat(1) } else { "█".repeat(bar_len) };
            println!("    λ_{}={:7.4} {}", i + 1, val, bar);
        }
        println!();
    }

    // ── Application 4: Comparing Scores ─────────────────────────
    println!("━━━ Application 4: Score Comparison ━━━\n");
    println!("  How does the same graph sound with different node counts?\n");

    for n in [4, 6, 8, 10] {
        let g = Graph::cycle(n);
        let params = compose(&g);
        let unique_notes: std::collections::HashSet<u8> =
            params.notes.iter().map(|n| n.midi).collect();
        let events = generate_score(&params, 16);
        println!(
            "  Cycle({}): {} voices, {} unique pitches, {} events/16 beats",
            n,
            params.notes.len(),
            unique_notes.len(),
            events.len()
        );
    }

    println!("\n✅ Advanced examples complete.");
}

//! Basic example: compose music from different graph topologies
//!
//! Run with: `cargo run --example basic`

use conservation_composer::*;

fn main() {
    println!("=== Conservation Composer: Graph → Music ===\n");

    let graphs: Vec<(&str, Graph)> = vec![
        ("Path(8) — contemplative", Graph::path(8)),
        ("Cycle(8) — cyclical", Graph::cycle(8)),
        ("Complete(6) — rich", Graph::complete(6)),
        ("Star(6) — asymmetrical", Graph::star(6)),
        ("Barbell(4) — dramatic", Graph::barbell(4)),
        ("Jazz ii-V-I — harmonic", Graph::jazz_ii_v_i()),
    ];

    for (name, graph) in &graphs {
        let params = compose(graph);
        println!("── {} ──", name);
        println!("  CR:    {:.4} → {}", params.cr, params.style_name());
        println!("  Key:   {}", params.key_name());
        println!("  BPM:   {}", params.bpm);
        println!("  Notes: {} voices", params.notes.len());
        for note in &params.notes {
            println!(
                "    {:>6} | {:>3} ({:>6} Hz) | amp {:.2} | dur {:.2} | λ={:.4}",
                note.role,
                note.name,
                format!("{:.1}", note.freq),
                note.amplitude,
                note.duration_mult,
                note.eigen_value
            );
        }
        println!();
    }

    // Generate a score
    println!("── Generating Score (Cycle 8, 16 beats) ──");
    let g = Graph::cycle(8);
    let params = compose(&g);
    let events = generate_score(&params, 16);
    println!("  {} note events generated", events.len());
    for event in events.iter().take(20) {
        println!(
            "    beat {:>2} | MIDI {:>3} | dur {:.2} | vel {:.2}",
            event.beat, event.midi, event.duration_beats, event.velocity
        );
    }
    if events.len() > 20 {
        println!("    ... and {} more events", events.len() - 20);
    }
}

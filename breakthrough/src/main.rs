// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use breakthrough::kernel::{
    Bootstrap, CoherenceEngine, EcosystemRuntime, EventField, EvolutionEngine,
    ResonanceEngine, Scheduler, TopologyEngine,
};
use breakthrough::memory::{AttractorMemoryState, MemoryRecall, SemanticConcept};
use breakthrough::perception::VisualStream;
use breakthrough::cognition::MetaReason;
use breakthrough::cognition::metacognition::StrategySelection;
use breakthrough::reality::{Measurement, Unit, Precision, Accuracy};
use breakthrough::horizons::{Discovery, Exploration};
use breakthrough::archives::ExtinctSpecies;

fn main() {
    println!("=== Breakthrough Cognitive Ecosystem ===\n");

    // Kernel Technology: initialize topology, then coherence, resonance,
    // evolution, event field, scheduler, and runtime in correct ownership order.
    // Coherence must be created before resonance (resonance consumes it by value).
    // Topology is cloned for each consumer since multiple engines need their own copy.
    let topology = TopologyEngine::new(10);
    let coherence = CoherenceEngine::new(topology.clone(), 0.8);
    let resonance = ResonanceEngine::new(topology.clone(), coherence.clone(), 0.5);
    let evolution = EvolutionEngine::new(topology.clone(), 0.01);
    let event_field = EventField::new(resonance, evolution);
    let scheduler = Scheduler::new(topology.clone(), coherence.clone(), 1000);

    // Bootstrap initializes a copy of the topology before the runtime takes ownership.
    let mut bootstrap = Bootstrap::new(topology.clone());
    bootstrap.initialize().expect("bootstrap failed");
    println!("Genesis: topology initialized with 10 nodes");
    println!("Kernel: resonance, evolution, event field online");
    println!("Kernel: coherence engine targeting 0.8");
    println!("Kernel: scheduler ready for 1000 max ticks");
    println!("Bootstrap: system initialized: {}\n", bootstrap.is_ready());

    // EcosystemRuntime consumes the final topology instance.
    let mut runtime = EcosystemRuntime::new(event_field, coherence, topology);

    // Dynamic Memory Recall: associative retrieval from trace store.
    let mut recall = MemoryRecall::new(vec![0.8, 0.2], 0.65, 0.5).expect("valid recall");
    let traces: Vec<(Vec<f64>, f64)> = vec![(vec![1.0, 0.0], 0.9)];
    let result = recall.retrieve(&traces);
    println!("\nMemory: recall retrieval result: {:?}", result.is_some());

    // Cognition: semantic concept activation with associative density.
    let mut concept = SemanticConcept::new("river").expect("valid concept");
    concept.activate(0.5).expect("activation failed");
    println!(
        "Cognition: concept 'river' activated, density={:.4}",
        concept.associative_density()
    );

    // Enhanced Perception: Visual Stream at 64x64 resolution, 30Hz refresh.
    let stream = VisualStream::new((64, 64), 30.0).expect("valid stream");
    println!("Perception: visual stream {}x{} @ {}Hz", 64, 64, 30.0);

    // Meta Reasoning: multi-layered cognitive processing.
    let meta = MetaReason::new(3, StrategySelection::Heuristic).expect("valid meta reason");
    println!("Cognition: meta-reason depth={}", meta.depth());

    // Measurable Reality: quantified measurement with precision and accuracy.
    let precision = Precision::new(0.05, 100).expect("valid precision");
    let accuracy = Accuracy::new(0.0, 0.05, 0.95).expect("valid accuracy");
    let measurement = Measurement::new_scalar("cog-output-001", 0.75, Unit::None, precision, accuracy, 0.0)
        .expect("valid measurement");
    println!(
        "Reality: measurement = {:.4} +/- {:.4}",
        measurement.as_scalar().unwrap_or(0.0),
        measurement.absolute_uncertainty()
    );

    // Exploration and Discovery: venturing beyond the known.
    let exploration = Exploration::new(1, 0.0, 100).expect("valid exploration");
    println!("Horizons: exploration bearing={:.2} depth={}", 0.0, 100);

    let mut discovery = Discovery::new(2, 0.9, 10.0).expect("valid discovery");
    discovery.verify().expect("verify failed");
    println!(
        "Horizons: discovery confidence={:.2} distance={:.1}",
        discovery.confidence(),
        discovery.distance()
    );

    // Archives: preserving knowledge of Extinct Species (Neanderthal).
    let species = ExtinctSpecies::new(1, "Neanderthal".to_string(), "Pleistocene".to_string(), 0.9, 0.8)
        .expect("valid species");
    println!("Archives: extinct species '{}'", species.name);

    // Attractor Memory: stability of cognitive attractor basins.
    let state = AttractorMemoryState::new(vec![0.1, 0.2]).expect("valid state");
    println!(
        "Memory: attractor state stability={:.4}",
        state.potential_energy()
    );

    // Run the cognitive runtime for 5 cycles, demonstrating the integrated ecosystem.
    println!("\nRunning cognitive runtime for 5 cycles...\n");
    for i in 1..=5 {
        let _ = runtime.run(1.0);
        println!("Cycle {}: ticks={}", i, runtime.tick_count());
    }

    println!("\n=== Cognitive ecosystem prototype running ===");
}

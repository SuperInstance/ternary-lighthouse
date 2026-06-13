# ternary-lighthouse

Guidance and warning system for fleet navigation. Implements beacons for emitting directional signals, foghorns for low-visibility warnings, lenses for focusing attention, coded light patterns for identification, and watchkeepers for monitoring dangers — all using ternary directional semantics {-1, 0, +1}.

## Why It Matters

Autonomous fleet management requires reliable navigation aids. Physical lighthouses solve this for maritime navigation by providing:
- **Beacons** — persistent signals for orientation ("you are here relative to me")
- **Foghorns** — auditory warnings when visibility is poor (analog: degraded system health)
- **Coded patterns** — unique identifiers distinguishing one light source from another
- **Watchkeepers** — continuous monitoring with escalation protocols

This crate translates these concepts to fleet management for GPU compute clusters:
- **Beacons** = service health signals with ternary direction (negative/neutral/positive)
- **Foghorns** = severity-tiered alerts (Low → Medium → High → Critical)
- **Lenses** = attention-focusing filters that prioritize certain signal types
- **Watchkeepers** = rule-based monitors that detect dangerous conditions

The ternary direction {-1, 0, +1} encodes whether a signal is negative (trouble), neutral (informational), or positive (healthy), giving consumers a three-state semantic without parsing message content.

## How It Works

### Signal Model

Each beacon emits a `Signal`:

```
Signal = {
    source: String,        // beacon identifier
    direction: Ternary,    // −1 = negative, 0 = neutral, +1 = positive
    strength: f64,         // [0.0, ∞) — signal amplitude
    message: String        // human-readable content
}
```

The ternary direction provides **O(1) triage** — a consumer can filter signals by direction without parsing:

```
signals.filter(|s| s.direction == Ternary::Neg)  // all trouble signals
```

### Severity Escalation

Warnings carry a severity level using a 4-tier ordinal scale:

```
Low < Medium < High < Critical
```

The `Ord` derivation enables:
- **Threshold filtering**: `warnings.filter(|w| w.severity >= Severity::High)`
- **Priority queues**: sort by severity descending
- **Escalation rules**: if Low warnings persist for N ticks, escalate to Medium

### Coded Light Patterns

Inspired by real lighthouses (each has a unique flash pattern), the crate supports coded patterns for beacon identification:

A pattern is a ternary sequence — each element is {-1 (dark), 0 (dim), +1 (bright)}:

```
Pattern = [1, 1, 0, −1, 0]  →  "flash-flash-dim-dark-dim"
```

Two beacons with identical names can be distinguished by their patterns, enabling multi-source navigation in dense fleet topologies.

### Watchkeeper Logic

Watchkeepers are rule-based monitors. Each watch defines:
- **Watched resource**: beacon name or signal source
- **Trigger condition**: e.g., direction == Neg AND severity >= High
- **Action**: log, escalate, or broadcast warning

The watchkeeper evaluates active signals against all registered watches each scan cycle.

### Signal Attenuation

Signal strength follows inverse-square attenuation with distance:

```
strength_at(d) = strength₀ / max(1, d²)
```

where d = number of network hops (not physical distance). This models how information degrades in multi-hop fleet communication.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `Lighthouse::scan()` | O(B) | O(B) |
| `Lighthouse::sound_warnings()` | O(F) | O(F) |
| `Beacon::emit()` | O(1) | O(1) |
| `Foghorn::sound()` | O(1) | O(1) |
| `ShipLog::record(entry)` | O(1) amortized | O(1) |
| `ShipLog::query(filters)` | O(N) | O(k) |

Where B = number of beacons, F = number of foghorns, N = log entries, k = matching entries.

## Quick Start

```rust
use ternary_lighthouse::{Lighthouse, Beacon, Foghorn, Severity, Ternary};

let mut lighthouse = Lighthouse::new("Fleet Control");

// Register guidance beacons
let mut beacon = Beacon::new("gpu-cluster-health");
beacon.direction = Ternary::Pos;
beacon.active = true;
beacon.strength = 0.95;
beacon.message = "All GPUs operational".to_string();
lighthouse.add_beacon(beacon);

// Register warning foghorns
let mut foghorn = Foghorn::new("thermal-alert");
foghorn.severity = Severity::High;
foghorn.active = true;
foghorn.message = "GPU 3 temperature: 89°C".to_string();
lighthouse.add_foghorn(foghorn);

// Scan all active beacons
let signals = lighthouse.scan();
for signal in &signals {
    println!("[{}] {} (strength: {:.2})",
        signal.source, signal.message, signal.strength);
}

// Sound all warnings
let warnings = lighthouse.sound_warnings();
for warning in &warnings {
    println!("WARNING [{:?}]: {}", warning.severity, warning.message);
}

// Log observations
lighthouse.log_mut().record("All systems nominal at T=0");
```

## API

### `Lighthouse`

| Method | Description |
|--------|-------------|
| `new(name)` | Create lighthouse hub |
| `add_beacon(beacon) / add_foghorn(foghorn)` | Register navigation aids |
| `scan() -> Vec<Signal>` | Emit all active beacon signals |
| `sound_warnings() -> Vec<Warning>` | Sound all active foghorns |
| `log() / log_mut()` | Access ship's log |

### `Beacon`

| Field | Description |
|-------|-------------|
| `name` | Beacon identifier |
| `direction` | Ternary: Neg/Zero/Pos |
| `active` | Whether beacon is emitting |
| `strength` | Signal amplitude [0, ∞) |
| `emit() -> Signal` | Produce a signal snapshot |

### `Foghorn`

| Field | Description |
|-------|-------------|
| `name` | Foghorn identifier |
| `severity` | Low/Medium/High/Critical |
| `active` | Whether foghorn is sounding |
| `sound() -> Warning` | Produce a warning snapshot |

### `Severity`

```rust
pub enum Severity { Low, Medium, High, Critical }
```

Ordered: `Low < Medium < High < Critical`. Supports comparison for threshold filtering.

## Architecture Notes

This crate implements the **γ (gamma) coordination layer** of the γ + η = C framework:

- **γ (gamma)**: Fleet-level observability and signaling — beacons, foghorns, and watchkeepers are all γ-level coordination primitives that inform other system components about state and danger.
- **η (eta)**: The compute fleet being monitored — GPU workers, inference engines, and data pipelines that the lighthouse watches over.
- **C**: The complete fleet navigation system. γ ensures η-layer components can discover each other, detect problems, and coordinate responses.

The ternary direction {-1, 0, +1} is the universal signal domain across the ecosystem — a negative beacon direction means the same thing as a negative ternary weight or a negative Ising spin: "this is bad / inhibitory / trouble."

## References

- **Lighthouse Concept**: Maritime navigation theory — International Association of Lighthouse Authorities (IALA), "Aids to Navigation Manual," 2023.
- **Service Health Monitoring**: Nagios Enterprises, "Nagios Core Administration Guide," monitoring severity levels, 2023.
- **Alert Systems**: Liu, Y. et al., "Alert Correlation for Network Security Monitoring," IEEE Transactions on Information Forensics and Security, 2021.
- **Ternary Logic in Monitoring**: Shell, R.L., "Monitoring Systems with Ternary State Indicators," International Journal of Industrial Engineering, 2018.
- **Signal Attenuation**: Rappaport, T.S., "Wireless Communications: Principles and Practice," Prentice Hall, 2002. Chapter 4 on path-loss models.

## License

MIT

# ternary-lighthouse

**An observability system disguised as a maritime metaphor.**

A lighthouse does six things: it *emits* guidance signals (beacons), *warns* of danger (foghorns), *focuses* attention (lenses), *identifies* itself (coded light patterns), *monitors* conditions (watchkeepers), and *remembers* everything (ship's log). That's also what a good observability system does — and this crate implements both, using ternary signals throughout.

Every signal carries a ternary value: `+1` (positive/good/safe), `0` (neutral/unknown), `-1` (negative/danger/alert). Every warning has a severity level. Every pattern is a ternary code. The log records everything with timestamps and severity filtering.

## What's Inside

- **`Lighthouse`** — central hub. Manages beacons, foghorns, and the ship's log
- **`Beacon`** — emits `Signal`s with direction (ternary), strength (0-1), and message
- **`Foghorn`** — emits `Warning`s with severity (Low/Medium/High/Critical) and message
- **`Lens`** — focus on signals matching a keyword, amplify their strength. Like a log filter with gain
- **`LightPattern`** — ternary identification codes. Match patterns, compute similarity, generate checksums (mod 3)
- **`WatchKeeper`** — register known dangers, check conditions against thresholds. Only alerts when danger × condition exceeds alert level
- **`ShipLog`** — timestamped entries with severity, filtering, and recent-N retrieval. The memory of the system

## Quick Example

```rust
use ternary_lighthouse::*;

// Create a lighthouse for your system
let mut lh = Lighthouse::new("production");

// Register guidance beacons
lh.add_beacon(Beacon::new("api-health", Ternary::Pos, 0.9));
lh.add_beacon(Beacon::new("db-latency", Ternary::Neg, 0.6));

// Register warnings
lh.add_foghorn(Foghorn::new("disk-space", Severity::High, "Disk usage above 90%"));

// Scan for signals
let signals = lh.scan();
// [Signal{direction: Pos, strength: 0.9}, Signal{direction: Neg, strength: 0.6}]

// Focus with a lens
let lens = Lens::new("api", 3.0);
let focused = lens.focus(&signals);
// Only the api-health beacon survives the filter

// Watchkeeper monitors dangers
let mut wk = WatchKeeper::new("ops", Severity::Medium);
wk.register_danger("cpu-spike", Severity::High);
// check() against current conditions → alerts only when threshold exceeded

// Log everything
lh.log_mut().record("monitor", "System healthy", Severity::Low);
let recent = lh.log().recent(10);
let critical = lh.log().filter_severity(Severity::Critical);
```

## The Deeper Truth

**Observability is navigation.** You're steering a system through fog. The beacons tell you where you are. The foghorns warn you what's ahead. The lens focuses your attention when there's too much noise. The watchkeeper handles the repetitive checking so you don't have to. The ship's log is how you learn from the past.

The ternary constraint is deliberate: `+1/0/-1` is exactly the resolution you need for operational signals. Is the system healthy? Positive. Degraded? Zero. Failing? Negative. You don't need floating-point health scores — you need *decisive* signals that trigger *decisive* actions.

**Use cases:**
- **Microservice observability** — beacons for health, foghorns for incidents, log for postmortems
- **IoT monitoring** — lighthouses on edge devices, ternary signals upstream
- **Game server management** — watchkeepers for player experience degradation
- **Financial systems** — severity-based alerting on market anomalies
- **Any system that needs to *know what's happening* without drowning in data**

## See Also

- **ternary-bus** — the message bus that delivers signals between lighthouses
- **ternary-gauge** — the instruments that feed the lighthouse readings
- **ternary-beacon** — (if it exists) focused beacon-only functionality
- **ternary-epoch** — detect when the lighthouse enters a new operational era

## Install

```bash
cargo add ternary-lighthouse
```

## License

MIT

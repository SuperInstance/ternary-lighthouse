#![forbid(unsafe_code)]

//! Guidance and warning system for fleet navigation.
//!
//! Inspired by the lighthouse keeper concept from Oracle1, this crate provides
//! beacons for emitting guidance signals, foghorns for low-visibility warnings,
//! lenses for focusing attention, coded light patterns for identification,
//! watchkeepers for monitoring dangers, and ship logs for recording observations.

use std::collections::HashMap;

/// Ternary value: -1 (negative), 0 (neutral), +1 (positive).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        self as i8
    }
}

/// Severity level for warnings and signals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// The lighthouse: central hub managing all navigation aids.
#[derive(Debug)]
pub struct Lighthouse {
    pub name: String,
    beacons: Vec<Beacon>,
    foghorns: Vec<Foghorn>,
    log: ShipLog,
}

impl Lighthouse {
    pub fn new(name: &str) -> Self {
        Lighthouse {
            name: name.to_string(),
            beacons: Vec::new(),
            foghorns: Vec::new(),
            log: ShipLog::new(),
        }
    }

    pub fn add_beacon(&mut self, beacon: Beacon) {
        self.beacons.push(beacon);
    }

    pub fn add_foghorn(&mut self, foghorn: Foghorn) {
        self.foghorns.push(foghorn);
    }

    pub fn beacons(&self) -> &[Beacon] {
        &self.beacons
    }

    pub fn foghorns(&self) -> &[Foghorn] {
        &self.foghorns
    }

    pub fn log(&self) -> &ShipLog {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut ShipLog {
        &mut self.log
    }

    /// Emit all active beacons, returning their combined guidance.
    pub fn scan(&self) -> Vec<Signal> {
        self.beacons
            .iter()
            .filter(|b| b.active)
            .map(|b| b.emit())
            .collect()
    }

    /// Sound all active foghorns.
    pub fn sound_warnings(&self) -> Vec<Warning> {
        self.foghorns
            .iter()
            .filter(|f| f.active)
            .map(|f| f.sound())
            .collect()
    }
}

/// A guidance signal emitted by a beacon.
#[derive(Clone, Debug, PartialEq)]
pub struct Signal {
    pub source: String,
    pub direction: Ternary,
    pub strength: f64,
    pub message: String,
}

/// Emits guidance signals for navigation.
#[derive(Clone, Debug)]
pub struct Beacon {
    pub name: String,
    pub active: bool,
    direction: Ternary,
    strength: f64,
}

impl Beacon {
    pub fn new(name: &str, direction: Ternary, strength: f64) -> Self {
        Beacon {
            name: name.to_string(),
            active: true,
            direction,
            strength: strength.clamp(0.0, 1.0),
        }
    }

    pub fn emit(&self) -> Signal {
        Signal {
            source: self.name.clone(),
            direction: self.direction,
            strength: self.strength,
            message: format!("Beacon {} pointing {:?}", self.name, self.direction),
        }
    }

    pub fn set_strength(&mut self, strength: f64) {
        self.strength = strength.clamp(0.0, 1.0);
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn activate(&mut self) {
        self.active = true;
    }
}

/// A warning issued by a foghorn.
#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub source: String,
    pub severity: Severity,
    pub message: String,
    pub ternary_signal: Ternary,
}

/// Audible warning for low-visibility conditions.
#[derive(Clone, Debug)]
pub struct Foghorn {
    pub name: String,
    pub active: bool,
    severity: Severity,
    message: String,
}

impl Foghorn {
    pub fn new(name: &str, severity: Severity, message: &str) -> Self {
        Foghorn {
            name: name.to_string(),
            active: true,
            severity,
            message: message.to_string(),
        }
    }

    pub fn sound(&self) -> Warning {
        let ternary_signal = match self.severity {
            Severity::Low => Ternary::Neg,
            Severity::Medium => Ternary::Neg,
            Severity::High => Ternary::Neg,
            Severity::Critical => Ternary::Neg,
        };
        Warning {
            source: self.name.clone(),
            severity: self.severity,
            message: self.message.clone(),
            ternary_signal,
        }
    }

    pub fn set_severity(&mut self, severity: Severity) {
        self.severity = severity;
    }
}

/// Focuses attention on a specific area by filtering signals.
#[derive(Debug)]
pub struct Lens {
    pub focus_area: String,
    pub magnification: f64,
}

impl Lens {
    pub fn new(focus_area: &str, magnification: f64) -> Self {
        Lens {
            focus_area: focus_area.to_string(),
            magnification: magnification.max(1.0),
        }
    }

    /// Filter signals, keeping only those matching the focus area.
    pub fn focus<'a>(&self, signals: &'a [Signal]) -> Vec<&'a Signal> {
        signals
            .iter()
            .filter(|s| s.source.contains(&self.focus_area) || s.message.contains(&self.focus_area))
            .collect()
    }

    /// Amplify signal strengths by magnification.
    pub fn amplify(&self, signal: &Signal) -> Signal {
        Signal {
            source: signal.source.clone(),
            direction: signal.direction,
            strength: (signal.strength * self.magnification).min(1.0),
            message: signal.message.clone(),
        }
    }
}

/// Coded identification pattern using ternary sequences.
#[derive(Clone, Debug, PartialEq)]
pub struct LightPattern {
    pub name: String,
    pub code: Vec<Ternary>,
}

impl LightPattern {
    pub fn new(name: &str, code: Vec<Ternary>) -> Self {
        LightPattern {
            name: name.to_string(),
            code,
        }
    }

    /// Create a pattern from i8 values.
    pub fn from_i8s(name: &str, values: &[i8]) -> Option<Self> {
        let code: Vec<Ternary> = values.iter().filter_map(|&v| Ternary::from_i8(v)).collect();
        if code.len() == values.len() {
            Some(LightPattern {
                name: name.to_string(),
                code,
            })
        } else {
            None
        }
    }

    /// Check if a received pattern matches this one.
    pub fn matches(&self, received: &[Ternary]) -> bool {
        self.code == received
    }

    /// Compute similarity: fraction of matching positions.
    pub fn similarity(&self, other: &[Ternary]) -> f64 {
        if self.code.is_empty() || other.is_empty() {
            return 0.0;
        }
        let min_len = self.code.len().min(other.len());
        let matches = (0..min_len).filter(|&i| self.code[i] == other[i]).count();
        matches as f64 / self.code.len().max(other.len()) as f64
    }

    /// Compute the checksum (sum of ternary values mod 3).
    pub fn checksum(&self) -> i8 {
        let sum: i8 = self.code.iter().map(|t| t.to_i8()).sum();
        ((sum % 3) + 3) % 3
    }
}

/// Monitors for dangers and triggers warnings.
#[derive(Debug)]
pub struct WatchKeeper {
    pub name: String,
    dangers: HashMap<String, Severity>,
    alert_threshold: Severity,
}

impl WatchKeeper {
    pub fn new(name: &str, alert_threshold: Severity) -> Self {
        WatchKeeper {
            name: name.to_string(),
            dangers: HashMap::new(),
            alert_threshold,
        }
    }

    pub fn register_danger(&mut self, danger: &str, severity: Severity) {
        self.dangers.insert(danger.to_string(), severity);
    }

    pub fn remove_danger(&mut self, danger: &str) -> Option<Severity> {
        self.dangers.remove(danger)
    }

    /// Check if any registered danger exceeds the alert threshold.
    pub fn check(&self, conditions: &HashMap<String, Severity>) -> Vec<Warning> {
        let mut alerts = Vec::new();
        for (danger, &observed_severity) in conditions {
            if let Some(&registered) = self.dangers.get(danger) {
                let effective = observed_severity.max(registered);
                if effective >= self.alert_threshold {
                    alerts.push(Warning {
                        source: self.name.clone(),
                        severity: effective,
                        message: format!("Danger '{}' detected at {:?} severity", danger, effective),
                        ternary_signal: Ternary::Neg,
                    });
                }
            }
        }
        alerts
    }

    pub fn danger_count(&self) -> usize {
        self.dangers.len()
    }
}

/// Records all observed events in chronological order.
#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub timestamp: u64,
    pub source: String,
    pub message: String,
    pub severity: Severity,
}

/// Ship log: persistent record of observations.
#[derive(Debug, Clone)]
pub struct ShipLog {
    entries: Vec<LogEntry>,
    next_timestamp: u64,
}

impl ShipLog {
    pub fn new() -> Self {
        ShipLog {
            entries: Vec::new(),
            next_timestamp: 0,
        }
    }

    pub fn record(&mut self, source: &str, message: &str, severity: Severity) -> LogEntry {
        let entry = LogEntry {
            timestamp: self.next_timestamp,
            source: source.to_string(),
            message: message.to_string(),
            severity,
        };
        self.next_timestamp += 1;
        self.entries.push(entry.clone());
        entry
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Filter entries by minimum severity.
    pub fn filter_severity(&self, min_severity: Severity) -> Vec<&LogEntry> {
        self.entries.iter().filter(|e| e.severity >= min_severity).collect()
    }

    /// Get the most recent N entries.
    pub fn recent(&self, count: usize) -> &[LogEntry] {
        let start = self.entries.len().saturating_sub(count);
        &self.entries[start..]
    }
}

impl Default for ShipLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_values() {
        assert_eq!(Ternary::Neg.to_i8(), -1);
        assert_eq!(Ternary::Zero.to_i8(), 0);
        assert_eq!(Ternary::Pos.to_i8(), 1);
    }

    #[test]
    fn test_beacon_emit() {
        let b = Beacon::new("north", Ternary::Pos, 0.8);
        let signal = b.emit();
        assert_eq!(signal.direction, Ternary::Pos);
        assert!((signal.strength - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_beacon_strength_clamped() {
        let b = Beacon::new("strong", Ternary::Pos, 2.0);
        assert!((b.strength - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_beacon_deactivate() {
        let mut b = Beacon::new("test", Ternary::Zero, 0.5);
        b.deactivate();
        assert!(!b.active);
    }

    #[test]
    fn test_foghorn_sound() {
        let f = Foghorn::new("main", Severity::High, "Rocks ahead");
        let w = f.sound();
        assert_eq!(w.severity, Severity::High);
        assert_eq!(w.message, "Rocks ahead");
    }

    #[test]
    fn test_lighthouse_scan() {
        let mut l = Lighthouse::new("tower1");
        l.add_beacon(Beacon::new("a", Ternary::Pos, 0.5));
        l.add_beacon(Beacon::new("b", Ternary::Neg, 0.3));
        let signals = l.scan();
        assert_eq!(signals.len(), 2);
    }

    #[test]
    fn test_lighthouse_scan_skips_inactive() {
        let mut l = Lighthouse::new("tower1");
        let mut b = Beacon::new("a", Ternary::Pos, 0.5);
        b.deactivate();
        l.add_beacon(b);
        assert!(l.scan().is_empty());
    }

    #[test]
    fn test_lighthouse_sound_warnings() {
        let mut l = Lighthouse::new("tower1");
        l.add_foghorn(Foghorn::new("fog1", Severity::Medium, "Low visibility"));
        let warnings = l.sound_warnings();
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_lens_focus() {
        let lens = Lens::new("north", 2.0);
        let signals = vec![
            Signal { source: "north_beacon".into(), direction: Ternary::Pos, strength: 0.3, message: "Go north".into() },
            Signal { source: "south_beacon".into(), direction: Ternary::Neg, strength: 0.5, message: "Go south".into() },
        ];
        let focused = lens.focus(&signals);
        assert_eq!(focused.len(), 1);
        assert_eq!(focused[0].source, "north_beacon");
    }

    #[test]
    fn test_lens_amplify() {
        let lens = Lens::new("x", 3.0);
        let signal = Signal { source: "s".into(), direction: Ternary::Pos, strength: 0.2, message: "m".into() };
        let amplified = lens.amplify(&signal);
        assert!((amplified.strength - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_lens_amplify_capped() {
        let lens = Lens::new("x", 10.0);
        let signal = Signal { source: "s".into(), direction: Ternary::Pos, strength: 0.5, message: "m".into() };
        let amplified = lens.amplify(&signal);
        assert!((amplified.strength - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_light_pattern_from_i8s() {
        let lp = LightPattern::from_i8s("id1", &[1, 0, -1, 1]).unwrap();
        assert_eq!(lp.code.len(), 4);
    }

    #[test]
    fn test_light_pattern_rejects_invalid() {
        assert!(LightPattern::from_i8s("bad", &[1, 2]).is_none());
    }

    #[test]
    fn test_light_pattern_matches() {
        let lp = LightPattern::new("id", vec![Ternary::Pos, Ternary::Zero, Ternary::Neg]);
        assert!(lp.matches(&[Ternary::Pos, Ternary::Zero, Ternary::Neg]));
        assert!(!lp.matches(&[Ternary::Pos, Ternary::Pos, Ternary::Neg]));
    }

    #[test]
    fn test_light_pattern_similarity() {
        let lp = LightPattern::new("id", vec![Ternary::Pos, Ternary::Zero, Ternary::Neg]);
        let sim = lp.similarity(&[Ternary::Pos, Ternary::Pos, Ternary::Neg]);
        assert!((sim - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_light_pattern_checksum() {
        let lp = LightPattern::new("id", vec![Ternary::Pos, Ternary::Neg, Ternary::Zero]);
        assert_eq!(lp.checksum(), 0);
    }

    #[test]
    fn test_watchkeeper_check() {
        let mut wk = WatchKeeper::new("sentinel", Severity::Medium);
        wk.register_danger("storm", Severity::High);
        let mut conditions = HashMap::new();
        conditions.insert("storm".to_string(), Severity::High);
        let alerts = wk.check(&conditions);
        assert_eq!(alerts.len(), 1);
    }

    #[test]
    fn test_watchkeeper_no_alert_below_threshold() {
        let mut wk = WatchKeeper::new("sentinel", Severity::Critical);
        wk.register_danger("breeze", Severity::Low);
        let mut conditions = HashMap::new();
        conditions.insert("breeze".to_string(), Severity::Low);
        assert!(wk.check(&conditions).is_empty());
    }

    #[test]
    fn test_shiplog_record_and_read() {
        let mut log = ShipLog::new();
        log.record("radar", "Contact detected", Severity::Medium);
        log.record("sonar", "Submerged contact", Severity::High);
        assert_eq!(log.len(), 2);
        assert_eq!(log.entries()[0].source, "radar");
    }

    #[test]
    fn test_shiplog_filter_severity() {
        let mut log = ShipLog::new();
        log.record("a", "low", Severity::Low);
        log.record("b", "high", Severity::High);
        log.record("c", "critical", Severity::Critical);
        let high = log.filter_severity(Severity::High);
        assert_eq!(high.len(), 2);
    }

    #[test]
    fn test_shiplog_recent() {
        let mut log = ShipLog::new();
        for i in 0..10 {
            log.record("src", &format!("entry_{}", i), Severity::Low);
        }
        let recent = log.recent(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[2].message, "entry_9");
    }
}

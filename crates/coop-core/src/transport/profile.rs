//! Named network condition profiles for [`super::ChaosTransport`].

/// The six named profiles.
pub const PROFILE_NAMES: [&str; 6] = [
    "good_wifi",
    "bad_wifi",
    "mobile",
    "congested",
    "flaky",
    "partition",
];

/// Parameters of one simulated link direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChaosProfile {
    /// Profile name.
    pub name: &'static str,
    /// Base one-way latency in ms.
    pub latency_ms: u64,
    /// Extra uniform random delay `0..=jitter_ms`.
    pub jitter_ms: u64,
    /// Loss probability per message.
    pub loss: f64,
    /// Probability of delivering an unreliable message twice.
    pub dup: f64,
    /// Probability of delaying an unreliable message by `reorder_extra_ms`.
    pub reorder: f64,
    /// Extra delay applied to reordered messages.
    pub reorder_extra_ms: u64,
    /// Every `partition_period_ms` the link is dead for `partition_len_ms`
    /// (0 disables).
    pub partition_period_ms: u64,
    /// Length of each outage.
    pub partition_len_ms: u64,
}

impl ChaosProfile {
    /// A perfect link (zero latency, no faults).
    pub const PERFECT: ChaosProfile = ChaosProfile {
        name: "perfect",
        latency_ms: 0,
        jitter_ms: 0,
        loss: 0.0,
        dup: 0.0,
        reorder: 0.0,
        reorder_extra_ms: 0,
        partition_period_ms: 0,
        partition_len_ms: 0,
    };

    /// Wired or strong Wi-Fi.
    pub const GOOD_WIFI: ChaosProfile = ChaosProfile {
        name: "good_wifi",
        latency_ms: 15,
        jitter_ms: 5,
        loss: 0.005,
        ..Self::PERFECT
    };

    /// Weak Wi-Fi.
    pub const BAD_WIFI: ChaosProfile = ChaosProfile {
        name: "bad_wifi",
        latency_ms: 50,
        jitter_ms: 40,
        loss: 0.05,
        dup: 0.01,
        reorder: 0.05,
        reorder_extra_ms: 80,
        ..Self::PERFECT
    };

    /// Cellular.
    pub const MOBILE: ChaosProfile = ChaosProfile {
        name: "mobile",
        latency_ms: 90,
        jitter_ms: 60,
        loss: 0.03,
        dup: 0.005,
        reorder: 0.03,
        reorder_extra_ms: 120,
        ..Self::PERFECT
    };

    /// Saturated uplink.
    pub const CONGESTED: ChaosProfile = ChaosProfile {
        name: "congested",
        latency_ms: 150,
        jitter_ms: 100,
        loss: 0.10,
        dup: 0.02,
        reorder: 0.10,
        reorder_extra_ms: 200,
        ..Self::PERFECT
    };

    /// Very lossy and disordered.
    pub const FLAKY: ChaosProfile = ChaosProfile {
        name: "flaky",
        latency_ms: 40,
        jitter_ms: 30,
        loss: 0.25,
        dup: 0.05,
        reorder: 0.15,
        reorder_extra_ms: 100,
        ..Self::PERFECT
    };

    /// Decent link that goes completely dead for 1.5 s every 6 s.
    pub const PARTITION: ChaosProfile = ChaosProfile {
        name: "partition",
        latency_ms: 30,
        jitter_ms: 10,
        loss: 0.02,
        partition_period_ms: 6_000,
        partition_len_ms: 1_500,
        ..Self::PERFECT
    };

    /// Looks a profile up by name.
    pub fn by_name(name: &str) -> Option<ChaosProfile> {
        Some(match name {
            "perfect" => Self::PERFECT,
            "good_wifi" => Self::GOOD_WIFI,
            "bad_wifi" => Self::BAD_WIFI,
            "mobile" => Self::MOBILE,
            "congested" => Self::CONGESTED,
            "flaky" => Self::FLAKY,
            "partition" => Self::PARTITION,
            _ => return None,
        })
    }

    /// If the link is down at `now`, the time at which it comes back.
    pub fn partition_end(&self, now: u64) -> Option<u64> {
        if self.partition_period_ms == 0 || self.partition_len_ms == 0 {
            return None;
        }
        let phase = now % self.partition_period_ms;
        // the outage is the tail of each period
        let start = self
            .partition_period_ms
            .saturating_sub(self.partition_len_ms);
        (phase >= start).then(|| now - phase + self.partition_period_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_names_resolve() {
        for n in PROFILE_NAMES {
            let p = ChaosProfile::by_name(n).unwrap();
            assert_eq!(p.name, n);
        }
        assert!(ChaosProfile::by_name("nope").is_none());
    }

    #[test]
    fn partition_windows() {
        let p = ChaosProfile::PARTITION;
        assert_eq!(p.partition_end(0), None);
        assert_eq!(p.partition_end(4_499), None);
        assert_eq!(p.partition_end(4_500), Some(6_000));
        assert_eq!(p.partition_end(5_999), Some(6_000));
        assert_eq!(p.partition_end(6_000), None);
        assert_eq!(p.partition_end(10_600), Some(12_000));
        assert_eq!(ChaosProfile::BAD_WIFI.partition_end(5_000), None);
    }
}

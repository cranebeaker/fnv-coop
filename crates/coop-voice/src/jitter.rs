//! Jitter buffer. Clocked playout: the caller calls [`JitterBuffer::pop`]
//! once per frame interval. Handles reorder, duplicates, loss and 16 bit
//! sequence wraparound.

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pop {
    /// Not started yet (prebuffering) or the talk spurt ended. Play silence.
    Idle,
    /// The next frame in order.
    Frame(Vec<u8>),
    /// A frame is missing; the caller should run the codec's concealment.
    Conceal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Push {
    Accepted,
    Duplicate,
    /// Arrived after its playout slot passed.
    Late,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub accepted: u64,
    pub duplicates: u64,
    pub late: u64,
    pub played: u64,
    pub concealed: u64,
    pub overflow_drops: u64,
    pub resets: u64,
}

pub struct JitterBuffer {
    /// Frames to hold before starting playout.
    target: usize,
    /// Hard cap on buffered frames; oldest are skipped past it.
    max_depth: usize,
    /// Consecutive concealments with nothing buffered before going idle.
    max_conceal_run: usize,
    frames: BTreeMap<i64, Vec<u8>>,
    highest: Option<i64>,
    next_play: Option<i64>,
    conceal_run: usize,
    ending: bool,
    pub stats: Stats,
}

impl JitterBuffer {
    pub fn new(target: usize, max_depth: usize) -> Self {
        let target = target.max(1);
        Self {
            target,
            max_depth: max_depth.max(target + 1),
            max_conceal_run: 5,
            frames: BTreeMap::new(),
            highest: None,
            next_play: None,
            conceal_run: 0,
            ending: false,
            stats: Stats::default(),
        }
    }

    pub fn set_max_conceal_run(&mut self, n: usize) {
        self.max_conceal_run = n.max(1);
    }

    pub fn buffered(&self) -> usize {
        self.frames.len()
    }

    pub fn is_playing(&self) -> bool {
        self.next_play.is_some()
    }

    fn extend(&self, seq: u16) -> i64 {
        match self.highest {
            None => 65536 + seq as i64,
            Some(h) => {
                let d = seq.wrapping_sub(h as u16) as i16 as i64;
                h + d
            }
        }
    }

    pub fn push(&mut self, seq: u16, payload: Vec<u8>) -> Push {
        let ext = self.extend(seq);
        if let Some(np) = self.next_play {
            if ext < np {
                self.stats.late += 1;
                return Push::Late;
            }
            // A huge forward jump means the stream restarted: resync.
            if ext > np + 2000 {
                self.reset();
            }
        }
        let ext = if self.highest.is_none() {
            ext
        } else {
            self.extend(seq)
        };
        if self.frames.contains_key(&ext) {
            self.stats.duplicates += 1;
            return Push::Duplicate;
        }
        self.frames.insert(ext, payload);
        self.stats.accepted += 1;
        self.ending = false;
        if self.highest.map_or(true, |h| ext > h) {
            self.highest = Some(ext);
        }
        while self.frames.len() > self.max_depth {
            // Skip the oldest frame.
            if let Some((&k, _)) = self.frames.iter().next() {
                self.frames.remove(&k);
                self.stats.overflow_drops += 1;
                if let Some(np) = self.next_play {
                    if k >= np {
                        self.next_play = Some(k + 1);
                    }
                }
            }
        }
        Push::Accepted
    }

    /// The sender signalled end of talk: drain what we have, then go idle
    /// instead of concealing.
    pub fn finish(&mut self) {
        self.ending = true;
    }

    fn reset(&mut self) {
        self.frames.clear();
        self.highest = None;
        self.next_play = None;
        self.conceal_run = 0;
        self.ending = false;
        self.stats.resets += 1;
    }

    pub fn pop(&mut self) -> Pop {
        if self.next_play.is_none() {
            if self.frames.len() >= self.target || (self.ending && !self.frames.is_empty()) {
                self.next_play = self.frames.keys().next().copied();
            } else {
                return Pop::Idle;
            }
        }
        let np = self.next_play.unwrap();
        if let Some(f) = self.frames.remove(&np) {
            self.next_play = Some(np + 1);
            self.conceal_run = 0;
            self.stats.played += 1;
            return Pop::Frame(f);
        }
        let later = self.frames.range(np..).next().is_some();
        if !later && self.ending {
            self.reset();
            return Pop::Idle;
        }
        if !later {
            self.conceal_run += 1;
            if self.conceal_run > self.max_conceal_run {
                self.reset();
                return Pop::Idle;
            }
        }
        self.next_play = Some(np + 1);
        self.stats.concealed += 1;
        Pop::Conceal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn pl(seq: u16) -> Vec<u8> {
        seq.to_le_bytes().to_vec()
    }

    #[test]
    fn in_order_no_conceal() {
        let mut j = JitterBuffer::new(2, 10);
        assert_eq!(j.pop(), Pop::Idle);
        j.push(0, pl(0));
        assert_eq!(j.pop(), Pop::Idle);
        j.push(1, pl(1));
        assert_eq!(j.pop(), Pop::Frame(pl(0)));
        assert_eq!(j.pop(), Pop::Frame(pl(1)));
    }

    #[test]
    fn reorder_loss_dup_late() {
        let mut j = JitterBuffer::new(3, 10);
        j.push(1, pl(1));
        j.push(0, pl(0));
        assert_eq!(j.push(1, pl(1)), Push::Duplicate);
        j.push(3, pl(3));
        assert_eq!(j.pop(), Pop::Frame(pl(0)));
        assert_eq!(j.pop(), Pop::Frame(pl(1)));
        assert_eq!(j.pop(), Pop::Conceal); // 2 lost
        assert_eq!(j.push(2, pl(2)), Push::Late);
        assert_eq!(j.pop(), Pop::Frame(pl(3)));
    }

    #[test]
    fn wraparound() {
        let mut j = JitterBuffer::new(2, 10);
        j.push(65535, pl(65535));
        j.push(0, pl(0));
        assert_eq!(j.pop(), Pop::Frame(pl(65535)));
        assert_eq!(j.pop(), Pop::Frame(pl(0)));
    }

    #[test]
    fn goes_idle_after_silence() {
        let mut j = JitterBuffer::new(1, 10);
        j.push(0, pl(0));
        assert!(matches!(j.pop(), Pop::Frame(_)));
        let mut idle = false;
        for _ in 0..10 {
            if j.pop() == Pop::Idle {
                idle = true;
                break;
            }
        }
        assert!(idle);
        assert!(!j.is_playing());
    }

    #[test]
    fn finish_drains_then_idle() {
        let mut j = JitterBuffer::new(4, 10);
        j.push(0, pl(0));
        j.push(1, pl(1));
        j.finish();
        assert_eq!(j.pop(), Pop::Frame(pl(0)));
        assert_eq!(j.pop(), Pop::Frame(pl(1)));
        assert_eq!(j.pop(), Pop::Idle);
    }

    #[test]
    fn overflow_skips_oldest() {
        let mut j = JitterBuffer::new(2, 4);
        for s in 0..8u16 {
            j.push(s, pl(s));
        }
        assert!(j.buffered() <= 4);
        assert!(j.stats.overflow_drops >= 4);
    }

    proptest! {
        /// Any reordering/duplication/loss: output is strictly increasing,
        /// each frame is a genuine pushed payload, none emitted twice.
        #[test]
        fn prop_ordered_unique(
            start in any::<u16>(),
            ops in proptest::collection::vec((0u16..40, any::<bool>(), 0u8..3), 1..200),
        ) {
            let mut j = JitterBuffer::new(3, 16);
            let mut pushed = std::collections::HashSet::new();
            let mut last: Option<i64> = None;
            let base = 0i64;
            let mut resets = 0;
            for (off, do_pop, dups) in ops {
                let seq = start.wrapping_add(off);
                for _ in 0..=dups {
                    j.push(seq, pl(seq));
                }
                pushed.insert(seq);
                if do_pop {
                    let r = j.pop();
                    if j.stats.resets != resets { resets = j.stats.resets; last = None; }
                    if let Pop::Frame(p) = r {
                        let s = u16::from_le_bytes([p[0], p[1]]);
                        prop_assert!(pushed.contains(&s));
                        // unwrap relative to start (offsets < 40, so tiny window)
                        let rel = s.wrapping_sub(start) as i64 + base;
                        if let Some(l) = last { prop_assert!(rel > l, "rel {rel} last {l}"); }
                        last = Some(rel);
                    }
                }
            }
        }

        /// Complete in-order delivery with shuffled arrival inside the
        /// prebuffer window yields every frame, no concealment.
        #[test]
        fn prop_local_reorder_lossless(start in any::<u16>(), n in 4usize..60, swaps in proptest::collection::vec(0usize..60, 0..20)) {
            let mut order: Vec<u16> = (0..n as u16).collect();
            for s in swaps {
                let i = s % (n - 1);
                order.swap(i, i + 1);
            }
            let mut j = JitterBuffer::new(4, 64);
            for s in &order {
                j.push(start.wrapping_add(*s), pl(start.wrapping_add(*s)));
            }
            let mut got = vec![];
            while got.len() < n {
                match j.pop() {
                    Pop::Frame(p) => got.push(u16::from_le_bytes([p[0], p[1]])),
                    Pop::Conceal => prop_assert!(false, "unexpected conceal"),
                    Pop::Idle => break,
                }
                if got.len() > n { break; }
            }
            let want: Vec<u16> = (0..n as u16).map(|s| start.wrapping_add(s)).collect();
            prop_assert_eq!(got, want);
        }

        /// Dropping random frames yields exactly one Conceal per interior gap slot.
        #[test]
        fn prop_loss_conceals_gaps(start in any::<u16>(), keep in proptest::collection::vec(any::<bool>(), 8..60)) {
            let mut keep = keep;
            keep[0] = true;
            let last_kept = keep.iter().rposition(|k| *k).unwrap();
            let mut j = JitterBuffer::new(1, 128);
            // buffer all, then drain; first frame defines start of playout
            for (i, k) in keep.iter().enumerate() {
                if *k { j.push(start.wrapping_add(i as u16), pl(i as u16)); }
            }
            let mut frames = 0; let mut conceals = 0;
            for _ in 0..=last_kept {
                match j.pop() { Pop::Frame(_) => frames += 1, Pop::Conceal => conceals += 1, Pop::Idle => break }
            }
            let kept = keep.iter().filter(|k| **k).count();
            prop_assert_eq!(frames, kept);
            prop_assert_eq!(conceals, last_kept + 1 - kept);
        }
    }
}

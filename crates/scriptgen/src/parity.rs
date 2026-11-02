//! Checks that generated code runs exactly as the interpreter does: both
//! run the same script against a [`Recorder`] host that answers from a
//! seeded sequence (sometimes "can't work it out"), over several rounds
//! with every block kind, and every call, variable read and write, the
//! values they see and the variables left afterwards must match.

use script::interp::{self, Flow, Host, Locals};
use script::{Block, Call, Script, VarKind};

/// A generated `run_blocks`: block kind, which blocks to run, variables, host.
pub type RunBlocks = fn(&str, &dyn Fn(&Block) -> bool, &mut Locals, &mut dyn Host);

/// The generated functions for one script.
pub struct Generated {
    pub variables: &'static [(VarKind, &'static str)],
    pub blocks: fn() -> &'static [Block],
    pub run_blocks: RunBlocks,
    pub run_body: fn(&mut Locals, &mut dyn Host) -> Flow,
}

/// A host that writes down everything asked of it and answers from a
/// seeded sequence: small whole numbers mostly (so `if x == 1` branches
/// both ways), now and then a fraction, a large value or nothing.
pub struct Recorder {
    state: u64,
    pub log: Vec<String>,
}

impl Recorder {
    pub fn new(seed: u64) -> Recorder {
        Recorder {
            state: seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1,
            log: Vec::new(),
        }
    }

    fn next(&mut self) -> u64 {
        // xorshift64*
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn value(&mut self) -> f64 {
        let r = self.next();
        match r % 10 {
            0 => (r >> 8) as f64 / 7.0 % 100.0,
            1 => -(((r >> 8) % 5) as f64),
            2 => ((r >> 8) % 100_000) as f64,
            _ => ((r >> 8) % 3) as f64,
        }
    }

    fn answer(&mut self, none_in: u64) -> Option<f64> {
        if self.next() % none_in == 0 {
            None
        } else {
            Some(self.value())
        }
    }
}

fn snapshot(locals: &Locals) -> String {
    locals
        .iter()
        .map(|(n, _, v)| format!("{n}={v}"))
        .collect::<Vec<_>>()
        .join(",")
}

impl Host for Recorder {
    fn call(&mut self, call: &Call, on: Option<u32>, locals: &mut Locals) -> Option<f64> {
        let answer = self.answer(9);
        self.log.push(format!(
            "call {} on {on:?} args {:?} [{}] -> {answer:?}",
            script::function_name(call.function),
            call.args,
            snapshot(locals)
        ));
        answer
    }
    fn get_var(&mut self, owner: &str, name: &str, _locals: &Locals) -> Option<f64> {
        let answer = self.answer(5);
        self.log.push(format!("get {owner}.{name} -> {answer:?}"));
        answer
    }
    fn set_var(&mut self, owner: &str, name: &str, value: f64, _locals: &mut Locals) -> bool {
        self.log.push(format!("set {owner}.{name} = {value}"));
        true
    }
    fn resolve(&mut self, word: &str) -> Option<f64> {
        let answer = self.answer(3);
        self.log.push(format!("resolve {word} -> {answer:?}"));
        answer
    }
}

/// Seeds per script; each runs [`ROUNDS`] rounds of every block kind.
pub const SEEDS: u64 = 16;
pub const ROUNDS: usize = 4;

/// Runs `source` through the interpreter and `generated` side by side;
/// the first difference, if any.
pub fn check(source: &str, generated: &Generated) -> Result<(), String> {
    let script = script::parse(source).map_err(|e| format!("doesn't parse: {e}"))?;
    let vars: Vec<(VarKind, String)> = generated
        .variables
        .iter()
        .map(|(k, n)| (*k, n.to_string()))
        .collect();
    if vars != script.variables {
        return Err("variables differ".into());
    }
    let headers: Vec<(&str, &[script::Arg])> = (generated.blocks)()
        .iter()
        .map(|b| (b.kind.as_str(), b.args.as_slice()))
        .collect();
    let parsed: Vec<(&str, &[script::Arg])> = script
        .blocks
        .iter()
        .map(|b| (b.kind.as_str(), b.args.as_slice()))
        .collect();
    if headers != parsed {
        return Err("block headers differ".into());
    }
    let shell = Script {
        variables: vars,
        ..Script::default()
    };
    let mut kinds: Vec<&str> = Vec::new();
    for b in &script.blocks {
        if !kinds.contains(&b.kind.as_str()) {
            kinds.push(&b.kind);
        }
    }

    for seed in 0..SEEDS {
        // Blocks with an argument are picked half the time on odd seeds.
        let matches = move |b: &Block| seed % 2 == 0 || b.args.len() % 2 == 0;
        let mut setup = Recorder::new(seed ^ 0xA5A5);
        let mut li = Locals::new(&script);
        for (_, name) in &script.variables {
            let v = setup.value();
            li.set(name, v);
        }
        let mut lg = Locals::new(&shell);
        for (name, _, v) in li.iter() {
            lg.set(name, v);
        }
        let (mut hi, mut hg) = (Recorder::new(seed), Recorder::new(seed));
        for round in 0..ROUNDS {
            for kind in &kinds {
                interp::run_blocks(&script, kind, matches, &mut li, &mut hi);
                (generated.run_blocks)(kind, &matches, &mut lg, &mut hg);
            }
            let fi = interp::run(&script.body, &mut li, &mut hi);
            let fg = (generated.run_body)(&mut lg, &mut hg);
            if fi != fg {
                return Err(format!(
                    "seed {seed} round {round}: body ended {fi:?}, generated {fg:?}"
                ));
            }
            if let Some(at) =
                (0..hi.log.len().max(hg.log.len())).find(|&i| hi.log.get(i) != hg.log.get(i))
            {
                return Err(format!(
                    "seed {seed} round {round}, step {at}: interpreter {:?}, generated {:?}",
                    hi.log.get(at),
                    hg.log.get(at)
                ));
            }
            if li != lg {
                return Err(format!(
                    "seed {seed} round {round}: variables [{}] vs [{}]",
                    snapshot(&li),
                    snapshot(&lg)
                ));
            }
        }
    }
    Ok(())
}

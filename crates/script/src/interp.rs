//! Running parsed scripts. The game is reached through a [`Host`]: it
//! carries out function calls, reads and writes other scripts' variables,
//! and turns words into values (globals, references). Numbers are `f64`;
//! references are their form IDs as numbers, as the game's own `ref`
//! variables hold them.

use crate::parser::{Block, Call, Expr, Item, Op, Script, Stmt, VarKind};

/// A script's own variables, by name (case-insensitive).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Locals {
    names: Vec<String>,
    kinds: Vec<VarKind>,
    values: Vec<f64>,
}

impl Locals {
    /// A script's variables, all 0.
    pub fn new(script: &Script) -> Locals {
        Locals {
            names: script
                .variables
                .iter()
                .map(|(_, n)| n.to_ascii_lowercase())
                .collect(),
            kinds: script.variables.iter().map(|(k, _)| *k).collect(),
            values: vec![0.0; script.variables.len()],
        }
    }

    /// Every variable: name (lower case), kind and value.
    pub fn iter(&self) -> impl Iterator<Item = (&str, VarKind, f64)> {
        self.names
            .iter()
            .zip(&self.kinds)
            .zip(&self.values)
            .map(|((n, k), v)| (n.as_str(), *k, *v))
    }

    /// Adds a variable (for loading a saved game), or sets it when it's
    /// already there.
    pub fn insert(&mut self, name: &str, kind: VarKind, value: f64) {
        match self.index(name) {
            Some(i) => self.values[i] = value,
            None => {
                self.names.push(name.to_ascii_lowercase());
                self.kinds.push(kind);
                self.values.push(value);
            }
        }
    }

    fn index(&self, name: &str) -> Option<usize> {
        let name = name.to_ascii_lowercase();
        self.names.iter().position(|n| *n == name)
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.index(name).map(|i| self.values[i])
    }

    /// Sets a variable (whole-number ones are truncated, as in the game).
    /// `false` when the script has no such variable.
    pub fn set(&mut self, name: &str, value: f64) -> bool {
        match self.index(name) {
            Some(i) => {
                self.values[i] = match self.kinds[i] {
                    VarKind::Integer => value.trunc(),
                    _ => value,
                };
                true
            }
            None => false,
        }
    }
}

/// What a script runs against.
pub trait Host {
    /// Carries out a function. `on` is the reference it's called on (a
    /// form ID) or `None` for the script's owner; arguments are as written
    /// and the host interprets them (editor IDs, names, variables). `None`
    /// when the host can't work out the function's value: a run that
    /// needs it stops there (see [`Flow::Stopped`]).
    fn call(&mut self, call: &Call, on: Option<u32>, locals: &mut Locals) -> Option<f64>;
    /// Another script's variable (`VCG01.bRunTimer`): `owner` is a quest's
    /// or a reference's editor ID, or a `ref` variable in `locals`.
    fn get_var(&mut self, owner: &str, name: &str, locals: &Locals) -> Option<f64>;
    /// Sets one; `owner` is empty for a name that isn't a local (a
    /// global variable).
    fn set_var(&mut self, owner: &str, name: &str, value: f64, locals: &mut Locals) -> bool;
    /// A word used as a value: a global variable's value, or a reference's
    /// form ID.
    fn resolve(&mut self, word: &str) -> Option<f64>;
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Done,
    Returned,
    /// A value it needed couldn't be worked out (a function the host
    /// doesn't carry out yet), so the rest wasn't run rather than run on
    /// a made-up value.
    Stopped,
}

/// Runs every block of a kind (`gamemode`, `onactivate`, …) in order.
/// `matches` decides on blocks with an argument (`MenuMode 1036`,
/// `OnActivate Player`).
pub fn run_blocks(
    script: &Script,
    kind: &str,
    matches: impl Fn(&Block) -> bool,
    locals: &mut Locals,
    host: &mut dyn Host,
) {
    for block in script
        .blocks
        .iter()
        .filter(|b| b.kind == kind && matches(b))
    {
        if run(&block.body, locals, host) != Flow::Done {
            break;
        }
    }
}

/// Runs statements (a block's body, or a result script).
pub fn run(stmts: &[Stmt], locals: &mut Locals, host: &mut dyn Host) -> Flow {
    for stmt in stmts {
        match stmt {
            Stmt::Set { target, value } => {
                let Some(v) = eval(value, locals, host) else {
                    return Flow::Stopped;
                };
                set(target, v, locals, host);
            }
            Stmt::If {
                branches,
                otherwise,
            } => {
                let mut taken = None;
                for (cond, body) in branches {
                    match eval(cond, locals, host) {
                        None => return Flow::Stopped,
                        Some(v) if v != 0.0 => {
                            taken = Some(body);
                            break;
                        }
                        Some(_) => {}
                    }
                }
                let body = match taken {
                    Some(b) => b.as_slice(),
                    None => otherwise.as_deref().unwrap_or_default(),
                };
                let flow = run(body, locals, host);
                if flow != Flow::Done {
                    return flow;
                }
            }
            Stmt::Return => return Flow::Returned,
            Stmt::Call(call) => {
                // A statement's own value isn't needed: one the host can't
                // carry out is skipped.
                call_function(call, locals, host);
            }
        }
    }
    Flow::Done
}

/// `set <target> to <value>`: a local (whole numbers truncated), else a
/// global; or another script's variable (`[owner, name]`).
pub fn set<S: AsRef<str>>(target: &[S], v: f64, locals: &mut Locals, host: &mut dyn Host) {
    match target {
        [name] => {
            let name = name.as_ref();
            if !locals.set(name, v) {
                // Not a local: perhaps a global.
                host.set_var("", name, v, locals);
            }
        }
        [owner, name] => {
            host.set_var(owner.as_ref(), name.as_ref(), v, locals);
        }
        _ => {}
    }
}

/// A variable's value: a local, else whatever the host makes of the word
/// (`[name]`); or another script's (`[owner, name]`). Unknown ones are 0.
pub fn var<S: AsRef<str>>(path: &[S], locals: &mut Locals, host: &mut dyn Host) -> f64 {
    match path {
        [name] => {
            let name = name.as_ref();
            locals
                .get(name)
                .or_else(|| host.resolve(name))
                .unwrap_or(0.0)
        }
        [owner, name] => host
            .get_var(owner.as_ref(), name.as_ref(), locals)
            .unwrap_or(0.0),
        _ => 0.0,
    }
}

/// An operator applied to the value(s) before it: `y` is the top one, `x`
/// the one below (ignored by the unary ones). Comparisons and logic give 1
/// or 0; dividing by 0 gives 0.
pub fn apply(op: Op, x: f64, y: f64) -> f64 {
    let flag = |b: bool| if b { 1.0 } else { 0.0 };
    match op {
        Op::Neg => -y,
        Op::Not => flag(y == 0.0),
        Op::Or => flag(x != 0.0 || y != 0.0),
        Op::And => flag(x != 0.0 && y != 0.0),
        Op::Eq => flag(x == y),
        Op::Ne => flag(x != y),
        Op::Lt => flag(x < y),
        Op::Le => flag(x <= y),
        Op::Gt => flag(x > y),
        Op::Ge => flag(x >= y),
        Op::Add => x + y,
        Op::Sub => x - y,
        Op::Mul => x * y,
        Op::Div => {
            if y == 0.0 {
                0.0
            } else {
                x / y
            }
        }
        Op::Mod => {
            if y == 0.0 {
                0.0
            } else {
                (x as i64 % y as i64) as f64
            }
        }
    }
}

/// Calls a function on the reference it names (a `ref` variable, else
/// whatever the host makes of the word), or on the script's owner.
pub fn call_function(call: &Call, locals: &mut Locals, host: &mut dyn Host) -> Option<f64> {
    let on = call.on.as_ref().and_then(|word| {
        // A ref variable, else whatever the host makes of the word.
        locals
            .get(word)
            .or_else(|| host.resolve(word))
            .map(|v| v as u32)
    });
    host.call(call, on, locals)
}

/// Evaluates an expression on a stack, in its stored order: every operand
/// is worked out (`&&` and `||` don't skip their right side, as the
/// game's stored form has no way to), each operator takes the values
/// before it. Comparisons and logic give 1 or 0. When operands outnumber
/// operators (`GetStage Quest 110 == 1`) the value on top, the last one
/// worked out, is the answer: a guess at what the game does with the
/// ones left below. `None` when a function's value can't be worked out.
/// Unknown words and variables are 0.
pub fn eval(e: &Expr, locals: &mut Locals, host: &mut dyn Host) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::with_capacity(e.0.len());
    for item in &e.0 {
        let value = match item {
            Item::Number(n) => *n,
            Item::Str(_) => 0.0,
            Item::Var(path) => var(path, locals, host),
            Item::Call(call) => call_function(call, locals, host)?,
            Item::Op(op @ (Op::Neg | Op::Not)) => apply(*op, 0.0, stack.pop().unwrap_or(0.0)),
            Item::Op(op) => {
                let y = stack.pop().unwrap_or(0.0);
                let x = stack.pop().unwrap_or(0.0);
                apply(*op, x, y)
            }
        };
        stack.push(value);
    }
    Some(stack.pop().unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use std::collections::HashMap;

    /// A game with one quest: stages, a variable, and calls logged.
    #[derive(Default)]
    struct TestHost {
        stages: HashMap<String, f64>,
        quest_vars: HashMap<String, f64>,
        log: Vec<String>,
    }

    impl Host for TestHost {
        fn call(&mut self, call: &Call, _on: Option<u32>, _locals: &mut Locals) -> Option<f64> {
            let name = crate::function_name(call.function);
            let arg = |i: usize| match call.args.get(i) {
                Some(crate::Arg::Word(w)) => w.to_ascii_lowercase(),
                Some(crate::Arg::Number(n)) => n.to_string(),
                _ => String::new(),
            };
            Some(match name.as_str() {
                "GetStage" => self.stages.get(&arg(0)).copied().unwrap_or(0.0),
                "SetStage" => {
                    let v: f64 = arg(1).parse().unwrap_or(0.0);
                    self.stages.insert(arg(0), v);
                    0.0
                }
                "GetSecondsPassed" => 0.5,
                "GetDistance" => return None,
                other => {
                    self.log.push(other.to_string());
                    0.0
                }
            })
        }
        fn get_var(&mut self, owner: &str, name: &str, _locals: &Locals) -> Option<f64> {
            self.quest_vars
                .get(&format!("{owner}.{name}").to_ascii_lowercase())
                .copied()
        }
        fn set_var(&mut self, owner: &str, name: &str, value: f64, _locals: &mut Locals) -> bool {
            self.quest_vars
                .insert(format!("{owner}.{name}").to_ascii_lowercase(), value);
            true
        }
        fn resolve(&mut self, _word: &str) -> Option<f64> {
            None
        }
    }

    #[test]
    fn runs_the_timer_logic_of_a_quest_script() {
        let script = parse(
            "short bRunTimer\nfloat fTimer\n\
             Begin GameMode\n\
               if bRunTimer == 1\n\
                 if fTimer > 0\n\
                   set fTimer to fTimer - GetSecondsPassed\n\
                 elseif GetStage VCG01 == 0\n\
                   SetStage VCG01 1\n\
                   set bRunTimer to 0\n\
                 endif\n\
               endif\n\
             End",
        )
        .unwrap();
        let mut host = TestHost::default();
        let mut locals = Locals::new(&script);
        locals.set("bRunTimer", 1.0);
        locals.set("fTimer", 1.0);
        run_blocks(&script, "gamemode", |_| true, &mut locals, &mut host);
        assert_eq!(locals.get("fTimer"), Some(0.5));
        run_blocks(&script, "gamemode", |_| true, &mut locals, &mut host);
        run_blocks(&script, "gamemode", |_| true, &mut locals, &mut host);
        assert_eq!(host.stages.get("vcg01"), Some(&1.0));
        assert_eq!(locals.get("bruntimer"), Some(0.0));
    }

    #[test]
    fn evaluates_in_the_games_order() {
        let mut host = TestHost::default();
        host.stages.insert("vmq".into(), 50.0);
        let script = parse(
            "short a\nshort b\n\
             set a to 1 && 0 || 1\n\
             set b to GetStage VMQ 100 == 1",
        )
        .unwrap();
        let mut locals = Locals::new(&script);
        run(&script.body, &mut locals, &mut host);
        // `1 && (0 || 1)`.
        assert_eq!(locals.get("a"), Some(1.0));
        // The extra argument is compared, not the stage.
        assert_eq!(locals.get("b"), Some(0.0));
        let script = parse("float f\nset f to -(2 + 1) * 2 - -1").unwrap();
        let mut locals = Locals::new(&script);
        run(&script.body, &mut locals, &mut host);
        assert_eq!(locals.get("f"), Some(-5.0));
    }

    #[test]
    fn a_value_that_cant_be_worked_out_stops_the_run() {
        let script = parse(
            "short a\nshort b\n\
             set a to 1\n\
             if GetDistance Player < 500\n  SetStage VMQ 10\nendif\n\
             set b to 1",
        )
        .unwrap();
        let mut host = TestHost::default();
        let mut locals = Locals::new(&script);
        assert_eq!(run(&script.body, &mut locals, &mut host), Flow::Stopped);
        // Before it ran; the stage isn't set on a made-up distance.
        assert_eq!(locals.get("a"), Some(1.0));
        assert!(host.stages.is_empty());
        assert_eq!(locals.get("b"), Some(0.0));
        // As a statement, a function it can't do is skipped.
        let script = parse("short b\nPlayer.GetDistance Player\nset b to 1").unwrap();
        let mut locals = Locals::new(&script);
        assert_eq!(run(&script.body, &mut locals, &mut host), Flow::Done);
        assert_eq!(locals.get("b"), Some(1.0));
    }

    #[test]
    fn whole_number_variables_truncate_and_others_go_to_the_host() {
        let script =
            parse("short n\nset n to 7 / 2\nset VCG01.x to n + 1\nreturn\nset n to 9").unwrap();
        let mut host = TestHost::default();
        let mut locals = Locals::new(&script);
        assert_eq!(run(&script.body, &mut locals, &mut host), Flow::Returned);
        assert_eq!(locals.get("n"), Some(3.0));
        assert_eq!(host.quest_vars.get("vcg01.x"), Some(&4.0));
    }
}

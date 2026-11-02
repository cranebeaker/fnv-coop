//! Parsing script source into blocks of statements.
//!
//! Expressions are turned into the same form the game's compiler stores
//! (`SCDA`): operands and operators in evaluation order. The operator
//! order was read from the compiled conditions of the shipped scripts
//! (`nvinspect <Data> scripts --grep`):
//!
//! - unary minus (stored as `~`) and `!` bind tightest;
//! - then `*` `/` `%`, then `+` `-`;
//! - then the comparisons (`==` and `<` were never seen mixed, so they're
//!   one level here: unconfirmed);
//! - then `||`, and `&&` loosest of all: `a && b || c` is
//!   `a && (b || c)` (`VMS29a`'s bomber, the Lucky 38 terminal and
//!   `VMS19` conditions all compile that way);
//! - all left to right.
//!
//! Operands written side by side are each kept, as the game's compiler
//! does: `GetStage VMQYesMan01a 110 != 1` (`GetStage` takes one argument)
//! is stored `GetStage 110 1 !=`, and the comparison is between 110 and 1.

use std::fmt;

use crate::function;
use crate::lexer::{lex, Token};

/// A variable's type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarKind {
    /// `short`, `int`, `long`: whole numbers.
    Integer,
    Float,
    /// `ref`: a reference to an object.
    Ref,
}

/// A parsed script.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Script {
    pub name: Option<String>,
    pub variables: Vec<(VarKind, String)>,
    pub blocks: Vec<Block>,
    /// Statements outside any block (result scripts are just these).
    pub body: Vec<Stmt>,
}

/// `Begin <kind> [args]` … `End`.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Lower case: `gamemode`, `menumode`, `onactivate`, …
    pub kind: String,
    pub args: Vec<Arg>,
    pub body: Vec<Stmt>,
}

/// A function argument as written: a word (an editor ID, a variable, an
/// actor value or other name), a number or a string.
#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Word(String),
    Number(f64),
    Str(String),
}

/// A function call: `[on.]Function args…`.
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    /// The function's number (see [`crate::functions::FUNCTIONS`]).
    pub function: u16,
    /// The reference it's called on, as written (`Player`, a variable, an
    /// editor ID); `None` for the script's owner.
    pub on: Option<String>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    /// Unary minus.
    Neg,
    Not,
}

impl Op {
    /// How tightly it binds (higher first), as the game's compiler does.
    fn precedence(self) -> u8 {
        match self {
            Op::Neg | Op::Not => 7,
            Op::Mul | Op::Div | Op::Mod => 6,
            Op::Add | Op::Sub => 5,
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => 4,
            Op::Or => 3,
            Op::And => 2,
        }
    }

    fn unary(self) -> bool {
        matches!(self, Op::Neg | Op::Not)
    }

    /// How the game's compiled form writes it (unary minus is `~`).
    pub fn symbol(self) -> &'static str {
        match self {
            Op::Or => "||",
            Op::And => "&&",
            Op::Eq => "==",
            Op::Ne => "!=",
            Op::Lt => "<",
            Op::Le => "<=",
            Op::Gt => ">",
            Op::Ge => ">=",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::Neg => "~",
            Op::Not => "!",
        }
    }
}

/// One step of an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Number(f64),
    Str(String),
    /// A variable: a local (`[name]`), or another script's
    /// (`[quest or reference, name]`).
    Var(Vec<String>),
    Call(Call),
    /// Applies to the one (unary) or two values before it.
    Op(Op),
}

/// An expression in evaluation order: operands, each operator after the
/// values it works on (`a + b * c` is `a b c * +`), as the game stores it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Expr(pub Vec<Item>);

impl Expr {
    /// Its shape: `o` for each value, operators as the game stores them
    /// (`o o == o &&`), to compare with [`crate::compiled::expression_shape`].
    pub fn shape(&self) -> String {
        self.0
            .iter()
            .map(|item| match item {
                Item::Op(op) => op.symbol(),
                _ => "o",
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// One line's expression (an `if`'s condition, a `set`'s value).
pub fn parse_expression(text: &str) -> Result<Expr, String> {
    let tokens: Vec<Token> = lex(text).into_iter().flat_map(|(_, t)| t).collect();
    expr(&tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// `set <variable> to <expression>`.
    Set {
        target: Vec<String>,
        value: Expr,
    },
    /// `if` … `elseif` … `else` … `endif`.
    If {
        branches: Vec<(Expr, Vec<Stmt>)>,
        otherwise: Option<Vec<Stmt>>,
    },
    Return,
    Call(Call),
}

/// Why a script couldn't be parsed, and where.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// An `if` being built.
struct OpenIf {
    branches: Vec<(Expr, Vec<Stmt>)>,
    otherwise: Option<Vec<Stmt>>,
    /// Set by an `elseif` or `else` written after this if's `else`: those
    /// branches can never run, so what's in them is dropped.
    unreachable: Option<Vec<Stmt>>,
}

impl OpenIf {
    fn new(cond: Expr) -> OpenIf {
        OpenIf {
            branches: vec![(cond, Vec::new())],
            otherwise: None,
            unreachable: None,
        }
    }

    /// Where its statements go now.
    fn current(&mut self) -> &mut Vec<Stmt> {
        if let Some(list) = &mut self.unreachable {
            list
        } else if let Some(list) = &mut self.otherwise {
            list
        } else {
            &mut self.branches.last_mut().expect("an if has a branch").1
        }
    }

    fn close(self) -> Stmt {
        Stmt::If {
            branches: self.branches,
            otherwise: self.otherwise,
        }
    }
}

/// The ifs open in the block being filled, and where statements go.
struct Nesting<'a> {
    script: &'a mut Script,
    block: Option<Block>,
    ifs: Vec<OpenIf>,
}

impl Nesting<'_> {
    /// The list the next statement goes into: the innermost open branch.
    fn current(&mut self) -> &mut Vec<Stmt> {
        if let Some(open) = self.ifs.last_mut() {
            open.current()
        } else if let Some(b) = self.block.as_mut() {
            &mut b.body
        } else {
            &mut self.script.body
        }
    }

    fn push(&mut self, stmt: Stmt) {
        self.current().push(stmt);
    }
}

/// Parses a script's source.
pub fn parse(source: &str) -> Result<Script, ParseError> {
    let mut script = Script::default();
    let mut n = Nesting {
        script: &mut script,
        block: None,
        ifs: Vec::new(),
    };
    for (line, tokens) in lex(source) {
        let err = |message: String| ParseError { line, message };
        // A line of only punctuation (`-----` or `=====` rules between
        // sections, in the shipped scripts) compiles to nothing.
        if tokens.iter().all(|t| matches!(t, Token::Sym(_))) {
            continue;
        }
        let first = match &tokens[0] {
            Token::Word(w) => w.to_ascii_lowercase(),
            _ => String::new(),
        };
        // Whether the innermost open if can take another branch: not once
        // it's had its `else`.
        let can_branch = n
            .ifs
            .last()
            .is_some_and(|open| open.otherwise.is_none() && open.unreachable.is_none());
        match first.as_str() {
            "scn" | "scriptname" => {
                n.script.name = word_at(&tokens, 1);
            }
            "short" | "int" | "long" | "float" | "ref" | "reference" => {
                let kind = match first.as_str() {
                    "float" => VarKind::Float,
                    "ref" | "reference" => VarKind::Ref,
                    _ => VarKind::Integer,
                };
                let name =
                    word_at(&tokens, 1).ok_or_else(|| err("a variable without a name".into()))?;
                n.script.variables.push((kind, name));
            }
            "begin" => {
                if n.block.is_some() {
                    return Err(err("Begin inside a block".into()));
                }
                let kind = word_at(&tokens, 1)
                    .ok_or_else(|| err("Begin without a block type".into()))?
                    .to_ascii_lowercase();
                let args = tokens[2..].iter().filter_map(arg_of).collect();
                n.block = Some(Block {
                    kind,
                    args,
                    body: Vec::new(),
                });
            }
            "end" => {
                if !n.ifs.is_empty() {
                    return Err(err("End with an if still open".into()));
                }
                let b = n
                    .block
                    .take()
                    .ok_or_else(|| err("End without Begin".into()))?;
                n.script.blocks.push(b);
            }
            "if" => {
                let cond = expr(&tokens[1..]).map_err(err)?;
                n.ifs.push(OpenIf::new(cond));
            }
            // The game's compiler pairs `if` with `endif` by counting them
            // and gives each `elseif` / `else` to the innermost open `if`,
            // however the source is indented: the compiled form's skip
            // counts show it (the Lucky 38 terminal's and the Goodsprings
            // campfire's `else` spans stop at an `elseif` that the authors
            // meant for an inner `if`). It accepts `elseif` / `else` that
            // have no `if` to belong to, or that come after the `else`
            // (35 shipped scripts). How the game runs those isn't
            // confirmed; the guess: a branch after the `else` never runs
            // (one branch of an `if` has always run by then), and one with
            // no `if` open acts as a new `if` (an `else` as one whose
            // condition failed), which is what pairs up the `endif`s of
            // `SantiagoFXSCRIPT` and `CrNightkinCloakUntilFireScript`.
            "elseif" => {
                let cond = expr(&tokens[1..]).map_err(err)?;
                match n.ifs.last_mut() {
                    Some(open) if can_branch => open.branches.push((cond, Vec::new())),
                    Some(open) => open.unreachable = Some(Vec::new()),
                    None => n.ifs.push(OpenIf::new(cond)),
                }
            }
            "else" => {
                // Anything after `else` on its line is ignored, as the
                // compiler does (`else if (…)` in the campfire script).
                match n.ifs.last_mut() {
                    Some(open) if can_branch => open.otherwise = Some(Vec::new()),
                    Some(open) => open.unreachable = Some(Vec::new()),
                    None => {
                        let mut open = OpenIf::new(Expr(vec![Item::Number(0.0)]));
                        open.otherwise = Some(Vec::new());
                        n.ifs.push(open);
                    }
                }
            }
            "endif" => {
                // An `endif` with no `if` open (27 shipped scripts) does
                // nothing.
                if let Some(open) = n.ifs.pop() {
                    n.push(open.close());
                }
            }
            "set" => {
                let to = tokens
                    .iter()
                    .position(|t| matches!(t, Token::Word(w) if w.eq_ignore_ascii_case("to")))
                    .ok_or_else(|| err("set without to".into()))?;
                let target =
                    path(&tokens[1..to]).ok_or_else(|| err("set: not a variable".into()))?;
                let value = expr(&tokens[to + 1..]).map_err(err)?;
                n.push(Stmt::Set { target, value });
            }
            "return" => n.push(Stmt::Return),
            _ => {
                let mut p = Parser {
                    tokens: &tokens,
                    at: 0,
                };
                match p.operand().map_err(err)? {
                    Item::Call(call) => n.push(Stmt::Call(call)),
                    _ => return Err(err(format!("not a statement: {:?}", tokens[0]))),
                }
            }
        }
    }
    if !n.ifs.is_empty() {
        return Err(ParseError {
            line: 0,
            message: "an if is never closed".into(),
        });
    }
    if let Some(b) = n.block.take() {
        // A missing final End: keep the block.
        n.script.blocks.push(b);
    }
    Ok(script)
}

/// How many arguments a function takes: its table entry's parameters, and
/// for `ShowMessage` up to nine values after the message (they aren't in
/// its entry; the compiled form keeps them in a list of its own).
fn arg_count(sig: &crate::Signature) -> usize {
    if sig.name == "ShowMessage" {
        sig.params.len() + 9
    } else {
        sig.params.len()
    }
}

fn word_at(tokens: &[Token], i: usize) -> Option<String> {
    match tokens.get(i) {
        Some(Token::Word(w)) => Some(w.clone()),
        _ => None,
    }
}

fn arg_of(t: &Token) -> Option<Arg> {
    match t {
        Token::Word(w) => Some(Arg::Word(w.clone())),
        Token::Number(n) => Some(Arg::Number(*n)),
        Token::Str(s) => Some(Arg::Str(s.clone())),
        Token::Sym(_) => None,
    }
}

/// `name` or `owner.name`.
fn path(tokens: &[Token]) -> Option<Vec<String>> {
    match tokens {
        [Token::Word(a)] => Some(vec![a.clone()]),
        [Token::Word(a), Token::Sym("."), Token::Word(b)] => Some(vec![a.clone(), b.clone()]),
        _ => None,
    }
}

fn binary_op(s: &str) -> Option<Op> {
    Some(match s {
        "||" => Op::Or,
        "&&" => Op::And,
        "==" => Op::Eq,
        // A lone `=` reads as a comparison.
        "=" => Op::Eq,
        "!=" => Op::Ne,
        "<" => Op::Lt,
        "<=" => Op::Le,
        ">" => Op::Gt,
        ">=" => Op::Ge,
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        "%" => Op::Mod,
        _ => return None,
    })
}

/// An expression, in evaluation order (the shunting-yard way).
fn expr(tokens: &[Token]) -> Result<Expr, String> {
    let mut p = Parser { tokens, at: 0 };
    let mut out = Vec::new();
    // Operators waiting for their right-hand side; `None` is a `(`.
    let mut waiting: Vec<Option<Op>> = Vec::new();
    // Whether the last thing read was a value (so `-` is a subtraction).
    let mut after_value = false;
    while let Some(token) = tokens.get(p.at) {
        match token {
            Token::Sym("(") => {
                p.at += 1;
                waiting.push(None);
                after_value = false;
            }
            Token::Sym(")") => {
                p.at += 1;
                loop {
                    match waiting.pop() {
                        Some(Some(op)) => out.push(Item::Op(op)),
                        Some(None) => break,
                        None => return Err("a ) without its (".into()),
                    }
                }
                pop_unary(&mut waiting, &mut out);
                after_value = true;
            }
            Token::Sym(s @ ("-" | "!")) if !after_value => {
                p.at += 1;
                waiting.push(Some(if *s == "-" { Op::Neg } else { Op::Not }));
            }
            Token::Sym(s) => {
                p.at += 1;
                // Stray punctuation (a comma) is skipped.
                let Some(op) = binary_op(s) else { continue };
                while let Some(Some(top)) = waiting.last() {
                    if top.precedence() < op.precedence() {
                        break;
                    }
                    out.push(Item::Op(*top));
                    waiting.pop();
                }
                waiting.push(Some(op));
                after_value = false;
            }
            _ => {
                out.push(p.operand()?);
                pop_unary(&mut waiting, &mut out);
                after_value = true;
            }
        }
    }
    while let Some(w) = waiting.pop() {
        match w {
            Some(op) => out.push(Item::Op(op)),
            None => return Err("a ( without its )".into()),
        }
    }
    if out.is_empty() {
        return Err("an empty expression".into());
    }
    Ok(Expr(out))
}

/// Unary operators waiting apply to the value just read.
fn pop_unary(waiting: &mut Vec<Option<Op>>, out: &mut Vec<Item>) {
    while let Some(Some(top)) = waiting.last() {
        if !top.unary() {
            break;
        }
        out.push(Item::Op(*top));
        waiting.pop();
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
    at: usize,
}

impl Parser<'_> {
    fn peek_sym(&self) -> Option<&'static str> {
        match self.tokens.get(self.at) {
            Some(Token::Sym(s)) => Some(s),
            _ => None,
        }
    }

    /// A value: a number, a string, a variable or a function call with its
    /// arguments ([`arg_count`]).
    fn operand(&mut self) -> Result<Item, String> {
        let token = self.tokens.get(self.at).cloned();
        self.at += 1;
        match token {
            Some(Token::Number(n)) => Ok(Item::Number(n)),
            Some(Token::Str(s)) => Ok(Item::Str(s)),
            Some(Token::Word(first)) => {
                // `owner.name`: a function called on a reference, or
                // another script's variable.
                if self.peek_sym() == Some(".") {
                    if let Some(Token::Word(second)) = self.tokens.get(self.at + 1).cloned() {
                        self.at += 2;
                        if let Some((index, sig)) = function(&second) {
                            let args = self.args(arg_count(sig));
                            return Ok(Item::Call(Call {
                                function: index,
                                on: Some(first),
                                args,
                            }));
                        }
                        return Ok(Item::Var(vec![first, second]));
                    }
                }
                if let Some((index, sig)) = function(&first) {
                    let args = self.args(arg_count(sig));
                    return Ok(Item::Call(Call {
                        function: index,
                        on: None,
                        args,
                    }));
                }
                Ok(Item::Var(vec![first]))
            }
            other => Err(format!("unexpected {other:?}")),
        }
    }

    /// Up to `count` arguments: words, numbers (negative too) and strings,
    /// optionally separated by commas. An operator or the line's end stops
    /// them.
    fn args(&mut self, count: usize) -> Vec<Arg> {
        let mut out = Vec::new();
        while out.len() < count {
            match self.tokens.get(self.at) {
                Some(Token::Sym(",")) => self.at += 1,
                // A minus and a number while parameters remain: a negative
                // argument (`SetPos z -500`). Once the function has its
                // arguments a minus is a subtraction.
                Some(Token::Sym("-")) => match self.tokens.get(self.at + 1) {
                    Some(Token::Number(n)) => {
                        out.push(Arg::Number(-n));
                        self.at += 2;
                    }
                    _ => break,
                },
                Some(Token::Word(w)) => {
                    // `owner.name` as one argument.
                    if let (Some(Token::Sym(".")), Some(Token::Word(b))) =
                        (self.tokens.get(self.at + 1), self.tokens.get(self.at + 2))
                    {
                        out.push(Arg::Word(format!("{w}.{b}")));
                        self.at += 3;
                    } else {
                        out.push(Arg::Word(w.clone()));
                        self.at += 1;
                    }
                }
                Some(Token::Number(n)) => {
                    out.push(Arg::Number(*n));
                    self.at += 1;
                }
                Some(Token::Str(s)) => {
                    out.push(Arg::Str(s.clone()));
                    self.at += 1;
                }
                _ => break,
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rpn(source: &str) -> String {
        let e = expr(&crate::lexer::lex(source)[0].1).unwrap();
        e.0.iter()
            .map(|item| match item {
                Item::Number(n) => n.to_string(),
                Item::Str(s) => format!("{s:?}"),
                Item::Var(p) => p.join("."),
                Item::Call(c) => format!("{}()", crate::function_name(c.function)),
                Item::Op(op) => format!("{op:?}"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn parses_blocks_ifs_sets_and_calls() {
        let src = r#"
ScriptName TestScript
short nStage
float fTimer ; a comment

Begin GameMode
    if GetStage VCG01 == 36
        SetStage VCG01 40
    elseif fTimer > 0
        set fTimer to fTimer - GetSecondsPassed
    else
        Player.AddItem Caps001 10
    endif
End

BEGIN menumode 1036
    set VCG01.bRunTimer to 1
END
"#;
        let s = parse(src).unwrap();
        assert_eq!(s.name.as_deref(), Some("TestScript"));
        assert_eq!(
            s.variables,
            vec![
                (VarKind::Integer, "nStage".into()),
                (VarKind::Float, "fTimer".into())
            ]
        );
        assert_eq!(s.blocks.len(), 2);
        assert_eq!(s.blocks[1].kind, "menumode");
        assert_eq!(s.blocks[1].args, vec![Arg::Number(1036.0)]);
        let Stmt::If {
            branches,
            otherwise,
        } = &s.blocks[0].body[0]
        else {
            panic!("an if");
        };
        assert_eq!(branches.len(), 2);
        // GetStage takes its one argument, then the comparison.
        let items = &branches[0].0 .0;
        assert!(matches!(&items[0], Item::Call(c) if c.args == vec![Arg::Word("VCG01".into())]));
        assert_eq!(items[1..], [Item::Number(36.0), Item::Op(Op::Eq)]);
        let Stmt::Call(add) = &otherwise.as_ref().unwrap()[0] else {
            panic!()
        };
        assert_eq!(add.on.as_deref(), Some("Player"));
        assert_eq!(
            add.args,
            vec![Arg::Word("Caps001".into()), Arg::Number(10.0)]
        );
        let Stmt::Set { target, .. } = &s.blocks[1].body[0] else {
            panic!()
        };
        assert_eq!(target, &vec!["VCG01".to_string(), "bRunTimer".to_string()]);
    }

    #[test]
    fn result_scripts_are_bare_statements() {
        let s = parse("setstage VCG01 20\nset VCG01.bGiveTest to 1").unwrap();
        assert!(s.blocks.is_empty());
        assert_eq!(s.body.len(), 2);
    }

    #[test]
    fn reports_unclosed_ifs() {
        assert!(parse("Begin GameMode\nif 1\nEnd").is_err());
        assert!(parse("if 1\nset x to 1").is_err());
    }

    #[test]
    fn operators_group_as_the_games_compiler_groups_them() {
        // The usual arithmetic.
        assert_eq!(rpn("1 + 2 * 3 - 4"), "1 2 3 Mul Add 4 Sub");
        assert_eq!(rpn("a - 1 >= 8"), "a 1 Sub 8 Ge");
        // `||` binds tighter than `&&` (VMS29a's bomber condition).
        assert_eq!(
            rpn("a == 1 && b == 0 && c != 2 || d == 1"),
            "a 1 Eq b 0 Eq And c 2 Ne d 1 Eq Or And"
        );
        assert_eq!(rpn("a == 1 || a == 2 && b"), "a 1 Eq a 2 Eq Or b And");
        // Brackets and unary minus (stored after its value, as `~`).
        assert_eq!(rpn("(a || b) && -c < 1"), "a b Or c Neg 1 Lt And");
        assert_eq!(rpn("-1"), "1 Neg");
        // A function's extra argument is a value of its own.
        assert_eq!(
            rpn("(GetStage VMQYesMan01a 110 != 1)"),
            "GetStage() 110 1 Ne"
        );
        assert!(expr(&crate::lexer::lex("(a")[0].1).is_err());
    }

    #[test]
    fn stray_endifs_and_lines_of_punctuation_are_ignored() {
        let s = parse("Begin GameMode\nendif\n------\nset x to 1\n====\nendif\nEnd").unwrap();
        assert_eq!(s.blocks[0].body.len(), 1);
    }

    #[test]
    fn branches_belong_to_the_innermost_open_if_however_indented() {
        // The Lucky 38 terminal: the `elseif` meant for the inner `if`
        // goes to the outer one, after its `else`, so it never runs; its
        // `endif` closes the outer `if`; the next `elseif` has no `if` and
        // acts as a new one; the last `endif` is left over.
        let s = parse(
            "if a\nreturn\nelse\n  if b\n  set x to 1\n  endif\nelseif c\nset x to 2\nendif\n\
             elseif d\nset x to 3\nendif\nendif",
        )
        .unwrap();
        assert_eq!(s.body.len(), 2);
        let Stmt::If {
            branches,
            otherwise,
        } = &s.body[0]
        else {
            panic!()
        };
        assert_eq!(branches.len(), 1);
        assert_eq!(otherwise.as_ref().unwrap().len(), 1);
        let Stmt::If { branches, .. } = &s.body[1] else {
            panic!()
        };
        assert_eq!(branches[0].0 .0, vec![Item::Var(vec!["d".into()])]);
        // SantiagoFXSCRIPT: every `elseif` after an `endif` is an `if` of
        // its own.
        let s = parse(
            "if t > 1\nset b to 1\nendif\nelseif t > 2\nset b to 2\nendif\n\
             elseif t > 3\nset b to 3\nendif\nset c to 1",
        )
        .unwrap();
        assert_eq!(s.body.len(), 4);
        // CrNightkinCloakUntilFireScript: an `else` after an `endif` runs.
        let s = parse("if a\nset b to 1\nendif\nelse\nset b to 2\nendif").unwrap();
        assert_eq!(s.body.len(), 2);
        let Stmt::If { otherwise, .. } = &s.body[1] else {
            panic!()
        };
        assert_eq!(otherwise.as_ref().map(Vec::len), Some(1));
        // The campfire script: `else if (…)` is an `else`.
        let s = parse("if a\nset x to 1\nelse if (b)\nset x to 2\nendif").unwrap();
        let Stmt::If {
            branches,
            otherwise,
        } = &s.body[0]
        else {
            panic!()
        };
        assert_eq!((branches.len(), otherwise.as_ref().unwrap().len()), (1, 1));
    }
}

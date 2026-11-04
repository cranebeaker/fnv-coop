//! `fallout shared/expression.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CExpression` (Xbox PDB), size 0x710 and stack offsets checked
    /// against PC 00593490/00594b60/00594c60 and caller 005e1160.
    pub struct CExpression: 0x710 {
        0 m_ExpressionError: u32,
    }
}

// Translated from 00593490 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `CExpression::CExpression` (Xbox PDB): initializes the two stack indices,
/// then the error word. The infix/postfix and stack storage are untouched.
pub fn expression_construct(e: &mut Engine, this: Ptr<CExpression>) -> Ptr<CExpression> {
    e.call(0x0059_4b60, &args![this.addr() + 0x404]);
    e.call(0x0059_4c60, &args![this.addr() + 0x508]);
    e.set(this, CExpression::m_ExpressionError, 0);
    this
}

// Translated from 00594b60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Initializes the operator stack's index; its 64 data words remain intact.
pub fn operator_stack_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x100, u32::MAX);
    this
}

// Translated from 00594c60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Initializes the operand stack's index; its 32 operands remain intact.
pub fn operand_stack_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0x200, u32::MAX);
    this
}

/// Offsets inside `CExpression` (Xbox PDB), checked against the PC code:
/// the infix text (+4), the postfix text (+0x204), the operator stack
/// (+0x404: 64 words and the index at +0x100) and the operand stack
/// (+0x508: 32 operands of 16 bytes and the index at +0x200).
const INFIX: u32 = 0x4;
const POSTFIX: u32 = 0x204;
const OPERATORS: u32 = 0x404;
const OPERANDS: u32 = 0x508;

/// The operator table: 16 entries of 8 bytes, each a word (the operator's
/// number), a signed precedence byte (+4) and a text of up to 3 bytes (+5).
/// Number 16 means "operand" and 15 is unary minus (`~`).
const OPERATOR_TABLE: u32 = 0x0118_cad0;
const OPERATOR_PRECEDENCE: u32 = 0x0118_cad4;
const OPERATOR_TEXT: u32 = 0x0118_cad5;
const OPERAND: u32 = 0x10;
const NEGATE: u32 = 0xf;
/// The error names, one pointer per `CExpression` error code.
const ERROR_NAMES: u32 = 0x0118_c594;
/// `"%s"`, the format the error is reported with.
const PERCENT_S: u32 = 0x0101_9f08;
/// `SCRIPTS: Unhandled operator '%s' in Expression::Eval()`.
const UNHANDLED_OPERATOR: u32 = 0x0103_3fd0;

const MEMCPY: u32 = 0x0040_1460;
const STRLEN: u32 = 0x0044_a670;
const ERROR: u32 = 0x0040_fbe0;
const FLOAT_TO_LONG: u32 = 0x00ec_62c0;

/// x87 `FLD qword`: a signaling NaN is quieted (payload kept); every other
/// value is exact.
fn x87_load(value: f64) -> f64 {
    let bits = value.to_bits();
    if value.is_nan() && bits & 0x0008_0000_0000_0000 == 0 {
        f64::from_bits(bits | 0x0008_0000_0000_0000)
    } else {
        value
    }
}

/// A 16-byte operand as it sits on the operand stack: a type byte in the
/// low byte of word 0 (0 integer, otherwise double), word 1 unused, and the
/// value at word 2 (integer: word 2 only; double: words 2 and 3).
#[derive(Clone, Copy)]
struct Operand([u32; 4]);

impl Operand {
    fn read(e: &Engine, at: u32) -> Self {
        Operand([
            e.mem.u32(at),
            e.mem.u32(at + 4),
            e.mem.u32(at + 8),
            e.mem.u32(at + 12),
        ])
    }
    fn write(self, e: &mut Engine, at: u32) {
        for (i, word) in self.0.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *word);
        }
    }
    fn is_float(self) -> bool {
        self.0[0] as u8 != 0
    }
    fn int(self) -> i32 {
        self.0[2] as i32
    }
    fn float(self) -> f64 {
        f64::from_bits(self.0[2] as u64 | (self.0[3] as u64) << 32)
    }
    fn set_type(&mut self, float: bool) {
        self.0[0] = self.0[0] & !0xff | u32::from(float);
    }
    fn set_int(&mut self, value: i32) {
        self.0[2] = value as u32;
    }
    fn set_float(&mut self, value: f64) {
        let bits = value.to_bits();
        self.0[2] = bits as u32;
        self.0[3] = (bits >> 32) as u32;
    }
    /// The value as stored, for a memory operand of an x87 instruction
    /// (`FILD` of an integer is exact).
    fn memory_f64(self) -> f64 {
        if self.is_float() {
            self.float()
        } else {
            self.int() as f64
        }
    }
    /// The value as x87 loads it: `FILD dword` or `FLD qword`.
    fn as_f64(self) -> f64 {
        if self.is_float() {
            x87_load(self.float())
        } else {
            self.int() as f64
        }
    }
}

// Translated from 005934d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Sets the expression's error code and reports its name (`Error`,
/// `0040fbe0`, with `"%s"`). `RET 4`.
pub fn expression_set_error(e: &mut Engine, this: Ptr<CExpression>, code: u32) {
    e.set(this, CExpression::m_ExpressionError, code);
    let name = e.mem.u32(ERROR_NAMES.wrapping_add(code.wrapping_mul(4)));
    e.call(ERROR, &args![PERCENT_S, name]);
}

// Translated from 00593500 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 00593500 - stack cookie.
/// Converts an infix expression of `*length` bytes into the expression's
/// postfix buffer (+0x204), the shunting-yard way: the text is wrapped in
/// parentheses in the infix buffer (+4), read token by token (`00593870`),
/// operands are copied after a space, operators wait on the operator stack
/// by precedence (a `-` that follows no operand becomes unary minus `~`)
/// and are written after a space with their table text. Answers the
/// postfix buffer with `*length` set to its length, or null after setting
/// error 2 (operator stack full) or 5 (unbalanced). A closing parenthesis
/// pops until it pops number 0 without checking for an empty stack, as on
/// PC. The token buffer is 0x44 bytes, as on PC.
pub fn expression_to_postfix(
    e: &mut Engine,
    this: Ptr<CExpression>,
    infix: Ptr,
    length: Ptr,
) -> Ptr {
    let base = this.addr();
    e.set(this, CExpression::m_ExpressionError, 0);
    operator_clear(e, Ptr::new(base + OPERATORS));
    operand_clear(e, Ptr::new(base + OPERANDS));
    let mut write = base + INFIX;
    e.mem.set_u8(write, b'(');
    write += 1;
    let size = e.mem.u32(length.addr());
    e.call(MEMCPY, &args![write, infix, size]);
    write = write.wrapping_add(e.mem.u32(length.addr()));
    e.mem.set_u8(write, b')');
    e.mem.set_u8(write + 1, 0);
    let operators = Ptr::new(base + OPERATORS);
    e.with_stack(0x44 + 12, |e, work| {
        let token = work;
        let cursor = work.byte_add(0x44);
        let kind = work.byte_add(0x48);
        e.mem.set_u32(cursor.addr(), base + INFIX);
        let mut out = base + POSTFIX;
        e.mem.set_u8(out, 0);
        let mut follows_operand = false;
        let append_operator = |e: &mut Engine, out: &mut u32, operator: u32| {
            e.mem.set_u8(*out, b' ');
            *out = out.wrapping_add(1);
            let text = OPERATOR_TEXT.wrapping_add(operator.wrapping_mul(8));
            let size = e.call(STRLEN, &args![text]).u32();
            e.call(MEMCPY, &args![*out, text, size]);
            *out = out.wrapping_add(size);
        };
        loop {
            e.mem.set_u32(kind.addr(), OPERAND);
            let count = expression_next_token(e, this, cursor, token, kind, false, Ptr::NULL);
            if count != 0 {
                let current = e.mem.i32(kind.addr());
                if current < OPERAND as i32 {
                    match current {
                        0 => {
                            if operator_is_full(e, operators) {
                                expression_set_error(e, this, 2);
                                return Ptr::NULL;
                            }
                            operator_push(e, operators, 0);
                        }
                        1 => {
                            if operator_is_empty(e, operators) {
                                expression_set_error(e, this, 5);
                                return Ptr::NULL;
                            }
                            loop {
                                let operator = operator_pop(e, operators);
                                if operator == 0 {
                                    break;
                                }
                                append_operator(e, &mut out, operator);
                            }
                        }
                        _ => {
                            if current == 10 && !follows_operand {
                                e.mem.set_u32(kind.addr(), NEGATE);
                            }
                            let current = e.mem.u32(kind.addr());
                            while !operator_is_empty(e, operators) {
                                let operator = operator_pop(e, operators);
                                let top = e
                                    .mem
                                    .i8(OPERATOR_PRECEDENCE.wrapping_add(operator.wrapping_mul(8)));
                                let next = e
                                    .mem
                                    .i8(OPERATOR_PRECEDENCE.wrapping_add(current.wrapping_mul(8)));
                                if top < next {
                                    operator_push(e, operators, operator);
                                    break;
                                }
                                append_operator(e, &mut out, operator);
                            }
                            if operator_is_full(e, operators) {
                                expression_set_error(e, this, 2);
                                return Ptr::NULL;
                            }
                            operator_push(e, operators, current);
                        }
                    }
                } else {
                    e.mem.set_u8(out, b' ');
                    out = out.wrapping_add(1);
                    e.call(MEMCPY, &args![out, token, count]);
                    out = out.wrapping_add(count);
                }
            }
            let current = e.mem.i32(kind.addr());
            if current == OPERAND as i32 {
                follows_operand = true;
            } else if current < OPERAND as i32 && current > 1 {
                follows_operand = false;
            }
            if count == 0 {
                break;
            }
        }
        e.mem.set_u8(out, 0);
        if !operator_is_empty(e, operators) {
            expression_set_error(e, this, 5);
            return Ptr::NULL;
        }
        e.mem
            .set_u32(length.addr(), out.wrapping_sub(base + POSTFIX));
        Ptr::new(base + POSTFIX)
    })
}

// Translated from 00593870 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Copies the next token at `*cursor` into `out` (NUL-terminated), moves
/// the cursor past it and answers its length; `*kind` is the operator's
/// number, or 16 for an operand. Leading bytes below `!` (signed, so also
/// bytes 0x80 and above) are skipped, counted in `*spaces` when
/// `count_spaces` is set. Compiled operands are copied whole: `X` and a
/// script command opcode (0x1000..0x1280) with its parameter bytes (the
/// 16-bit length + 5), `s l f G Z` and an index (3 bytes), `n` (5), `z`
/// (9), a quoted string; `r` and an index start an operand that goes on.
/// Other bytes collect into an operand until a space, a tab or an
/// operator; an operator alone at the start is the token. A byte that
/// starts an operator's text without completing any sets error 5 and is
/// kept as operand text.
#[allow(clippy::too_many_arguments)] // PC receiver plus five stack words.
pub fn expression_next_token(
    e: &mut Engine,
    this: Ptr<CExpression>,
    cursor: Ptr,
    out: Ptr,
    kind: Ptr,
    count_spaces: bool,
    spaces: Ptr,
) -> u32 {
    let mut source = e.mem.u32(cursor.addr());
    let mut write = out.addr();
    let mut first = true;
    e.mem.set_u32(kind.addr(), OPERAND);
    while e.mem.i8(source) != 0 && e.mem.i8(source) < 0x21 {
        source = source.wrapping_add(1);
        if count_spaces && !spaces.is_null() {
            let count = e.mem.u32(spaces.addr());
            e.mem.set_u32(spaces.addr(), count.wrapping_add(1));
        }
    }
    let copy = |e: &mut Engine, write: &mut u32, source: &mut u32, count: u32| {
        for _ in 0..count {
            let byte = e.mem.u8(*source);
            e.mem.set_u8(*write, byte);
            *write = write.wrapping_add(1);
            *source = source.wrapping_add(1);
        }
    };
    loop {
        let byte = e.mem.u8(source);
        if byte == 0 {
            break;
        }
        if byte == b'X' {
            let opcode = e.mem.i16(source.wrapping_add(1));
            if (0x1000..0x1280).contains(&opcode) {
                let size = e.mem.i16(source.wrapping_add(3)).wrapping_add(5) as i32;
                e.call(MEMCPY, &args![write, source, size]);
                write = write.wrapping_add(size as u32);
                source = source.wrapping_add(size as u32);
                break;
            }
        }
        match byte {
            b's' | b'l' | b'f' | b'G' | b'Z' | b'r' => {
                copy(e, &mut write, &mut source, 3);
                if byte != b'r' {
                    break;
                }
                first = false;
                continue;
            }
            b'n' => {
                copy(e, &mut write, &mut source, 5);
                break;
            }
            b'z' => {
                copy(e, &mut write, &mut source, 9);
                break;
            }
            b'"' => {
                copy(e, &mut write, &mut source, 1);
                while e.mem.u8(source) != 0 && e.mem.u8(source) != b'"' {
                    copy(e, &mut write, &mut source, 1);
                }
                if e.mem.u8(source) == b'"' {
                    copy(e, &mut write, &mut source, 1);
                }
                break;
            }
            b' ' | b'\t' => break,
            _ => {}
        }
        let mut matched = false;
        let mut partial = false;
        let mut number = 0u32;
        while number < 0x10 {
            matched = true;
            let mut text = OPERATOR_TABLE + number * 8 + 5;
            let mut at = 0u32;
            while e.mem.u8(text) != 0 {
                if e.mem.u8(text) != e.mem.u8(source.wrapping_add(at)) {
                    matched = false;
                    break;
                }
                partial = true;
                at += 1;
                text += 1;
            }
            if matched {
                break;
            }
            number += 1;
        }
        e.mem.set_u32(kind.addr(), number);
        if matched {
            if first {
                let number = e.mem.u32(kind.addr());
                let mut text = OPERATOR_TEXT.wrapping_add(number.wrapping_mul(8));
                while e.mem.u8(text) != 0 {
                    let character = e.mem.u8(text);
                    e.mem.set_u8(write, character);
                    write = write.wrapping_add(1);
                    text = text.wrapping_add(1);
                    source = source.wrapping_add(1);
                }
            } else {
                e.mem.set_u32(kind.addr(), OPERAND);
            }
            break;
        }
        if partial {
            expression_set_error(e, this, 5);
        }
        e.mem.set_u8(write, byte);
        write = write.wrapping_add(1);
        first = false;
        source = source.wrapping_add(1);
    }
    e.mem.set_u8(write, 0);
    e.mem.set_u32(cursor.addr(), source);
    write.wrapping_sub(out.addr())
}

/// Integer division as `CDQ`/`IDIV`: the PC raises a divide error (the
/// game stops) for a zero divisor and for `i32::MIN / -1`.
fn idiv(dividend: i32, divisor: i32) -> (i32, i32) {
    match (dividend.checked_div(divisor), dividend.checked_rem(divisor)) {
        (Some(quotient), Some(remainder)) => (quotient, remainder),
        _ => panic!(
            "integer divide error (#DE) in CExpression::Eval 00593db0: {dividend} / {divisor}"
        ),
    }
}

// Translated from 00593db0 (PC disassembly, FalloutNV.exe 1.4.0.525).
// Translation incomplete: 00593db0 - stack cookie.
/// `CExpression::Eval` (Xbox PDB): evaluates `length` bytes of a compiled
/// (postfix) expression and answers the value in `ST0`, 0 after an error
/// (the error code is in the expression). Nothing runs when `length` is 0
/// or the expression already has an error; over 0x200 bytes is error 1.
/// The bytes are copied after a 0x48-byte token buffer (one stack block,
/// as on PC, so a token of 72 bytes or more reaches the "skip push" byte
/// at its offset 0x47 and then the copy). Operands: decimal integers
/// (`004b1230`, `strtol` `00eca6bd`), decimal floats (`004b12d0`, `atof`),
/// else a compiled operand (`005ac7a0` with the reference, container,
/// script and locals; in `parse` mode at the token's position in the
/// original bytes and without failing). Operators pop their operands
/// (error 3 when missing): logical, comparison and arithmetic operators in
/// integer arithmetic when both operands are integers, else in double; a
/// zero divisor is error 4, or with `parse` replaced by the double 1.0 (an
/// integer operand then reads that double's low word, 0). One operand
/// must remain (error 5 otherwise).
#[allow(clippy::too_many_arguments)] // PC receiver plus seven stack words.
pub fn expression_eval(
    e: &mut Engine,
    this: Ptr<CExpression>,
    data: Ptr,
    reference: u32,
    container: u32,
    script: u32,
    locals: u32,
    length: i32,
    parse: u8,
) -> f64 {
    if length == 0 || e.get(this, CExpression::m_ExpressionError) != 0 {
        return 0.0;
    }
    let operands = Ptr::new(this.addr() + OPERANDS);
    operand_clear(e, operands);
    if length > 0x200 {
        expression_set_error(e, this, 1);
        return 0.0;
    }
    // Stack locals of the PC frame: the token buffer (EBP-0x298, its byte
    // 0x47 is the "skip push" flag at EBP-0x251) directly before the copy
    // of the bytes (EBP-0x250); then the current operand (EBP-0x10), the
    // copy cursor, the source position, the kind and the operand offset.
    e.with_stack(0x48 + 0x208 + 16 + 16, |e, frame| {
        let token = frame.addr();
        let skip_push = token + 0x47;
        let copy = token + 0x48;
        let current = copy + 0x208;
        let cursor = current + 16;
        let position = cursor + 4;
        let kind = cursor + 8;
        let offset = cursor + 12;
        e.mem.set_u32(position, data.addr());
        e.mem.set_u32(kind, OPERAND);
        e.mem.set_u8(skip_push, 0);
        e.mem.set_u8(current, 0);
        e.mem.set_f64(current + 8, 0.0);
        e.call(MEMCPY, &args![copy, data, length]);
        e.mem.set_u8(copy.wrapping_add(length as u32), 0);
        e.mem.set_u32(cursor, copy);
        let next = |e: &mut Engine| {
            expression_next_token(
                e,
                this,
                Ptr::new(cursor),
                Ptr::new(token),
                Ptr::new(kind),
                parse != 0,
                Ptr::new(position),
            )
        };
        let mut size = next(e);
        let mut token_text = if size == 0 { 0 } else { token };
        while size != 0 {
            let operator = e.mem.i32(kind);
            if operator < OPERAND as i32 && operator > 1 {
                if operand_is_empty(e, operands) {
                    expression_set_error(e, this, 3);
                    return 0.0;
                }
                let right = e.with_stack(16, |e, out| {
                    operand_pop(e, operands, out);
                    Operand::read(e, out.addr())
                });
                let mut result = Operand::read(e, current);
                if operator == NEGATE as i32 {
                    if right.is_float() {
                        result.set_float(-x87_load(right.float()));
                    } else {
                        result.set_int(right.int().wrapping_neg());
                    }
                    result.0[0] = result.0[0] & !0xff | (right.0[0] & 0xff);
                } else {
                    if operand_is_empty(e, operands) {
                        expression_set_error(e, this, 3);
                        return 0.0;
                    }
                    let left = e.with_stack(16, |e, out| {
                        operand_pop(e, operands, out);
                        Operand::read(e, out.addr())
                    });
                    match binary(e, this, operator as u32, left, right, parse, token_text) {
                        Some(value) => {
                            let (float, words) = value;
                            result.set_type(float);
                            match words {
                                Value::Int(v) => result.set_int(v),
                                Value::Float(v) => result.set_float(v),
                            }
                        }
                        None => return 0.0,
                    }
                }
                result.write(e, current);
                operand_push(
                    e,
                    operands,
                    result.0[0],
                    result.0[1],
                    result.0[2],
                    result.0[3],
                );
            } else {
                if e.call(0x004b_1230, &args![token]).u32() != 0 {
                    e.mem.set_u8(current, 0);
                    let value = e.call(0x00ec_a6bd, &args![token]).u32();
                    e.mem.set_u32(current + 8, value);
                } else if e.call(0x004b_12d0, &args![token]).u32() != 0 {
                    e.mem.set_u8(current, 1);
                    let value = e.call(0x00ec_a573, &args![token]).f64();
                    e.mem.set_f64(current + 8, value);
                } else {
                    e.mem.set_u32(offset, 0);
                    e.mem.set_u8(current, 1);
                    if parse != 0 {
                        let at = e.mem.u32(position);
                        e.call(
                            0x005a_c7a0,
                            &args![
                                current + 8,
                                at,
                                offset,
                                reference,
                                container,
                                script,
                                locals,
                                parse as u32
                            ],
                        );
                    } else if !e
                        .call(
                            0x005a_c7a0,
                            &args![
                                current + 8,
                                token,
                                offset,
                                reference,
                                container,
                                script,
                                locals,
                                0u32
                            ],
                        )
                        .bool()
                    {
                        expression_set_error(e, this, 5);
                        return 0.0;
                    }
                }
                if e.mem.u8(skip_push) == 0 {
                    let operand = Operand::read(e, current);
                    operand_push(
                        e,
                        operands,
                        operand.0[0],
                        operand.0[1],
                        operand.0[2],
                        operand.0[3],
                    );
                } else {
                    e.mem.set_u8(skip_push, 0);
                }
            }
            let moved = e.mem.u32(position).wrapping_add(size);
            e.mem.set_u32(position, moved);
            size = next(e);
            token_text = if size == 0 { 0 } else { token };
        }
        if operand_is_empty(e, operands) {
            expression_set_error(e, this, 5);
            return 0.0;
        }
        e.with_stack(16, |e, out| {
            operand_pop(e, operands, out);
            let words = Operand::read(e, out.addr());
            words.write(e, current);
        });
        if !operand_is_empty(e, operands) {
            expression_set_error(e, this, 5);
            return 0.0;
        }
        Operand::read(e, current).as_f64()
    })
}

enum Value {
    Int(i32),
    Float(f64),
}

/// One binary operator of [`expression_eval`]: (double result, value), or
/// `None` after error 4. Integer results only replace the operand's low
/// value word, as on PC.
#[allow(clippy::too_many_arguments)]
fn binary(
    e: &mut Engine,
    this: Ptr<CExpression>,
    operator: u32,
    left: Operand,
    mut right: Operand,
    parse: u8,
    token: u32,
) -> Option<(bool, Value)> {
    let both_int = !left.is_float() && !right.is_float();
    let truth = |v: bool| Some((false, Value::Int(i32::from(v))));
    match operator {
        2 => {
            let l = if left.is_float() {
                left.as_f64() != 0.0
            } else {
                left.int() != 0
            };
            let r = if right.is_float() {
                right.as_f64() != 0.0
            } else {
                right.int() != 0
            };
            truth(l && r)
        }
        3 => {
            let l = if left.is_float() {
                left.as_f64() != 0.0
            } else {
                left.int() != 0
            };
            let r = if right.is_float() {
                right.as_f64() != 0.0
            } else {
                right.int() != 0
            };
            truth(l || r)
        }
        4..=9 => {
            let (l, r) = (left.as_f64(), right.as_f64());
            let value = if both_int {
                let (l, r) = (left.int(), right.int());
                match operator {
                    4 => l <= r,
                    5 => l < r,
                    6 => l >= r,
                    7 => l > r,
                    8 => l == r,
                    _ => l != r,
                }
            } else {
                match operator {
                    4 => l <= r,
                    5 => l < r,
                    6 => l >= r,
                    7 => l > r,
                    8 => l == r,
                    _ => l != r,
                }
            };
            truth(value)
        }
        10..=12 => {
            if both_int {
                let (l, r) = (left.int(), right.int());
                let v = match operator {
                    10 => l.wrapping_sub(r),
                    11 => l.wrapping_add(r),
                    _ => l.wrapping_mul(r),
                };
                return Some((false, Value::Int(v)));
            }
            let (l, r) = (left.as_f64(), right.memory_f64());
            let v = match operator {
                10 => x87_sub(x87_load(l), r),
                11 => x87_add(x87_load(l), r),
                _ => x87_mul(x87_load(l), r),
            };
            Some((true, Value::Float(v)))
        }
        13 | 14 => {
            let zero = if right.is_float() {
                right.as_f64() == 0.0
            } else {
                right.int() == 0
            };
            if zero {
                if parse == 0 {
                    expression_set_error(e, this, 4);
                    return None;
                }
                // FLD1 / FSTP qword over the right operand's value.
                right.set_float(1.0);
            }
            let divide = operator == 13;
            let to_int = |e: &mut Engine, v: f64| e.call(FLOAT_TO_LONG, &args![v]).u32() as i32;
            if both_int {
                let (quotient, remainder) = idiv(left.int(), right.int());
                return Some((false, Value::Int(if divide { quotient } else { remainder })));
            }
            let value = match (left.is_float(), right.is_float(), divide) {
                (_, _, true) => x87_div(x87_load(left.as_f64()), right.memory_f64()),
                (false, true, false) => {
                    let divisor = to_int(e, right.float());
                    idiv(left.int(), divisor).1 as f64
                }
                (true, false, false) => {
                    let dividend = to_int(e, left.float());
                    idiv(dividend, right.int()).1 as f64
                }
                _ => {
                    let dividend = to_int(e, left.float());
                    let divisor = to_int(e, right.float());
                    idiv(dividend, divisor).1 as f64
                }
            };
            Some((true, Value::Float(value)))
        }
        _ => {
            e.call(0x005b_5e40, &args![UNHANDLED_OPERATOR, token]);
            Some((false, Value::Int(0)))
        }
    }
}

// Translated from 00594b80 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Pushes a word on the operator stack (no bounds check). `RET 4`.
pub fn operator_push(e: &mut Engine, this: Ptr, value: u32) {
    let index = e.mem.u32(this.addr() + 0x100).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x100, index);
    e.mem
        .set_u32(this.addr().wrapping_add(index.wrapping_mul(4)), value);
}

// Translated from 00594bc0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Pops the operator stack's top word (no bounds check).
pub fn operator_pop(e: &mut Engine, this: Ptr) -> u32 {
    let index = e.mem.u32(this.addr() + 0x100);
    let value = e.mem.u32(this.addr().wrapping_add(index.wrapping_mul(4)));
    e.mem.set_u32(this.addr() + 0x100, index.wrapping_sub(1));
    value
}

// Translated from 00594c00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Whether the operator stack is empty (index -1); 0 or 1 in `EAX`.
pub fn operator_is_empty(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x100) == u32::MAX
}

// Translated from 00594c20 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Whether the operator stack is full (index + 1 == 64); 0 or 1 in `EAX`.
pub fn operator_is_full(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x100).wrapping_add(1) == 0x40
}

// Translated from 00594c40 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Empties the operator stack (index -1).
pub fn operator_clear(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x100, u32::MAX);
}

// Translated from 00594c80 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Pushes a 16-byte operand (four words by value) on the operand stack (no
/// bounds check). `RET 0x10`.
pub fn operand_push(e: &mut Engine, this: Ptr, word0: u32, word1: u32, word2: u32, word3: u32) {
    let index = e.mem.u32(this.addr() + 0x200).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x200, index);
    let slot = this.addr().wrapping_add(index.wrapping_mul(0x10));
    Operand([word0, word1, word2, word3]).write(e, slot);
}

// Translated from 00594cd0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Pops the top operand into `out` (no bounds check) and answers `out`.
/// `RET 4`.
pub fn operand_pop(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let index = e.mem.u32(this.addr() + 0x200);
    let slot = this.addr().wrapping_add(index.wrapping_mul(0x10));
    Operand::read(e, slot).write(e, out.addr());
    e.mem.set_u32(this.addr() + 0x200, index.wrapping_sub(1));
    out
}

// Translated from 00594d20 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Whether the operand stack is empty (index -1); 0 or 1 in `EAX`.
pub fn operand_is_empty(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x200) == u32::MAX
}

// Translated from 00594d40 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Empties the operand stack (index -1).
pub fn operand_clear(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x200, u32::MAX);
}

/// The uniform form of [`expression_eval`]: the result is in `ST0`.
fn expression_eval_entry(e: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        st0: expression_eval(
            e,
            Ptr::new(a[0]),
            Ptr::new(a[1]),
            a[2],
            a[3],
            a[4],
            a[5],
            a[6] as i32,
            a[7] as u8,
        ),
        ..Ret::default()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00593490,
            expression_construct(Ptr<CExpression>) -> Ptr<CExpression>
        ),
        entry!(0x00594b60, operator_stack_construct(Ptr) -> Ptr),
        entry!(0x00594c60, operand_stack_construct(Ptr) -> Ptr),
        entry!(0x005934d0, expression_set_error(Ptr<CExpression>, u32)),
        entry!(
            0x00593500,
            expression_to_postfix(Ptr<CExpression>, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00593870,
            expression_next_token(Ptr<CExpression>, Ptr, Ptr, Ptr, bool, Ptr) -> u32
        ),
        (0x00593db0, expression_eval_entry as AbiFn),
        entry!(0x00594b80, operator_push(Ptr, u32)),
        entry!(0x00594bc0, operator_pop(Ptr) -> u32),
        entry!(0x00594c00, operator_is_empty(Ptr) -> bool),
        entry!(0x00594c20, operator_is_full(Ptr) -> bool),
        entry!(0x00594c40, operator_clear(Ptr)),
        entry!(0x00594c80, operand_push(Ptr, u32, u32, u32, u32)),
        entry!(0x00594cd0, operand_pop(Ptr, Ptr) -> Ptr),
        entry!(0x00594d20, operand_is_empty(Ptr) -> bool),
        entry!(0x00594d40, operand_clear(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The operator table the code reads at 0118cad0 (number, signed
    /// precedence, text), and doubles for the error report.
    fn table_fixture(e: &mut Engine) {
        // The number literals go through the CRT's `strtol` (00ec8d08) and `_atof_l`
        // (00eca4c8), which read the initial locale's tables from the image; the bare
        // engine has no image, so both are doubled with decimal parsers.
        e.register(0x00ec_8d08, |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).to_string();
            let text = text.trim_start();
            let end = text
                .char_indices()
                .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && (c == '-' || c == '+'))))
                .map_or(text.len(), |(i, _)| i);
            text[..end].parse::<i32>().unwrap_or(0).into_ret()
        });
        e.register(0x00ec_a4c8, |e, a| {
            let text = String::from_utf8_lossy(&e.mem.cstr(a[0])).to_string();
            let text = text.trim_start();
            let end = text
                .char_indices()
                .find(|&(i, c)| {
                    !(c.is_ascii_digit() || c == '.' || (i == 0 && (c == '-' || c == '+')))
                })
                .map_or(text.len(), |(i, _)| i);
            Ret {
                st0: text[..end].parse::<f64>().unwrap_or(0.0),
                ..Ret::default()
            }
        });
        let operators: [(u8, &[u8]); 16] = [
            (0, b"("),
            (0, b")"),
            (1, b"&&"),
            (2, b"||"),
            (3, b"<="),
            (3, b"<"),
            (3, b">="),
            (3, b">"),
            (3, b"=="),
            (3, b"!="),
            (4, b"-"),
            (4, b"+"),
            (5, b"*"),
            (5, b"/"),
            (5, b"%"),
            (6, b"~"),
        ];
        e.map(OPERATOR_TABLE, 0x80);
        for (number, (precedence, text)) in operators.iter().enumerate() {
            let entry = OPERATOR_TABLE + number as u32 * 8;
            e.mem.set_u32(entry, number as u32);
            e.mem.set_u8(entry + 4, *precedence);
            e.mem.write(entry + 5, text);
        }
        e.map(ERROR_NAMES, 0x18);
        for code in 0..6 {
            e.mem.set_u32(ERROR_NAMES + code * 4, 0x5000 + code);
        }
        e.register_double(ERROR, |_, a| {
            assert_eq!(a[0], PERCENT_S);
            Ret::default()
        });
        e.register_double(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
    }

    fn expression(e: &mut Engine) -> Ptr<CExpression> {
        let this = e.new_object::<CExpression>();
        expression_construct(e, this);
        this
    }

    /// Evaluates `bytes` with dummy (nonzero) locals; (value bits, error).
    fn eval(bytes: &[u8], parse: u8) -> (u64, u32) {
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        let data = e.mem.alloc(bytes.len() as u32 + 1);
        e.mem.write(data, bytes);
        let value = e
            .call(
                0x00593db0,
                &args![
                    this,
                    data,
                    0u32,
                    0u32,
                    0u32,
                    1u32,
                    bytes.len() as i32,
                    parse
                ],
            )
            .f64();
        (value.to_bits(), e.get(this, CExpression::m_ExpressionError))
    }

    fn int_token(value: i32) -> Vec<u8> {
        let mut token = vec![b' ', b'n'];
        token.extend(value.to_le_bytes());
        token
    }

    fn float_token(value: f64) -> Vec<u8> {
        let mut token = vec![b' ', b'z'];
        token.extend(value.to_le_bytes());
        token
    }

    fn postfix(parts: &[&[u8]]) -> Vec<u8> {
        parts.concat()
    }

    #[test]
    fn eval_text_literals_in_integer_and_double_arithmetic() {
        let cases: &[(&[u8], f64)] = &[
            (b" 3 4 +", 7.0),
            (b" 3 4 -", -1.0),
            (b" 6 7 *", 42.0),
            (b" 7 2 /", 3.0),
            (b" 7 2 ~ /", -3.0),
            (b" 7 2 ~ %", 1.0),
            (b" 7 ~ 2 %", -1.0),
            (b" 1.5 2 *", 3.0),
            (b" 1 4.0 /", 0.25),
            (b" 7.9 2 %", 1.0),
            (b" 2 3 4 * +", 14.0),
            (b" 2147483647 1 +", -2147483648.0),
            (b" 0.5 ~", -0.5),
        ];
        for (bytes, expected) in cases {
            let (bits, error) = eval(bytes, 0);
            assert_eq!(error, 0, "{}", String::from_utf8_lossy(bytes));
            assert_eq!(
                f64::from_bits(bits),
                *expected,
                "{}",
                String::from_utf8_lossy(bytes)
            );
        }
    }

    #[test]
    fn eval_comparisons_and_logic_answer_integers() {
        let cases: &[(&[u8], f64)] = &[
            (b" 1 2 <", 1.0),
            (b" 2 2 <", 0.0),
            (b" 2 2 <=", 1.0),
            (b" 3 2 >", 1.0),
            (b" 2 2 >=", 1.0),
            (b" 2 2.0 ==", 1.0),
            (b" 2 2.5 !=", 1.0),
            (b" 1 0 &&", 0.0),
            (b" 1 0.5 &&", 1.0),
            (b" 0 0 ||", 0.0),
            (b" 0 0.0 ||", 0.0),
            (b" 0.0 3 ||", 1.0),
            (b" 1 2 < 3 4 > ||", 1.0),
        ];
        for (bytes, expected) in cases {
            let (bits, error) = eval(bytes, 0);
            assert_eq!(error, 0, "{}", String::from_utf8_lossy(bytes));
            assert_eq!(
                f64::from_bits(bits),
                *expected,
                "{}",
                String::from_utf8_lossy(bytes)
            );
        }
    }

    #[test]
    fn eval_compiled_integer_and_double_operands() {
        let nan = f64::NAN;
        let cases: Vec<(Vec<u8>, f64)> = vec![
            (postfix(&[&int_token(5), &int_token(-3), b" *"]), -15.0),
            (
                postfix(&[&float_token(0.1), &float_token(0.2), b" +"]),
                0.1 + 0.2,
            ),
            (
                postfix(&[&int_token(1), &float_token(3.0), b" /"]),
                1.0 / 3.0,
            ),
            (postfix(&[&float_token(nan), &int_token(0), b" =="]), 0.0),
            (postfix(&[&float_token(nan), &int_token(0), b" !="]), 1.0),
            (postfix(&[&float_token(nan), &int_token(1), b" <"]), 0.0),
            (postfix(&[&float_token(nan), &int_token(1), b" &&"]), 1.0),
        ];
        for (bytes, expected) in cases {
            let (bits, error) = eval(&bytes, 0);
            assert_eq!(error, 0, "{bytes:02x?}");
            assert_eq!(f64::from_bits(bits), expected, "{bytes:02x?}");
        }
    }

    #[test]
    fn eval_errors_answer_zero_with_their_code() {
        let cases: &[(&[u8], u32)] = &[
            (b" 1 0 /", 4),
            (b" 1 0 %", 4),
            (b" 1.0 0.0 /", 4),
            (b" 1 +", 3),
            (b" +", 3),
            (b" ~", 3),
            (b" 1 2", 5),
            (b" ", 5),
        ];
        for (bytes, code) in cases {
            let (bits, error) = eval(bytes, 0);
            assert_eq!(
                (bits, error),
                (0, *code),
                "{}",
                String::from_utf8_lossy(bytes)
            );
        }
        // Too long: error 1 before anything is read.
        let (bits, error) = eval(&[b' '; 0x201], 0);
        assert_eq!((bits, error), (0, 1));
    }

    #[test]
    fn eval_parse_mode_replaces_zero_divisors_with_one() {
        let cases: &[(&[u8], f64)] = &[
            (b" 3 0.0 /", 3.0),
            (b" 3 0.0 %", 0.0),
            (b" 3.5 0.0 /", 3.5),
            // The integer divisor reads the low word of 1.0 (0): FILD 0.
            (b" 3.5 0 /", f64::INFINITY),
        ];
        for (bytes, expected) in cases {
            let (bits, error) = eval(bytes, 1);
            assert_eq!(error, 0);
            assert_eq!(
                f64::from_bits(bits),
                *expected,
                "{}",
                String::from_utf8_lossy(bytes)
            );
        }
    }

    #[test]
    #[should_panic(expected = "integer divide error")]
    fn eval_parse_mode_integer_zero_divisor_faults_as_on_pc() {
        eval(b" 3 0 /", 1);
    }

    #[test]
    #[should_panic(expected = "integer divide error")]
    fn eval_remainder_of_a_truncated_zero_divisor_faults_as_on_pc() {
        // 7 % 0.5: the divisor truncates to 0 (`_ftol2`), IDIV faults.
        eval(
            &[int_token(7), float_token(0.5), b" %".to_vec()].concat(),
            0,
        );
    }

    #[test]
    fn eval_does_nothing_without_bytes_or_with_a_previous_error() {
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        e.mem.set_u32(this.addr() + OPERANDS + 0x200, 7);
        let data = e.mem.alloc(4);
        e.mem.write(data, b" 1 ");
        let words = |length: i32| args![this, data, 0u32, 0u32, 0u32, 1u32, length, 0u8];
        assert_eq!(e.call(0x00593db0, &words(0)).f64(), 0.0);
        e.set(this, CExpression::m_ExpressionError, 2);
        assert_eq!(e.call(0x00593db0, &words(3)).f64(), 0.0);
        assert_eq!(e.mem.u32(this.addr() + OPERANDS + 0x200), 7);
        assert_eq!(e.get(this, CExpression::m_ExpressionError), 2);
    }

    #[test]
    fn eval_passes_compiled_operands_and_context_to_the_operand_reader() {
        for parse in [0u8, 1] {
            let mut e = Engine::new();
            table_fixture(&mut e);
            let this = expression(&mut e);
            let bytes = b"  s\x02\x00 1 +";
            let data = e.mem.alloc(16);
            e.mem.write(data, bytes);
            e.register_double(0x005ac7a0, move |e, a| {
                assert_eq!(&a[3..], &[0x11, 0x22, 0x33, 0x44, u32::from(parse)]);
                assert_eq!(e.mem.u32(a[2]), 0);
                if parse == 0 {
                    assert_eq!(e.mem.bytes(a[1], 4), b"s\x02\x00\x00");
                } else {
                    // The original bytes at the token: two spaces skipped.
                    assert_eq!(a[1], data + 2);
                }
                e.mem.set_f64(a[0], 2.5);
                1u32.into_ret()
            });
            let value = e
                .call(
                    0x00593db0,
                    &args![this, data, 0x11u32, 0x22u32, 0x33u32, 0x44u32, 9i32, parse],
                )
                .f64();
            assert_eq!(value, 3.5);
        }
    }

    #[test]
    fn eval_a_failed_operand_read_is_error_five_outside_parse_mode() {
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        let data = e.mem.alloc(8);
        e.mem.write(data, b" s\x01\x00");
        e.register_double(0x005ac7a0, |_, _| 0u32.into_ret());
        let words = args![this, data, 0u32, 0u32, 0u32, 1u32, 4i32, 0u8];
        assert_eq!(e.call(0x00593db0, &words).f64(), 0.0);
        assert_eq!(e.get(this, CExpression::m_ExpressionError), 5);
    }

    #[test]
    fn eval_a_long_token_sets_the_skip_push_byte_as_on_pc() {
        // A command token of 72 bytes: its byte 0x47 lands in the flag that
        // suppresses one push, so the value is not pushed (error 5).
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        let mut bytes = vec![b' ', b'X', 0x00, 0x10, 67, 0];
        bytes.extend([7u8; 67]);
        let data = e.mem.alloc(bytes.len() as u32);
        e.mem.write(data, &bytes);
        e.register_double(0x005ac7a0, |e, a| {
            e.mem.set_f64(a[0], 1.0);
            1u32.into_ret()
        });
        let words = args![this, data, 0u32, 0u32, 0u32, 1u32, bytes.len() as i32, 0u8];
        assert_eq!(e.call(0x00593db0, &words).f64(), 0.0);
        assert_eq!(e.get(this, CExpression::m_ExpressionError), 5);
    }

    #[test]
    fn tokenizer_copies_compiled_operands_and_matches_operators() {
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        let source = e.mem.alloc(64);
        let cursor = e.mem.alloc(4);
        let out = e.mem.alloc(80);
        let kind = e.mem.alloc(4);
        let spaces = e.mem.alloc(4);
        let run = |e: &mut Engine, bytes: &[u8], count: bool| {
            e.mem.write(source, &[0; 64]);
            e.mem.write(source, bytes);
            e.mem.set_u32(cursor, source);
            e.mem.set_u32(spaces, 0);
            let size = e
                .call(0x00593870, &args![this, cursor, out, kind, count, spaces])
                .u32();
            (
                e.mem.bytes(out, size),
                e.mem.u32(kind),
                e.mem.u32(cursor) - source,
                e.mem.u32(spaces),
            )
        };
        let cases: &[(&[u8], &[u8], u32, u32)] = &[
            (b"  s\x01\x00+", b"s\x01\x00", 16, 5),
            (b"r\x02\x00s\x03\x00", b"r\x02\x00s\x03\x00", 16, 6),
            (b"n\x01\x02\x03\x04", b"n\x01\x02\x03\x04", 16, 5),
            (b"\"a b\"c", b"\"a b\"", 16, 5),
            (b"\"ab", b"\"ab", 16, 3),
            (b"<=1", b"<=", 4, 2),
            (b"<1", b"<", 5, 1),
            (b"12+3", b"12", 16, 2),
            (b"1.5 ", b"1.5", 16, 3),
            (b"X\x00\x10\x02\x00ab", b"X\x00\x10\x02\x00ab", 16, 7),
            (b"X\x00\x20", b"X", 16, 1),
            (b"\x80\x01~", b"~", 15, 3),
        ];
        for (bytes, token, kind, moved) in cases {
            let (got, got_kind, got_moved, _) = run(&mut e, bytes, false);
            assert_eq!(
                (got.as_slice(), got_kind, got_moved),
                (*token, *kind, *moved),
                "{bytes:02x?}"
            );
            assert_eq!(e.get(this, CExpression::m_ExpressionError), 0);
        }
        // Skipped bytes below '!' are counted only when asked.
        assert_eq!(run(&mut e, b" \t 1", true).3, 3);
        assert_eq!(run(&mut e, b" \t 1", false).3, 0);
        // A lone '&' begins "&&" only: error 5, kept as operand text.
        let (got, got_kind, _, _) = run(&mut e, b"& ", false);
        assert_eq!((got.as_slice(), got_kind), (&b"&"[..], 16));
        assert_eq!(e.get(this, CExpression::m_ExpressionError), 5);
    }

    #[test]
    fn postfix_orders_operators_by_precedence_and_marks_unary_minus() {
        let cases: &[(&[u8], &[u8])] = &[
            (b"1+2*3", b" 1 2 3 * +"),
            (b"(1+2)*3", b" 1 2 + 3 *"),
            (b"-1-2", b" 1 ~ 2 -"),
            (b"a&&b||c", b" a b c || &&"),
            (b"1 < 2 && 3", b" 1 2 < 3 &&"),
        ];
        for (infix, expected) in cases {
            let mut e = Engine::new();
            table_fixture(&mut e);
            let this = expression(&mut e);
            let text = e.mem.alloc(32);
            e.mem.write(text, infix);
            let length = e.mem.alloc(4);
            e.mem.set_u32(length, infix.len() as u32);
            let out = e.call(0x00593500, &args![this, text, length]).u32();
            assert_eq!(out, this.addr() + POSTFIX);
            let size = e.mem.u32(length);
            assert_eq!(
                e.mem.bytes(out, size),
                *expected,
                "{}",
                String::from_utf8_lossy(infix)
            );
            assert_eq!(
                e.mem.cstr(this.addr() + INFIX),
                [b"(", *infix, b")"].concat()
            );
        }
    }

    #[test]
    fn postfix_reports_unbalanced_parentheses() {
        for infix in [&b"(1+2"[..], b"1+2)"] {
            let mut e = Engine::new();
            table_fixture(&mut e);
            let this = expression(&mut e);
            let text = e.mem.alloc(8);
            e.mem.write(text, infix);
            let length = e.mem.alloc(4);
            e.mem.set_u32(length, infix.len() as u32);
            assert_eq!(e.call(0x00593500, &args![this, text, length]).u32(), 0);
            assert_eq!(e.get(this, CExpression::m_ExpressionError), 5);
            assert_eq!(e.mem.u32(length), infix.len() as u32);
        }
    }

    #[test]
    fn operator_and_operand_stacks_do_not_check_bounds() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x220);
        let operators = block + 4;
        e.call(0x00594c40, &[operators]);
        assert!(e.call(0x00594c00, &[operators]).bool());
        assert_eq!(e.call(0x00594c00, &[operators]).u32(), 1);
        e.call(0x00594b80, &[operators, 9]);
        assert_eq!(e.mem.u32(operators), 9);
        assert!(!e.call(0x00594c00, &[operators]).bool());
        e.mem.set_u32(operators + 0x100, 0x3f);
        assert_eq!(e.call(0x00594c20, &[operators]).u32(), 1);
        e.mem.set_u32(operators + 0x100, 0);
        assert_eq!(e.call(0x00594bc0, &[operators]).u32(), 9);
        // Popping the empty stack reads the word before it.
        e.mem.set_u32(block, 0x77);
        assert_eq!(e.call(0x00594bc0, &[operators]).u32(), 0x77);
        assert_eq!(e.mem.u32(operators + 0x100), (-2i32) as u32);

        let operands = e.mem.alloc(0x210);
        e.call(0x00594d40, &[operands]);
        assert_eq!(e.call(0x00594d20, &[operands]).u32(), 1);
        e.call(0x00594c80, &[operands, 1, 2, 3, 4]);
        assert_eq!(e.call(0x00594d20, &[operands]).u32(), 0);
        let out = e.mem.alloc(16);
        assert_eq!(e.call(0x00594cd0, &[operands, out]).u32(), out);
        assert_eq!(Operand::read(&e, out).0, [1, 2, 3, 4]);
        assert_eq!(e.mem.u32(operands + 0x200), u32::MAX);
    }

    #[test]
    fn set_error_stores_the_code_and_reports_its_name() {
        let mut e = Engine::new();
        table_fixture(&mut e);
        let this = expression(&mut e);
        e.register_double(ERROR, |_, a| {
            assert_eq!(a, [PERCENT_S, 0x5004]);
            Ret::default()
        });
        e.call(0x005934d0, &[this.addr(), 4]);
        assert_eq!(e.get(this, CExpression::m_ExpressionError), 4);
    }

    #[test]
    fn x87_nan_rules_match_the_native_cases() {
        // nv-call, PC 00593db0 on generated `z` operands (FCW 0x027F):
        // (left, right, operator, result).
        let cases: &[(u64, u64, &[u8], u64)] = &[
            (
                0x7ff8_0000_0000_0001,
                0xfff8_0000_0000_0100,
                b" +",
                0xfff8_0000_0000_0100,
            ),
            (
                0xfff8_0000_0000_0100,
                0x7ff8_0000_0000_0001,
                b" +",
                0xfff8_0000_0000_0100,
            ),
            (
                0x7ff8_0000_0000_0100,
                0xfff8_0000_0000_0100,
                b" -",
                0x7ff8_0000_0000_0100,
            ),
            (
                0xfff8_0000_0000_0100,
                0x7ff8_0000_0000_0100,
                b" *",
                0x7ff8_0000_0000_0100,
            ),
            (
                0x7ff0_0000_0000_0100,
                0x7ff8_0000_0000_0001,
                b" /",
                0x7ff8_0000_0000_0100,
            ),
            (
                0x7ff0_0000_0000_0001,
                0x7ff0_0000_0000_0100,
                b" +",
                0x7ff8_0000_0000_0001,
            ),
        ];
        for &(left, right, operator, expected) in cases {
            let bytes = [
                &float_token(f64::from_bits(left)),
                &float_token(f64::from_bits(right)),
                operator,
            ]
            .concat();
            assert_eq!(eval(&bytes, 0), (expected, 0), "{left:x} {right:x}");
        }
        // A lone signaling NaN comes out quiet; negation keeps the payload.
        let negated = [
            &float_token(f64::from_bits(0x7ff0_0000_0000_0001))[..],
            b" ~",
        ]
        .concat();
        assert_eq!(eval(&negated, 0), (0xfff8_0000_0000_0001, 0));
    }

    #[test]
    fn expression_initializes_only_error_and_indices_in_guarded_storage() {
        for fill in [0xa5, 0xcc, 0xff] {
            let mut e = Engine::new();
            let block = e.mem.alloc(0x718);
            let mut expected = vec![fill; 0x718];
            e.mem.write(block, &expected);
            expected[4..8].fill(0);
            expected[0x508..0x50c].fill(0xff);
            expected[0x70c..0x710].fill(0xff);
            assert_eq!(e.call(0x00593490, &[block + 4]).u32(), block + 4);
            assert_eq!(e.mem.bytes(block, 0x718), expected);
        }
    }

    #[test]
    fn expression_clears_error_after_ordered_stack_callbacks() {
        let mut e = Engine::new();
        let object = e.new_object::<CExpression>();
        e.set(object, CExpression::m_ExpressionError, 7);
        e.register_double(0x00594b60, move |e, a| {
            assert_eq!(a, [object.addr() + 0x404]);
            assert_eq!(e.get(object, CExpression::m_ExpressionError), 7);
            e.set(object, CExpression::m_ExpressionError, 8);
            Ret::default()
        });
        e.register_double(0x00594c60, move |e, a| {
            assert_eq!(a, [object.addr() + 0x508]);
            assert_eq!(e.get(object, CExpression::m_ExpressionError), 8);
            Ret::default()
        });
        assert_eq!(expression_construct(&mut e, object), object);
        assert_eq!(e.get(object, CExpression::m_ExpressionError), 0);
    }

    #[test]
    fn stack_constructors_preserve_data_and_outer_guards() {
        for (address, size) in [(0x00594b60, 0x104), (0x00594c60, 0x204)] {
            let mut e = Engine::new();
            let block = e.mem.alloc(size + 8);
            let mut expected = vec![0xa5; size as usize + 8];
            e.mem.write(block, &expected);
            expected[size as usize..size as usize + 4].fill(0xff);
            assert_eq!(e.call(address, &[block + 4]).u32(), block + 4);
            assert_eq!(e.mem.bytes(block, size + 8), expected);
        }
    }
}

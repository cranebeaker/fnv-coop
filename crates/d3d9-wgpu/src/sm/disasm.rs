//! A shader as text, in the style of Microsoft's `fxc /dumpbin`, each constant register named
//! from the shader's constant table (from the removed `crates/shaders`).

use super::{op, opcode_name, reg, Dst, Ins, Program, Src, Stage};
use std::fmt::Write;

const USAGES: [&str; 14] = [
    "position",
    "blendweight",
    "blendindices",
    "normal",
    "psize",
    "texcoord",
    "tangent",
    "binormal",
    "tessfactor",
    "positiont",
    "color",
    "fog",
    "depth",
    "sample",
];

/// The listing of a whole program.
pub fn listing(p: &Program) -> String {
    let mut text = format!("// {}, {} instructions", p.profile(), p.ins.len());
    if let Some(c) = &p.creator {
        let _ = write!(text, ", compiled by {c}");
    }
    text.push('\n');
    for c in &p.constants {
        let prefix = ["b", "i", "c", "s"].get(c.set as usize).unwrap_or(&"?");
        let _ = writeln!(
            text,
            "//   {:<24} {prefix}{}{}",
            c.name,
            c.index,
            if c.count > 1 {
                format!("-{prefix}{}", c.index + c.count - 1)
            } else {
                String::new()
            }
        );
    }
    let _ = writeln!(text, "{}", p.profile());
    let mut depth = 0usize;
    for ins in &p.ins {
        if matches!(ins.op, op::ELSE | op::ENDIF | op::ENDLOOP | op::ENDREP) {
            depth = depth.saturating_sub(1);
        }
        let _ = writeln!(text, "{}{}", "    ".repeat(depth), line(p, ins));
        if matches!(ins.op, op::IF | op::IFC | op::ELSE | op::LOOP | op::REP) {
            depth += 1;
        }
    }
    text
}

/// A register's name as the assembler writes it.
pub fn register_name(p: &Program, ty: u8, n: u32) -> String {
    let vertex = p.stage == Stage::Vertex;
    match ty {
        reg::TEMP => format!("r{n}"),
        reg::INPUT => format!("v{n}"),
        reg::CONST => format!("c{n}"),
        reg::ADDR_TEXTURE if vertex => format!("a{n}"),
        reg::ADDR_TEXTURE => format!("t{n}"),
        reg::RASTOUT => ["oPos", "oFog", "oPts"]
            .get(n as usize)
            .unwrap_or(&"oPts")
            .to_string(),
        reg::ATTROUT => format!("oD{n}"),
        reg::OUTPUT if vertex && p.major >= 3 => format!("o{n}"),
        reg::OUTPUT => format!("oT{n}"),
        reg::CONSTINT => format!("i{n}"),
        reg::COLOROUT => format!("oC{n}"),
        reg::DEPTHOUT => "oDepth".into(),
        reg::SAMPLER => format!("s{n}"),
        reg::CONST2 => format!("c{}", n + 2048),
        reg::CONST3 => format!("c{}", n + 4096),
        reg::CONST4 => format!("c{}", n + 6144),
        reg::CONSTBOOL => format!("b{n}"),
        reg::LOOP => "aL".into(),
        reg::TEMPFLOAT16 => format!("half{n}"),
        reg::MISC if n == 0 => "vPos".into(),
        reg::MISC => "vFace".into(),
        reg::LABEL => format!("l{n}"),
        reg::PREDICATE => format!("p{n}"),
        other => format!("?{other}_{n}"),
    }
}

fn dst_text(p: &Program, d: &Dst) -> String {
    let mut s = register_name(p, d.reg.ty, d.reg.n);
    if let Some(r) = d.rel {
        let _ = write!(
            s,
            "[{}.{}]",
            register_name(p, r.reg.ty, r.reg.n),
            ['x', 'y', 'z', 'w'][r.comp as usize]
        );
    }
    if d.mask != 0xF && d.mask != 0 {
        s.push('.');
        for (bit, c) in ['x', 'y', 'z', 'w'].into_iter().enumerate() {
            if d.mask & (1 << bit) != 0 {
                s.push(c);
            }
        }
    }
    s
}

fn src_text(p: &Program, s: &Src) -> String {
    let mut name = register_name(p, s.reg.ty, s.reg.n);
    if let Some(r) = s.rel {
        name = format!(
            "{name}[{}.{}]",
            register_name(p, r.reg.ty, r.reg.n),
            ['x', 'y', 'z', 'w'][r.comp as usize]
        );
    }
    let swizzle = if s.swizzle == 0xE4 {
        String::new()
    } else {
        let mut comps: Vec<char> = (0..4)
            .map(|k| ['x', 'y', 'z', 'w'][s.component(k) as usize])
            .collect();
        while comps.len() > 1 && comps[comps.len() - 1] == comps[comps.len() - 2] {
            comps.pop();
        }
        format!(".{}", comps.into_iter().collect::<String>())
    };
    match s.modifier {
        1 => format!("-{name}{swizzle}"),
        2 => format!("{name}_bias{swizzle}"),
        3 => format!("-{name}_bias{swizzle}"),
        4 => format!("{name}_bx2{swizzle}"),
        5 => format!("-{name}_bx2{swizzle}"),
        6 => format!("1 - {name}{swizzle}"),
        7 => format!("{name}_x2{swizzle}"),
        8 => format!("-{name}_x2{swizzle}"),
        9 => format!("{name}_dz{swizzle}"),
        10 => format!("{name}_dw{swizzle}"),
        11 => format!("|{name}|{swizzle}"),
        12 => format!("-|{name}|{swizzle}"),
        13 => format!("!{name}{swizzle}"),
        _ => format!("{name}{swizzle}"),
    }
}

fn modifiers(d: &Dst) -> String {
    let mut s = String::new();
    if d.sat {
        s.push_str("_sat");
    }
    if d.pp {
        s.push_str("_pp");
    }
    if d.centroid {
        s.push_str("_centroid");
    }
    s
}

/// The constant's name a register belongs to, from the constant table.
fn constant_note(p: &Program, s: &Src) -> Option<String> {
    let (set, index) = match s.reg.ty {
        reg::CONST => (2, s.reg.n),
        reg::CONSTINT => (1, s.reg.n),
        reg::CONSTBOOL => (0, s.reg.n),
        reg::SAMPLER => (3, s.reg.n),
        _ => return None,
    };
    p.constants.iter().find_map(|c| {
        let start = u32::from(c.index);
        let end = start + u32::from(c.count.max(1));
        (c.set == set && (start..end).contains(&index)).then(|| {
            if c.count > 1 {
                format!("{}[{}]", c.name, index - start)
            } else {
                c.name.clone()
            }
        })
    })
}

/// One instruction as text.
pub fn line(p: &Program, ins: &Ins) -> String {
    let Some(base) = opcode_name(ins.op) else {
        return format!("unknown_{}", ins.op);
    };
    let mut name = base.to_string();
    let mut operands: Vec<String> = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    match ins.op {
        op::DCL => {
            let (d, decl) = (ins.dst.expect("a dcl's register"), ins.decl.expect("usage"));
            match d.reg.ty {
                reg::SAMPLER => name.push_str(match decl.texture {
                    2 => "_2d",
                    3 => "_cube",
                    4 => "_volume",
                    _ => "",
                }),
                reg::MISC => {}
                _ if p.stage == Stage::Pixel && p.major < 3 => {}
                _ => {
                    name.push('_');
                    name.push_str(USAGES.get(decl.usage as usize).unwrap_or(&"usage"));
                    if decl.index > 0 {
                        name.push_str(&decl.index.to_string());
                    }
                }
            }
            name.push_str(&modifiers(&d));
            operands.push(dst_text(p, &d));
        }
        op::DEF => {
            operands.push(dst_text(p, &ins.dst.expect("def's register")));
            operands.extend(ins.values.iter().map(|&v| format!("{}", f32::from_bits(v))));
        }
        op::DEFI => {
            operands.push(dst_text(p, &ins.dst.expect("defi's register")));
            operands.extend(ins.values.iter().map(|&v| format!("{}", v as i32)));
        }
        op::DEFB => {
            operands.push(dst_text(p, &ins.dst.expect("defb's register")));
            operands.push(if ins.values[0] != 0 { "true" } else { "false" }.into());
        }
        _ => {
            if ins.op == op::TEX {
                name = match ins.controls {
                    1 => "texldp".into(),
                    2 => "texldb".into(),
                    _ => "texld".into(),
                };
            }
            if matches!(ins.op, op::IFC | op::BREAKC | op::SETP) {
                name.push_str(match ins.controls {
                    1 => "_gt",
                    2 => "_eq",
                    3 => "_ge",
                    4 => "_lt",
                    5 => "_ne",
                    6 => "_le",
                    _ => "",
                });
            }
            if let Some(d) = &ins.dst {
                name.push_str(&modifiers(d));
                operands.push(dst_text(p, d));
            }
            for s in &ins.src {
                operands.push(src_text(p, s));
                notes.extend(constant_note(p, s));
            }
            if let Some(pr) = &ins.predicate {
                name = format!("({}) {name}", src_text(p, pr));
            }
        }
    }
    let mut line = name;
    if !operands.is_empty() {
        line.push(' ');
        line.push_str(&operands.join(", "));
    }
    notes.dedup();
    if !notes.is_empty() {
        line = format!("{line:<44} // {}", notes.join(", "));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::small_vertex_shader};

    #[test]
    fn a_listing_reads_as_the_assembler_writes() {
        let p = parse(&small_vertex_shader()).unwrap();
        let text = super::listing(&p);
        assert!(text.contains("dcl_position v0"), "{text}");
        assert!(text.contains("m4x4 oPos, v0, c0"), "{text}");
        assert!(text.contains("mov oT0.xy, v0.yx"), "{text}");
    }
}

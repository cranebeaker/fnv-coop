// Writes a function card (Markdown and JSON) for each requested function.
//
// Args: addr=<hex>[,<hex>...] out=<dir> [depth=8]
//
// A card holds what an analyst needs before reading a function: address,
// size, prototype, calling convention, callers and callees, the globals it
// touches with their current bytes, and every constant that floating-point
// instructions read, decoded by machine at the access size the instruction
// uses. Nobody should convert 0x40600000 to 3.5 by hand.
//
// It also counts instruction classes and assigns a CPU-fidelity tier:
//   A  integer and/or SSE arithmetic and square root only
//   B  x87 arithmetic, compares or float-to-int stores
//   C  SSE reciprocal approximations (RSQRTSS/RSQRTPS/RCPSS/RCPPS)
//   D  x87 transcendentals, or calls to CRT math functions
// The tier is the highest class present. "tier" counts the function's own
// code and its calls to math routines by name. "tier_with_callees" also
// walks the callees, and theirs, down to depth levels (default 8),
// applying the instruction classes and the math-routine names at every
// level. A routine that FunctionID or a name import called _CIsin or
// sin counts as tier D wherever it is reached, whether it is called
// directly or through the import table ("call dword ptr [__imp__cos]").
//
// The tier is a heuristic for choosing how to test a function, not a proof.
// "tier_is_lower_bound" is true when something that could raise
// tier_with_callees was not looked at: an indirect call whose target is not
// known (a call through an absolute import-table slot is known), a direct
// call to an address with no function, an imported routine with no usable
// name, or callees below the depth limit. Writes through pointers and code
// reached by jump tables are not followed.
//
// The cards describe a program you own. Keep them in the private research
// tree. Never commit them.
//
// @category NV

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import ghidra.app.script.GhidraScript;
import ghidra.framework.Application;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressRange;
import ghidra.program.model.lang.Register;
import ghidra.program.model.listing.Data;
import ghidra.program.model.symbol.FlowType;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.listing.Program;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.pcode.PcodeOp;
import ghidra.program.model.pcode.Varnode;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.RefType;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.util.task.TaskMonitor;

public class NvFunctionCard extends GhidraScript {
    private static final String ME = "NvFunctionCard";

    // ---- instruction classes (Ghidra mnemonics are upper case) ----
    private static final Set<String> X87_TRANSCENDENTAL =
        Set.of("FSIN", "FCOS", "FSINCOS", "FPATAN", "FPTAN", "F2XM1", "FYL2X", "FYL2XP1", "FSCALE");
    private static final Set<String> X87_ARITH = Set.of("FADD", "FADDP", "FIADD", "FSUB", "FSUBP",
        "FISUB", "FSUBR", "FSUBRP", "FISUBR", "FMUL", "FMULP", "FIMUL", "FDIV", "FDIVP", "FIDIV",
        "FDIVR", "FDIVRP", "FIDIVR", "FSQRT", "FABS", "FCHS", "FRNDINT", "FPREM", "FPREM1",
        "FXTRACT");
    private static final Set<String> X87_COMPARE = Set.of("FCOM", "FCOMP", "FCOMPP", "FUCOM",
        "FUCOMP", "FUCOMPP", "FICOM", "FICOMP", "FCOMI", "FCOMIP", "FUCOMI", "FUCOMIP", "FTST",
        "FXAM");
    private static final Set<String> X87_CONVERT = Set.of("FIST", "FISTP", "FISTTP", "FBSTP");
    private static final Set<String> X87_CONTROL = Set.of("FLDCW", "FNSTCW", "FSTCW", "FNSTSW",
        "FSTSW", "FINIT", "FNINIT", "FCLEX", "FNCLEX", "FLDENV", "FNSTENV", "FSTENV", "FNSAVE",
        "FSAVE", "FRSTOR", "FXSAVE", "FXRSTOR");
    private static final Set<String> SSE_CONTROL = Set.of("LDMXCSR", "STMXCSR", "VLDMXCSR", "VSTMXCSR");
    private static final Set<String> NOT_DATA_READS = Set.of("FXSAVE", "FXRSTOR", "FLDENV", "FNSTENV",
        "FSTENV", "FNSAVE", "FSAVE", "FRSTOR", "FEMMS");

    private static final Pattern SSE_ARITH = Pattern.compile(
        "^V?(ADD|SUB|MUL|DIV|MIN|MAX|SQRT|HADD|HSUB|ADDSUB|DP|ROUND)(SS|SD|PS|PD)$" +
            "|^V?CMP(EQ|LT|LE|UNORD|NEQ|NLT|NLE|ORD)?(SS|SD|PS|PD)$" +
            "|^V?U?COMI(SS|SD)$|^V?CVT\\w+$|^VF(N)?M(ADD|SUB)\\d+(SS|SD|PS|PD)$");
    // Conversions: V?CVTT?<source>2<destination>. The source says what a memory operand holds.
    private static final Pattern SSE_CVT =
        Pattern.compile("^V?CVTT?(SS|SD|PS|PD|SI|DQ|PI)2(SS|SD|PS|PD|SI|DQ|PI)$");
    // SSE instructions that can have no XMM operand: conversions to a general or MMX register.
    private static final Pattern SSE_NO_XMM =
        Pattern.compile("^V?CVTT?(SS|SD)2SI$|^V?CVTT?(PS|PD)2PI$|^V?U?COMIS[SD]$");
    private static final Pattern SSE_APPROX = Pattern.compile("^V?(RSQRT|RCP)(SS|PS)$");
    private static final Pattern SSE_FLOAT_OTHER = Pattern.compile("^V?[A-Z0-9]+(SS|SD|PS|PD)$");
    // Integer-named moves and bit masks that are often used on float data: a constant loaded with
    // MOVQ xmm0,[m64] or masked with PAND is a float constant all the same.
    private static final Pattern SSE_INT_MOVE =
        Pattern.compile("^V?(MOVQ|MOVD|MOVDQA|MOVDQU|PAND|PANDN|POR|PXOR)$");

    // ---- CRT math routines by name ----
    private static final Pattern MATH_TRANSCENDENTAL = Pattern.compile(
        "^_{0,2}(CI)?(a?(sin|cos|tan)h?|atan2?|exp2?|log(2|10)?|pow|expm1|log1p)f?$" +
            "|^__?libm_sse2_\\w+$",
        Pattern.CASE_INSENSITIVE);
    // An import with no real name (Ghidra calls it Ordinal_<n>) cannot be classified.
    private static final Pattern UNNAMED_IMPORT = Pattern.compile("^(Ordinal_\\d+|EXTERNAL.*|<EXTERNAL>.*)$");
    private static final int MAX_CALLEE_FUNCTIONS = 5000;
    private static final int MAX_REASONS_LISTED = 25;
    private static final Pattern MATH_OTHER = Pattern.compile(
        "^_{0,2}(CI)?(sqrt|floor|ceil|fabs|fmod|ldexp|frexp|modf|hypot|cbrt)f?$" +
            "|^_{0,2}ftol2?(_sse)?(_sse2)?$",
        Pattern.CASE_INSENSITIVE);

    private Program prog;
    private Listing listing;
    private SymbolTable symbols;
    private FunctionManager fm;
    private int maxDepth;

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(ME, getScriptArgs(), Set.of("addr", "out", "depth"));
        String addrList = NvCommon.require(ME, args, "addr");
        Path outDir = Paths.get(NvCommon.require(ME, args, "out"));
        maxDepth = NvCommon.intArg(ME, args, "depth", 8, 1, 32);
        if (currentProgram == null) {
            throw new IllegalStateException(ME + ": no program is open");
        }
        prog = currentProgram;
        listing = prog.getListing();
        symbols = prog.getSymbolTable();
        fm = prog.getFunctionManager();

        // Resolve every address before writing anything.
        List<Function> targets = new ArrayList<>();
        for (String t : addrList.split(",")) {
            if (t.trim().isEmpty()) {
                continue;
            }
            long v = NvCommon.parseHex(ME, "addr", t);
            Address a = toAddr(v);
            Function f = fm.getFunctionAt(a);
            if (f == null || f.isExternal()) {
                Function in = fm.getFunctionContaining(a);
                throw new IllegalArgumentException(ME + ": no function starts at " + NvCommon.addr(a) +
                    (in != null ? " (it is inside " + in.getName() + " at " + NvCommon.addr(in.getEntryPoint()) + ")"
                        : ""));
            }
            targets.add(f);
        }
        if (targets.isEmpty()) {
            throw new IllegalArgumentException(ME + ": addr list is empty");
        }
        Files.createDirectories(outDir);
        for (Function f : targets) {
            Map<String, Object> card = buildCard(f);
            String entry = NvCommon.addr(f.getEntryPoint());
            NvCommon.writeText(outDir.resolve(entry + ".json"), NvCommon.toPrettyJson(card));
            NvCommon.writeText(outDir.resolve(entry + ".md"), markdown(card));
            println(ME + ": " + entry + " " + f.getName() + " tier " + card.get("tier") + " -> " +
                outDir.resolve(entry + ".md"));
        }
    }

    // ------------------------------------------------------------------
    // Card data
    // ------------------------------------------------------------------

    /** Per-function scan results, also used for callees. */
    private static final class Scan {
        final Map<String, Integer> classes = new LinkedHashMap<>();
        final TreeMap<String, Integer> fpMnemonics = new TreeMap<>();
        int total;
        int callsDirect;
        int callsIndirect; // every computed call
        int callsIndirectResolved; // computed calls whose target is a known function
        final List<String> unresolvedCalls = new ArrayList<>();
        // Functions reached by direct calls and by computed calls that resolve to a function.
        final Set<Function> callees = new LinkedHashSet<>();
        final List<String> writesGlobals = new ArrayList<>();
        final Set<String> transcendentalSeen = new TreeSet<>();

        int n(String k) {
            return classes.getOrDefault(k, 0);
        }

        void add(String k) {
            classes.merge(k, 1, Integer::sum);
        }

        /** Computed calls whose target is not known. */
        int indirectUnresolved() {
            return callsIndirect - callsIndirectResolved;
        }
    }

    private Scan scan(Function f) {
        Scan s = new Scan();
        for (String k : new String[] { "x87_arithmetic", "x87_compare", "x87_convert_store",
            "x87_transcendental", "x87_load_store", "x87_control", "sse_arithmetic",
            "sse_approximation", "sse_move_logic", "sse_control" }) {
            s.classes.put(k, 0);
        }
        for (Instruction ins : listing.getInstructions(f.getBody(), true)) {
            s.total++;
            String mn = ins.getMnemonicString().toUpperCase();
            String cls = classify(ins, mn);
            if (cls != null) {
                s.add(cls);
                s.fpMnemonics.merge(mn, 1, Integer::sum);
                if (cls.equals("x87_transcendental")) {
                    s.transcendentalSeen.add(mn);
                }
            }
            if (ins.getFlowType().isCall()) {
                if (ins.getFlowType().isComputed()) {
                    s.callsIndirect++;
                    resolveComputedCall(ins, s);
                }
                else {
                    s.callsDirect++;
                    for (Address t : ins.getFlows()) {
                        Function callee = fm.getFunctionAt(t);
                        if (callee != null) {
                            s.callees.add(callee);
                        }
                        else {
                            s.unresolvedCalls.add(NvCommon.addr(t));
                        }
                    }
                }
            }
            for (Reference ref : ins.getReferencesFrom()) {
                if (ref.isMemoryReference() && ref.getReferenceType().isWrite() &&
                    !NvCommon.inExecutableMemory(prog, ref.getToAddress())) {
                    s.writesGlobals.add(NvCommon.addr(ref.getToAddress()));
                }
            }
        }
        return s;
    }

    /**
     * A computed call through an absolute address, such as the import-table call
     * "call dword ptr [__imp__cos]", has a known target: Ghidra links the table
     * slot to the function it holds (an external function for an import). The
     * target is added to the callees and the call counts as resolved. Calls
     * through a register or an indexed table have no such target and stay
     * unresolved. A slot in writable memory can be changed by the program, so an
     * internal function found there is added to the callees but the call still
     * counts as unresolved.
     */
    private void resolveComputedCall(Instruction ins, Scan s) {
        for (Object o : ins.getOpObjects(0)) {
            if (o instanceof Register) {
                return;
            }
        }
        boolean resolved = false;
        for (Reference ref : ins.getReferencesFrom()) {
            if (!ref.isMemoryReference() && !ref.isExternalReference()) {
                continue;
            }
            Address to = ref.getToAddress();
            Function target = fm.getReferencedFunction(to);
            if (target == null) {
                continue;
            }
            // The operand holds a pointer to the function. A reference that is the function's
            // own entry would mean the call reads code bytes as a pointer.
            if (target.getEntryPoint().equals(to) && !to.isExternalAddress()) {
                continue;
            }
            s.callees.add(target);
            MemoryBlock slot = prog.getMemory().getBlock(to);
            if (target.isExternal() || slot == null || !slot.isWrite()) {
                resolved = true;
            }
        }
        if (resolved) {
            s.callsIndirectResolved++;
        }
    }

    private static String classify(Instruction ins, String mn) {
        if (mn.startsWith("F") && !mn.equals("FEMMS") && !mn.startsWith("FXSAVE") &&
            !mn.startsWith("FXRSTOR")) {
            if (X87_TRANSCENDENTAL.contains(mn)) {
                return "x87_transcendental";
            }
            if (X87_ARITH.contains(mn)) {
                return "x87_arithmetic";
            }
            if (X87_COMPARE.contains(mn) || mn.startsWith("FCMOV")) {
                return "x87_compare";
            }
            if (X87_CONVERT.contains(mn)) {
                return "x87_convert_store";
            }
            if (X87_CONTROL.contains(mn)) {
                return "x87_control";
            }
            return "x87_load_store";
        }
        if (X87_CONTROL.contains(mn)) {
            return "x87_control"; // FXSAVE, FXRSTOR
        }
        if (SSE_CONTROL.contains(mn)) {
            return "sse_control";
        }
        // Conversions to a general register (CVTTSS2SI eax,[m32]) and compares have
        // no XMM operand when the other operand is in memory, but they are SSE.
        if (SSE_NO_XMM.matcher(mn).matches()) {
            return "sse_arithmetic";
        }
        // String instructions share names such as MOVSD and CMPSD with SSE, so
        // an SSE name only counts when an XMM or YMM register is an operand.
        if (!hasVectorRegister(ins)) {
            return null;
        }
        if (SSE_APPROX.matcher(mn).matches()) {
            return "sse_approximation";
        }
        if (SSE_ARITH.matcher(mn).matches()) {
            return "sse_arithmetic";
        }
        if (SSE_FLOAT_OTHER.matcher(mn).matches() || SSE_INT_MOVE.matcher(mn).matches()) {
            return "sse_move_logic";
        }
        return null;
    }

    private static boolean hasVectorRegister(Instruction ins) {
        for (int i = 0; i < ins.getNumOperands(); i++) {
            Register r = ins.getRegister(i);
            if (r != null) {
                String n = r.getName().toUpperCase();
                if (n.startsWith("XMM") || n.startsWith("YMM") || n.startsWith("ZMM")) {
                    return true;
                }
            }
        }
        return false;
    }

    private Map<String, Object> buildCard(Function f) throws Exception {
        Map<String, Object> c = NvCommon.obj();
        c.put("tool", ME);
        c.put("exe_sha256", NvCommon.exeSha256(prog, ME));
        c.put("ghidra_version", Application.getApplicationVersion());
        c.put("program_name", prog.getName());
        c.put("entry", NvCommon.addr(f.getEntryPoint()));
        c.put("name", f.getName());
        c.put("name_source", f.getSymbol().getSource().name());
        List<String> tags = new ArrayList<>();
        f.getTags().forEach(t -> tags.add(t.getName()));
        java.util.Collections.sort(tags);
        c.put("tags", tags);
        List<Object> ranges = new ArrayList<>();
        for (AddressRange rg : f.getBody().getAddressRanges()) {
            ranges.add(List.of(NvCommon.addr(rg.getMinAddress()), NvCommon.addr(rg.getMaxAddress())));
        }
        c.put("body", ranges);
        c.put("size", f.getBody().getNumAddresses());
        c.put("prototype", f.getPrototypeString(false, true));
        c.put("calling_convention", f.getCallingConventionName());
        c.put("param_count", f.getParameterCount());
        int purge = f.getStackPurgeSize();
        c.put("stack_purge", purge == Function.UNKNOWN_STACK_DEPTH_CHANGE ||
            purge == Function.INVALID_STACK_DEPTH_CHANGE ? null : (Object) purge);
        c.put("is_thunk", f.isThunk());
        c.put("callers", functionList(f.getCallingFunctions(TaskMonitor.DUMMY)));
        c.put("callees", functionList(f.getCalledFunctions(TaskMonitor.DUMMY)));

        // ---- data operands ----
        TreeMap<String, Map<String, Object>> globals = new TreeMap<>();
        TreeMap<String, Map<String, Object>> codePointers = new TreeMap<>();
        TreeMap<String, Map<String, Object>> floatConsts = new TreeMap<>();
        List<Object> immediates = new ArrayList<>();
        Map<String, Set<Integer>> globalSizes = new TreeMap<>();
        Map<String, Set<String>> globalAccess = new TreeMap<>();
        Map<String, List<String>> globalInstrs = new TreeMap<>();

        for (Instruction ins : listing.getInstructions(f.getBody(), true)) {
            String mn = ins.getMnemonicString().toUpperCase();
            String cls = classify(ins, mn);
            boolean fpInstruction = cls != null && !NOT_DATA_READS.contains(mn);

            for (Reference ref : ins.getReferencesFrom()) {
                RefType rt = ref.getReferenceType();
                if (!ref.isMemoryReference() || rt.isFlow()) {
                    continue;
                }
                Address to = ref.getToAddress();
                String ta = NvCommon.addr(to);
                if (!prog.getMemory().contains(to)) {
                    continue;
                }
                int pcodeSize = pcodeAccessSize(ins, to);
                int size = pcodeSize > 0 ? pcodeSize : textAccessSize(ins, to);
                boolean exactAddress = pcodeSize > 0;

                if (NvCommon.inExecutableMemory(prog, to)) {
                    Map<String, Object> cp = NvCommon.obj();
                    cp.put("address", ta);
                    cp.put("label", namedLabel(to));
                    cp.put("function", fm.getFunctionAt(to) != null);
                    codePointers.putIfAbsent(ta, cp);
                    continue;
                }

                globalSizes.computeIfAbsent(ta, k -> new TreeSet<>());
                if (size > 0) {
                    globalSizes.get(ta).add(size);
                }
                globalAccess.computeIfAbsent(ta, k -> new TreeSet<>()).add(rt.getName());
                globalInstrs.computeIfAbsent(ta, k -> new ArrayList<>())
                    .add(NvCommon.addr(ins.getAddress()) + " " + mn);

                // Constants read by floating-point instructions.
                if (fpInstruction && rt.isRead() && size > 0 && size <= 16) {
                    String kind = primaryKind(mn, size);
                    String key = ta + ":" + size + ":" + kind;
                    Map<String, Object> e = floatConsts.get(key);
                    if (e == null) {
                        e = floatConstant(to, size, kind);
                        if (e == null) {
                            continue;
                        }
                        e.put("exact_address", exactAddress);
                        e.put("instructions", new ArrayList<String>());
                        floatConsts.put(key, e);
                    }
                    @SuppressWarnings("unchecked")
                    List<String> uses = (List<String>) e.get("instructions");
                    String use = NvCommon.addr(ins.getAddress()) + " " + mn;
                    if (!uses.contains(use)) {
                        uses.add(use);
                    }
                }
            }
            possibleFloatImmediate(ins, mn, immediates);
        }

        // Globals: one entry per address, with the sizes the code used.
        for (String ta : globalAccess.keySet()) {
            Address to = toAddr(Long.parseLong(ta, 16));
            Map<String, Object> g = NvCommon.obj();
            g.put("address", ta);
            g.put("label", namedLabel(to));
            MemoryBlock b = prog.getMemory().getBlock(to);
            g.put("block", b == null ? "" : b.getName());
            g.put("writable_block", b != null && b.isWrite());
            g.put("access", new ArrayList<>(globalAccess.get(ta)));
            Set<Integer> sizes = globalSizes.get(ta);
            g.put("access_sizes", new ArrayList<>(sizes));
            int size = sizes.isEmpty() ? 0 : sizes.stream().mapToInt(Integer::intValue).max().getAsInt();
            if (size == 0) {
                Data d = listing.getDataAt(to);
                if (d != null && d.isDefined() && d.getLength() <= 16) {
                    size = d.getLength();
                    g.put("size_source", "defined_data");
                }
            }
            else {
                g.put("size_source", "instruction");
            }
            g.put("size", size == 0 ? null : (Object) size);
            if (size > 0 && b != null && b.isInitialized()) {
                byte[] bytes = NvCommon.readBytes(prog, to, Math.min(size, 16));
                g.put("current_bytes", NvCommon.bytesToHex(bytes));
                g.put("interpretations", interpretations(bytes));
            }
            else {
                g.put("current_bytes", null);
                g.put("interpretations", NvCommon.obj());
            }
            List<String> instrs = globalInstrs.get(ta);
            g.put("instructions", instrs.size() > 12 ? new ArrayList<>(instrs.subList(0, 12)) : instrs);
            globals.put(ta, g);
        }

        c.put("globals", new ArrayList<>(globals.values()));
        c.put("code_pointers", new ArrayList<>(codePointers.values()));
        c.put("float_constants", new ArrayList<>(floatConsts.values()));
        c.put("possible_float_immediates", immediates);

        // ---- instruction classes and tier ----
        Scan own = scan(f);
        Map<String, Object> counts = NvCommon.obj();
        counts.put("instructions", own.total);
        for (Map.Entry<String, Integer> e : own.classes.entrySet()) {
            counts.put(e.getKey(), e.getValue());
        }
        counts.put("fldcw", own.fpMnemonics.getOrDefault("FLDCW", 0));
        counts.put("ldmxcsr", own.fpMnemonics.getOrDefault("LDMXCSR", 0) +
            own.fpMnemonics.getOrDefault("VLDMXCSR", 0));
        counts.put("calls_direct", own.callsDirect);
        counts.put("calls_indirect", own.callsIndirect);
        counts.put("calls_indirect_resolved", own.callsIndirectResolved);
        c.put("instruction_counts", counts);
        c.put("fp_mnemonics", new TreeMap<>(own.fpMnemonics));

        List<String> mathTrans = new ArrayList<>();
        List<String> mathOther = new ArrayList<>();
        mathCallees(own, mathTrans, mathOther);
        c.put("crt_math_callees_transcendental", mathTrans);
        c.put("crt_math_callees_other", mathOther);

        List<String> reasons = new ArrayList<>();
        String tier = tierOf(own, mathTrans, mathOther, reasons, "own code");
        c.put("tier", tier);
        c.put("tier_reasons", reasons);

        // The callees, their callees and so on, down to maxDepth levels.
        CalleeWalk walk = new CalleeWalk(tier);
        if (own.indirectUnresolved() > 0) {
            walk.lowerBound.add("own code: " + own.indirectUnresolved() + " indirect call(s), targets unknown");
        }
        if (!own.unresolvedCalls.isEmpty()) {
            walk.lowerBound.add("own code: direct call(s) to " + own.unresolvedCalls +
                " where no function exists");
        }
        walk.run(f, own);
        c.put("tier_with_callees", walk.tier);
        c.put("tier_with_callees_reasons", walk.reasons);
        c.put("tier_callee_depth_limit", maxDepth);
        c.put("tier_callee_functions_scanned", walk.scanned);
        c.put("tier_is_lower_bound", !walk.lowerBound.isEmpty());
        c.put("tier_lower_bound_reasons", walk.lowerBound);
        c.put("changes_fpu_control", own.n("x87_control") > 0 || own.n("sse_control") > 0);

        // ---- stateful ----
        List<String> st = new ArrayList<>();
        for (String g : new TreeSet<>(own.writesGlobals)) {
            st.add("writes global " + g);
        }
        if (own.callsDirect + own.callsIndirect > 0) {
            st.add("calls other functions (" + own.callsDirect + " direct, " + own.callsIndirect +
                " indirect)");
        }
        c.put("stateful", !st.isEmpty());
        c.put("stateful_reasons", st);
        c.put("stateful_note", "writes through pointers are not detected");
        return c;
    }

    /** The function a call reaches: a thunk is followed to the routine it stands for. */
    private static Function resolveThunk(Function callee) {
        return callee.isThunk() ? callee.getThunkedFunction(true) : callee;
    }

    /** Sorts the direct callees of a scan into CRT transcendental and other math routines, by name. */
    private static void mathCallees(Scan s, List<String> transcendental, List<String> other) {
        for (Function callee : s.callees) {
            Function target = resolveThunk(callee);
            String name = target == null ? callee.getName() : target.getName();
            // An import has no address in the image; it is shown as ext:<library>::<name>.
            String where = callee.isExternal() ? "ext:" + callee.getName(true)
                : NvCommon.addr(callee.getEntryPoint()) + " " + name;
            if (MATH_TRANSCENDENTAL.matcher(name).matches()) {
                transcendental.add(where);
            }
            else if (MATH_OTHER.matcher(name).matches()) {
                other.add(where);
            }
        }
    }

    /**
     * Breadth-first walk over direct callees with a visited set, a depth
     * limit and a function limit. Breadth-first order reaches each function
     * by its shortest call chain, so the depth limit never cuts off a
     * function that a shorter chain would have reached.
     */
    private final class CalleeWalk {
        final Set<Address> visited = new LinkedHashSet<>();
        final List<String> reasons = new ArrayList<>();
        final List<String> lowerBound = new ArrayList<>();
        String tier;
        int scanned;
        private final Map<Address, String> cut = new LinkedHashMap<>();
        private boolean limitHit;
        private int reasonsDropped;
        private int boundDropped;

        CalleeWalk(String tier) {
            this.tier = tier;
        }

        // A function deep in a call graph can have hundreds of callees with
        // reasons; only the first ones are listed, with a count of the rest.
        void addReasons(List<String> more) {
            for (String r : more) {
                if (reasons.size() < MAX_REASONS_LISTED) {
                    reasons.add(r);
                }
                else {
                    reasonsDropped++;
                }
            }
        }

        void addBound(String r) {
            if (lowerBound.size() < MAX_REASONS_LISTED) {
                lowerBound.add(r);
            }
            else {
                boundDropped++;
            }
        }

        /** Scans the callees of the root function, then theirs, and so on. */
        void run(Function root, Scan rootScan) {
            visited.add(root.getEntryPoint());
            ArrayDeque<Object[]> queue = new ArrayDeque<>();
            queue.add(new Object[] { rootScan, 0 });
            while (!queue.isEmpty()) {
                Object[] item = queue.poll();
                Scan s = (Scan) item[0];
                int depth = (Integer) item[1];
                for (Function callee : s.callees) {
                    Function target = resolveThunk(callee);
                    if (target == null) {
                        addBound("callee " + NvCommon.addr(callee.getEntryPoint()) + " " +
                            callee.getName() + ": thunk target unknown");
                        continue;
                    }
                    if (target.isExternal()) {
                        // Its name was already matched against the math patterns by the
                        // caller's scan; its code is not in the image.
                        if (UNNAMED_IMPORT.matcher(target.getName()).matches()) {
                            // A call through the import table reaches the import itself; a
                            // call to a stub in the image reaches it through that stub.
                            String who = callee.isExternal() ? "import " + target.getName(true)
                                : "callee " + NvCommon.addr(callee.getEntryPoint()) + " imports " +
                                    target.getName(true);
                            addBound(who + ", which has no usable name");
                        }
                        continue;
                    }
                    Address entry = target.getEntryPoint();
                    if (visited.contains(entry)) {
                        continue;
                    }
                    if (depth + 1 > maxDepth) {
                        cut.putIfAbsent(entry, NvCommon.addr(entry) + " " + target.getName());
                        continue;
                    }
                    if (scanned >= MAX_CALLEE_FUNCTIONS) {
                        limitHit = true;
                        continue;
                    }
                    visited.add(entry);
                    scanned++;
                    Scan s2 = scan(target);
                    List<String> trans = new ArrayList<>();
                    List<String> other = new ArrayList<>();
                    mathCallees(s2, trans, other);
                    String where = "callee " + NvCommon.addr(entry) + " " + target.getName() + " (depth " +
                        (depth + 1) + ")";
                    List<String> r2 = new ArrayList<>();
                    String t2 = tierOf(s2, trans, other, r2, where);
                    if (t2.compareTo(tier) > 0) {
                        tier = t2;
                    }
                    if (!t2.equals("A")) {
                        addReasons(r2);
                    }
                    if (s2.indirectUnresolved() > 0) {
                        addBound(where + ": " + s2.indirectUnresolved() + " indirect call(s), targets unknown");
                    }
                    if (!s2.unresolvedCalls.isEmpty()) {
                        addBound(where + ": direct call(s) to " + s2.unresolvedCalls +
                            " where no function exists");
                    }
                    queue.add(new Object[] { s2, depth + 1 });
                }
            }
            cut.keySet().removeAll(visited);
            if (reasonsDropped > 0) {
                reasons.add("... and " + reasonsDropped + " more reason(s) not listed");
            }
            if (boundDropped > 0) {
                lowerBound.add("... and " + boundDropped + " more not listed");
            }
            if (!cut.isEmpty()) {
                lowerBound.add("depth limit " + maxDepth + ": " + cut.size() +
                    " callee(s) below it were not scanned (first: " + cut.values().iterator().next() + ")");
            }
            if (limitHit) {
                lowerBound.add("scan stopped after " + MAX_CALLEE_FUNCTIONS + " functions");
            }
        }
    }

    private static String tierOf(Scan s, List<String> mathTrans, List<String> mathOther,
            List<String> reasons, String where) {
        String tier = "A";
        if (s.n("x87_arithmetic") + s.n("x87_compare") + s.n("x87_convert_store") > 0) {
            tier = "B";
            reasons.add(where + ": x87 arithmetic/compare/store-to-int (" + (s.n("x87_arithmetic") +
                s.n("x87_compare") + s.n("x87_convert_store")) + " instructions)");
        }
        if (!mathOther.isEmpty()) {
            tier = tier.compareTo("B") < 0 ? "B" : tier;
            reasons.add(where + ": calls CRT math/conversion " + mathOther);
        }
        if (s.n("sse_approximation") > 0) {
            tier = "C";
            reasons.add(where + ": SSE reciprocal approximation (" + s.n("sse_approximation") +
                " instructions)");
        }
        if (s.n("x87_transcendental") > 0) {
            tier = "D";
            reasons.add(where + ": x87 transcendental " + s.transcendentalSeen);
        }
        if (!mathTrans.isEmpty()) {
            tier = "D";
            reasons.add(where + ": calls CRT transcendental math " + mathTrans);
        }
        if (tier.equals("A")) {
            reasons.add(where + ": integer and/or SSE arithmetic only");
        }
        return tier;
    }

    private List<Object> functionList(Set<Function> set) {
        TreeMap<String, Map<String, Object>> out = new TreeMap<>();
        for (Function x : set) {
            Map<String, Object> m = NvCommon.obj();
            String key;
            if (x.isExternal()) {
                key = "ext:" + x.getName(true);
                m.put("entry", key);
            }
            else {
                key = NvCommon.addr(x.getEntryPoint());
                m.put("entry", key);
            }
            m.put("name", x.getName());
            out.put(key, m);
        }
        return new ArrayList<>(out.values());
    }

    private String namedLabel(Address a) {
        Symbol s = symbols.getPrimarySymbol(a);
        return s != null && s.getSource() != SourceType.DEFAULT ? s.getName(true) : "";
    }

    // ------------------------------------------------------------------
    // Access sizes and constant decoding
    // ------------------------------------------------------------------

    /**
     * Size in bytes of the memory access the instruction makes at exactly
     * {@code to}, from the instruction's p-code (the memory varnode at that
     * address). Returns 0 when the p-code has no access at that address,
     * which happens when the address is only a table base in an indexed
     * operand such as [EAX*4 + address].
     */
    private int pcodeAccessSize(Instruction ins, Address to) {
        int best = 0;
        for (PcodeOp op : ins.getPcode()) {
            Varnode out = op.getOutput();
            if (out != null && out.isAddress() && out.getOffset() == to.getOffset()) {
                best = Math.max(best, out.getSize());
            }
            for (Varnode in : op.getInputs()) {
                if (in.isAddress() && in.getOffset() == to.getOffset()) {
                    best = Math.max(best, in.getSize());
                }
            }
        }
        return best;
    }

    /** Access size from the operand's text ("dword ptr", "float ptr", ...); 0 when unknown. */
    private int textAccessSize(Instruction ins, Address to) {
        String hex = Long.toHexString(to.getOffset());
        for (int i = 0; i < ins.getNumOperands(); i++) {
            String r = ins.getDefaultOperandRepresentation(i).toLowerCase();
            if (!r.contains(hex)) {
                continue;
            }
            if (r.contains("extended double ptr") || r.contains("tword ptr")) {
                return 10;
            }
            if (r.contains("xmmword ptr") || r.contains("oword ptr")) {
                return 16;
            }
            if (r.contains("qword ptr") || r.contains("double ptr")) {
                return 8;
            }
            if (r.contains("dword ptr") || r.contains("float ptr")) {
                return 4;
            }
            if (r.contains("word ptr")) {
                return 2;
            }
            if (r.contains("byte ptr")) {
                return 1;
            }
        }
        return 0;
    }

    /**
     * What the instruction treats the memory operand as: f32, f64, f80, int,
     * int_packed, x87_cw, mxcsr or raw. A conversion reads its source type,
     * which is the first part of the name (CVTSD2SS reads a double), not the
     * destination type that the name ends with.
     */
    private static String primaryKind(String mn, int size) {
        if (mn.equals("FLDCW")) {
            return "x87_cw";
        }
        if (mn.equals("LDMXCSR") || mn.equals("VLDMXCSR")) {
            return "mxcsr";
        }
        if (mn.startsWith("FI") && !mn.equals("FINIT") && !mn.equals("FINCSTP")) {
            return "int";
        }
        Matcher cvt = SSE_CVT.matcher(mn);
        if (cvt.matches()) {
            switch (cvt.group(1)) {
                case "SS":
                case "PS":
                    return "f32";
                case "SD":
                case "PD":
                    return "f64";
                case "SI":
                    return "int";
                default:
                    return "int_packed"; // DQ and PI: packed 32-bit integers
            }
        }
        if (mn.startsWith("F")) {
            return size == 4 ? "f32" : size == 8 ? "f64" : size == 10 ? "f80" : "raw";
        }
        if (mn.endsWith("SS") || mn.endsWith("PS")) {
            return "f32";
        }
        if (mn.endsWith("SD") || mn.endsWith("PD")) {
            return "f64";
        }
        return size == 4 ? "f32" : size == 8 ? "f64" : "raw";
    }

    private Map<String, Object> floatConstant(Address at, int size, String kind) {
        MemoryBlock b = prog.getMemory().getBlock(at);
        if (b == null || !b.isInitialized()) {
            return null;
        }
        byte[] bytes = NvCommon.readBytes(prog, at, size);
        if (bytes.length < size) {
            return null;
        }
        Map<String, Object> e = NvCommon.obj();
        e.put("address", NvCommon.addr(at));
        e.put("label", namedLabel(at));
        e.put("block", b.getName());
        e.put("writable_block", b.isWrite());
        e.put("access_size", size);
        e.put("read_as", kind);
        e.put("bytes", NvCommon.bytesToHex(bytes));
        e.put("hex", "0x" + hexValue(bytes));
        switch (kind) {
            case "f32":
                if (size == 4) {
                    e.put("f32", Float.intBitsToFloat((int) NvCommon.le(bytes, 0, 4)));
                }
                else if (size == 8) {
                    e.put("f32_pair", Arrays.asList(Float.intBitsToFloat((int) NvCommon.le(bytes, 0, 4)),
                        Float.intBitsToFloat((int) NvCommon.le(bytes, 4, 4))));
                }
                else if (size == 16) {
                    e.put("f32x4", f32s(bytes, 4));
                }
                break;
            case "f64":
                if (size == 8) {
                    e.put("f64", Double.longBitsToDouble(NvCommon.le(bytes, 0, 8)));
                }
                else if (size == 16) {
                    e.put("f64x2", Arrays.asList(Double.longBitsToDouble(NvCommon.le(bytes, 0, 8)),
                        Double.longBitsToDouble(NvCommon.le(bytes, 8, 8))));
                }
                break;
            case "f80":
                e.put("f80", NvCommon.f80ToString(bytes));
                e.put("f80_as_f64", NvCommon.f80ToDouble(bytes));
                break;
            case "int":
                e.put("signed", signed(bytes));
                e.put("unsigned", Long.toUnsignedString(NvCommon.le(bytes, 0, Math.min(size, 8))));
                break;
            case "int_packed": {
                List<Object> lanes = new ArrayList<>();
                for (int i = 0; i + 4 <= size; i += 4) {
                    lanes.add((int) NvCommon.le(bytes, i, 4));
                }
                e.put("i32x" + lanes.size(), lanes);
                break;
            }
            case "x87_cw": {
                int cw = (int) NvCommon.le(bytes, 0, 2);
                int pc = (cw >> 8) & 3;
                e.put("precision_bits", pc == 0 ? 24 : pc == 2 ? 53 : pc == 3 ? 64 : 0);
                e.put("rounding", new String[] { "nearest", "down", "up", "truncate" }[(cw >> 10) & 3]);
                e.put("exception_masks", String.format("0x%02x", cw & 0x3f));
                break;
            }
            case "mxcsr": {
                int m = (int) NvCommon.le(bytes, 0, 4);
                e.put("rounding", new String[] { "nearest", "down", "up", "truncate" }[(m >> 13) & 3]);
                e.put("flush_to_zero", (m & 0x8000) != 0);
                e.put("denormals_are_zero", (m & 0x40) != 0);
                e.put("exception_masks", String.format("0x%02x", (m >> 7) & 0x3f));
                break;
            }
            default:
                if (size == 4) {
                    e.put("f32", Float.intBitsToFloat((int) NvCommon.le(bytes, 0, 4)));
                }
                if (size == 8) {
                    e.put("f64", Double.longBitsToDouble(NvCommon.le(bytes, 0, 8)));
                }
                if (size == 16) {
                    e.put("f32x4", f32s(bytes, 4));
                }
        }
        return e;
    }

    private static List<Object> f32s(byte[] b, int n) {
        List<Object> l = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            l.add(Float.intBitsToFloat((int) NvCommon.le(b, 4 * i, 4)));
        }
        return l;
    }

    private static long signed(byte[] b) {
        int n = Math.min(b.length, 8);
        long v = NvCommon.le(b, 0, n);
        if (n < 8) {
            long sign = 1L << (n * 8 - 1);
            if ((v & sign) != 0) {
                v -= 1L << (n * 8);
            }
        }
        return v;
    }

    private static String hexValue(byte[] bytes) {
        StringBuilder sb = new StringBuilder();
        for (int i = bytes.length - 1; i >= 0; i--) {
            sb.append(String.format("%02x", bytes[i] & 0xff));
        }
        return sb.toString();
    }

    /** u32/i32/f32 for four or more bytes, u64/i64/f64 for eight or more, and so on. */
    private Map<String, Object> interpretations(byte[] b) {
        Map<String, Object> m = NvCommon.obj();
        if (b.length >= 1 && b.length < 2) {
            m.put("u8", b[0] & 0xff);
            m.put("i8", (int) b[0]);
        }
        if (b.length >= 2 && b.length < 4) {
            long v = NvCommon.le(b, 0, 2);
            m.put("u16", v);
            m.put("i16", (int) (short) v);
        }
        if (b.length >= 4) {
            long v = NvCommon.le(b, 0, 4);
            m.put("u32", v);
            m.put("i32", (int) v);
            m.put("f32", Float.intBitsToFloat((int) v));
        }
        if (b.length >= 8) {
            long v = NvCommon.le(b, 0, 8);
            m.put("u64", Long.toUnsignedString(v));
            m.put("i64", v);
            m.put("f64", Double.longBitsToDouble(v));
        }
        if (b.length >= 10) {
            m.put("f80", NvCommon.f80ToString(Arrays.copyOf(b, 10)));
        }
        return m;
    }

    /**
     * 32-bit immediates that would be sensible single-precision floats
     * (finite, magnitude between 1e-4 and 1e8). Many integers look like
     * floats by chance, so these are only flagged "possible float".
     */
    private void possibleFloatImmediate(Instruction ins, String mn, List<Object> out) {
        FlowType ft = ins.getFlowType();
        if (ft.isCall() || ft.isJump() || ft.isTerminal()) {
            return; // an immediate here is a branch target, not data
        }
        for (int i = 0; i < ins.getNumOperands(); i++) {
            Scalar sc = ins.getScalar(i);
            if (sc == null || sc.bitLength() != 32) {
                continue;
            }
            long u = sc.getUnsignedValue();
            boolean isAddress = false;
            for (Reference r : ins.getOperandReferences(i)) {
                if (r.isMemoryReference()) {
                    isAddress = true;
                }
            }
            if (isAddress || prog.getMemory().contains(toAddr(u))) {
                continue;
            }
            float f = Float.intBitsToFloat((int) u);
            float a = Math.abs(f);
            if (Float.isNaN(f) || Float.isInfinite(f) || a < 1e-4f || a > 1e8f) {
                continue;
            }
            Map<String, Object> e = NvCommon.obj();
            e.put("instruction", NvCommon.addr(ins.getAddress()));
            e.put("mnemonic", mn);
            e.put("hex", String.format("0x%08x", u));
            e.put("f32", f);
            e.put("flag", "possible float");
            out.add(e);
        }
    }

    // ------------------------------------------------------------------
    // Markdown
    // ------------------------------------------------------------------

    @SuppressWarnings("unchecked")
    private String markdown(Map<String, Object> c) {
        StringBuilder sb = new StringBuilder();
        sb.append("# Function card ").append(c.get("entry")).append(" ").append(c.get("name")).append("\n\n");
        sb.append("Private research output for a program you own. Do not commit.\n\n");
        sb.append("- Program: ").append(c.get("program_name")).append(", exe SHA-256 `")
            .append(c.get("exe_sha256")).append("`, Ghidra ").append(c.get("ghidra_version")).append("\n");
        sb.append("- Size: ").append(c.get("size")).append(" bytes in ")
            .append(((List<Object>) c.get("body")).size()).append(" range(s)\n");
        sb.append("- Prototype: `").append(c.get("prototype")).append("`\n");
        sb.append("- Calling convention: ").append(c.get("calling_convention")).append(", stack purge ")
            .append(c.get("stack_purge")).append(", thunk: ").append(c.get("is_thunk")).append("\n");
        sb.append("- Name source: ").append(c.get("name_source")).append(", tags: ").append(c.get("tags")).append("\n");
        sb.append("- **CPU-fidelity tier: ").append(c.get("tier")).append("** (with callees, ")
            .append(c.get("tier_callee_functions_scanned")).append(" function(s) scanned within ")
            .append(c.get("tier_callee_depth_limit")).append(" levels: ").append(c.get("tier_with_callees"))
            .append(c.get("tier_is_lower_bound").equals(true) ? ", a lower bound" : "").append(")\n");
        for (Object r : (List<Object>) c.get("tier_reasons")) {
            sb.append("  - ").append(r).append("\n");
        }
        for (Object r : (List<Object>) c.get("tier_with_callees_reasons")) {
            sb.append("  - ").append(r).append("\n");
        }
        for (Object r : (List<Object>) c.get("tier_lower_bound_reasons")) {
            sb.append("  - not scanned: ").append(r).append("\n");
        }
        sb.append("- Stateful: ").append(c.get("stateful"));
        List<Object> sr = (List<Object>) c.get("stateful_reasons");
        if (!sr.isEmpty()) {
            sb.append(" (").append(String.join("; ", sr.stream().map(Object::toString).toList())).append(")");
        }
        sb.append(". Writes through pointers are not detected.\n");
        sb.append("- Changes FPU/SSE control state: ").append(c.get("changes_fpu_control")).append("\n\n");

        sb.append("## Callers\n\n");
        listFunctions(sb, (List<Object>) c.get("callers"));
        sb.append("\n## Callees\n\n");
        listFunctions(sb, (List<Object>) c.get("callees"));

        sb.append("\n## Float constants read by x87 and SSE instructions\n\n");
        List<Object> fc = (List<Object>) c.get("float_constants");
        if (fc.isEmpty()) {
            sb.append("None.\n");
        }
        else {
            sb.append("| address | label | bytes | read as | value | writable block | used by |\n");
            sb.append("| --- | --- | --- | --- | --- | --- | --- |\n");
            for (Object o : fc) {
                Map<String, Object> e = (Map<String, Object>) o;
                sb.append("| ").append(e.get("address")).append(" | ").append(e.get("label"))
                    .append(" | ").append(e.get("bytes")).append(" | ").append(e.get("read_as"))
                    .append(" (").append(e.get("access_size")).append(" bytes) | ")
                    .append(describeValue(e)).append(" | ").append(e.get("writable_block"))
                    .append(" | ").append(String.join(", ", ((List<Object>) e.get("instructions"))
                        .stream().map(Object::toString).toList()))
                    .append(" |\n");
            }
            sb.append("\nA constant in a writable block is its value in the image; the program may change it.\n");
            sb.append("exact_address=false means the address is only a table base in an indexed operand: only the first element is decoded.\n");
        }

        sb.append("\n## Possible float immediates\n\n");
        List<Object> pi = (List<Object>) c.get("possible_float_immediates");
        if (pi.isEmpty()) {
            sb.append("None.\n");
        }
        else {
            sb.append("32-bit immediates that would be sensible f32 values. Many integers look like this by chance.\n\n");
            sb.append("| instruction | mnemonic | hex | f32 |\n| --- | --- | --- | --- |\n");
            for (Object o : pi) {
                Map<String, Object> e = (Map<String, Object>) o;
                sb.append("| ").append(e.get("instruction")).append(" | ").append(e.get("mnemonic"))
                    .append(" | ").append(e.get("hex")).append(" | ").append(e.get("f32")).append(" (possible float) |\n");
            }
        }

        sb.append("\n## Globals\n\n");
        List<Object> gl = (List<Object>) c.get("globals");
        if (gl.isEmpty()) {
            sb.append("None.\n");
        }
        else {
            sb.append("| address | label | block | access | size | current bytes | interpretations |\n");
            sb.append("| --- | --- | --- | --- | --- | --- | --- |\n");
            for (Object o : gl) {
                Map<String, Object> g = (Map<String, Object>) o;
                sb.append("| ").append(g.get("address")).append(" | ").append(g.get("label"))
                    .append(" | ").append(g.get("block")).append(g.get("writable_block").equals(true) ? " (rw)" : "")
                    .append(" | ").append(g.get("access")).append(" | ").append(g.get("size"))
                    .append(" | ").append(g.get("current_bytes") == null ? "(uninitialized or unknown)" : g.get("current_bytes"))
                    .append(" | ").append(g.get("interpretations")).append(" |\n");
            }
        }
        List<Object> cp = (List<Object>) c.get("code_pointers");
        if (!cp.isEmpty()) {
            sb.append("\nCode addresses used as data: ");
            sb.append(String.join(", ", cp.stream().map(x -> ((Map<String, Object>) x).get("address").toString()).toList()));
            sb.append("\n");
        }

        sb.append("\n## Instruction classes\n\n");
        Map<String, Object> counts = (Map<String, Object>) c.get("instruction_counts");
        sb.append("| class | count |\n| --- | --- |\n");
        for (Map.Entry<String, Object> e : counts.entrySet()) {
            sb.append("| ").append(e.getKey()).append(" | ").append(e.getValue()).append(" |\n");
        }
        sb.append("\nFloating-point mnemonics: ").append(c.get("fp_mnemonics")).append("\n");
        if (!((List<Object>) c.get("crt_math_callees_transcendental")).isEmpty() ||
            !((List<Object>) c.get("crt_math_callees_other")).isEmpty()) {
            sb.append("\nCRT math callees: ").append(c.get("crt_math_callees_transcendental")).append(" ")
                .append(c.get("crt_math_callees_other")).append("\n");
        }
        return sb.toString();
    }

    @SuppressWarnings("unchecked")
    private static void listFunctions(StringBuilder sb, List<Object> list) {
        if (list.isEmpty()) {
            sb.append("None.\n");
            return;
        }
        for (Object o : list) {
            Map<String, Object> m = (Map<String, Object>) o;
            sb.append("- ").append(m.get("entry")).append(" ").append(m.get("name")).append("\n");
        }
    }

    private static String describeValue(Map<String, Object> e) {
        for (String k : new String[] { "f32", "f64", "f80", "f32x4", "f64x2", "f32_pair", "signed", "i32x2",
            "i32x4" }) {
            if (e.containsKey(k)) {
                return k + " = " + e.get(k);
            }
        }
        if (e.containsKey("precision_bits")) {
            return "x87 control word: " + e.get("precision_bits") + "-bit precision, round " + e.get("rounding");
        }
        if (e.containsKey("flush_to_zero")) {
            return "MXCSR: round " + e.get("rounding") + ", FTZ " + e.get("flush_to_zero") + ", DAZ " +
                e.get("denormals_are_zero");
        }
        return "";
    }
}

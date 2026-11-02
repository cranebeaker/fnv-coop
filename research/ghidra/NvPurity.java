// Purity facts: what each function touches besides its own stack, for the
// automatic oracle check (docs/HANDOFF.md next task 10, #182).
//
// Args: out=<file.tsv> [threads=N] [args=1|0]
//
// Read-only (run with -readOnly). Writes one row per function, sorted by
// entry:
//
//   entry, size, ret_pop, ebp_frame, calls, tail_calls, indirect, imports,
//   w_stack, w_global, w_pointer, w_implicit, r_pointer, fs
//
//   ret_pop     bytes the function pops on return (the largest `RET n`; 0 for
//               plain RET; -1 when the body has no RET, e.g. it only tail
//               calls or never returns). Callee cleanup (stdcall/thiscall
//               with stack arguments) when > 0.
//   ebp_frame   1 when the entry is `PUSH EBP; MOV EBP,ESP`, so EBP-based
//               operands are the frame; otherwise EBP is a general register
//               and EBP-based stores count as pointer stores.
//   calls       entries of directly called functions, ','-separated, call
//               order, repeats kept.
//   tail_calls  entries reached by an unconditional JMP to another
//               function's entry.
//   indirect    CALLs and JMPs whose target is not a known function and not
//               inside the body (vtable and function-pointer calls; a switch
//               JMP whose targets are all in the body does not count).
//   imports     calls or jumps through an import slot.
//   w_stack     stores to the frame (ESP-based, or EBP-based with ebp_frame);
//               PUSH and CALL count here too.
//   w_global    stores to an absolute address (a global, with or without an
//               index register).
//   w_pointer   stores through any other base register.
//   w_implicit  stores with no explicit memory destination that are not a
//               PUSH/CALL/ENTER (string instructions such as REP STOSD).
//   r_pointer   loads through a base register that is not the frame and
//               without an absolute displacement (reading an object that a
//               pointer argument or a global points at).
//   fs          number of operands that use the FS segment (TLS block, SEH
//               frame); any FS store is also counted in w_pointer.
//   reg_in      registers among ECX and EDX that the entry block reads before
//               writing ("ECX", "ECX,EDX" or empty): ECX alone means
//               __thiscall (or __fastcall with one register argument), both
//               __fastcall; empty means every argument is on the stack.
//   stack_args  bytes of stack arguments the body reads, measured from
//               [EBP+8+n] operands (a lower bound: an argument the body never
//               reads is not counted); -1 without an EBP frame.
//   consts_int  immediate operands that are not addresses or stack
//               adjustments (CMP EAX,0x14 -> 20), signed decimal, ','.
//   hidden_in   instructions whose result no input determines (RDTSC, RDTSCP,
//               RDPMC, RDRAND, RDSEED, CPUID): the clock and the like.
//   consts_float the values x87/SSE instructions load from absolute
//               addresses: f<8 hex> (dword) or d<16 hex> (qword), ','.
//               Both are test inputs for the oracle check (boundaries).
//   arg_access  from the decompiler's data flow, per input used as a
//               pointer, what goes through it: input=items|input=items, inputs
//               ecx, edx, s<n> (stack argument n bytes past the return
//               address), eax, ebx, esi, edi, ebp (register arguments of
//               whole-program optimized code); items u (eax..ebp: the value is
//               used at all; ecx, edx: used other than as a call argument),
//               r<off>.<size> / w<off>.<size> (load / store at
//               a byte offset), r? / w? (offset unknown), c<off> (the pointer
//               loaded from <off> is dereferenced), p<callee> (passed to a
//               call, p? for an indirect one), e (the pointer itself is
//               stored). Only for functions with indirect = imports = w_global
//               = w_implicit = fs = 0 (args=0 skips it); "-" otherwise or when
//               the decompiler gave no result.
//   untracked_loads, untracked_stores  LOADs / STOREs whose address no input
//               explains (pointers loaded from memory or returned by calls);
//               -1 where arg_access is "-".
//
// A function is pure for the oracle check when it and every function it
// calls (transitively) has indirect = imports = w_global = w_pointer =
// w_implicit = fs = 0; whether r_pointer is acceptable depends on its
// arguments. The join with the translations is done outside Ghidra
// (research/oracle-pure).
//
// The output holds addresses and counts only (no code, no strings), but it
// describes a program you own: keep it in the private research tree.
//
// Gate: rows written == getFunctionCount() minus external functions,
// otherwise the script throws and writes nothing.
//
// @category NV

import java.io.BufferedWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.lang.Register;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.pcode.PcodeOp;
import ghidra.program.model.scalar.Scalar;
import ghidra.program.model.symbol.FlowType;
import ghidra.program.model.symbol.RefType;
import ghidra.program.model.symbol.Reference;

public class NvPurity extends GhidraScript {

    private static final String NAME = "NvPurity";

    /** Lowest address of the image; an operand constant at or above it is an absolute address. */
    private long imageBase;

    @Override
    public void run() throws Exception {
        Map<String, String> args =
            NvCommon.parseArgs(NAME, getScriptArgs(), Set.of("out", "threads", "args"));
        Path out = Paths.get(NvCommon.require(NAME, args, "out"));
        if (out.getParent() != null) {
            Files.createDirectories(out.getParent());
        }
        imageBase = currentProgram.getImageBase().getOffset();

        FunctionManager fm = currentProgram.getFunctionManager();
        Listing listing = currentProgram.getListing();
        int externals = 0;
        for (Function f : fm.getExternalFunctions()) {
            externals++;
        }
        int expected = fm.getFunctionCount() - externals;

        int threads = NvCommon.intArg(NAME, args, "threads",
            Math.min(8, Runtime.getRuntime().availableProcessors()), 1, 64);
        boolean doArgs = NvCommon.flag(NAME, args, "args", true);

        List<String> lines = new ArrayList<>();
        lines.add("entry\tsize\tret_pop\tebp_frame\tcalls\ttail_calls\tindirect\timports\t"
            + "w_stack\tw_global\tw_pointer\tw_implicit\tr_pointer\tfs\treg_in\tstack_args\t"
            + "consts_int\tconsts_float\thidden_in\targ_access\tuntracked_loads\tuntracked_stores");
        List<Function> order = new ArrayList<>();
        List<String> base = new ArrayList<>();
        for (Function f : fm.getFunctions(true)) {
            monitor.checkCancelled();
            if (f.isExternal()) {
                continue;
            }
            order.add(f);
            base.add(row(f, fm, listing));
        }
        // Second pass: the decompiler's data flow for the functions that could be checked
        // (no indirect or imported calls, no stores to globals or implicit ones, no FS).
        String[] argCols = new String[order.size()];
        java.util.Arrays.fill(argCols, "-\t-1\t-1");
        if (doArgs) {
            List<Integer> todo = new ArrayList<>();
            for (int i = 0; i < base.size(); i++) {
                String[] c = base.get(i).split("\t", -1);
                if (c[6].equals("0") && c[7].equals("0") && c[9].equals("0") && c[11].equals("0")
                    && c[13].equals("0")) {
                    todo.add(i);
                }
            }
            println(NAME + ": data flow for " + todo.size() + " functions, threads=" + threads);
            argAccessAll(order, todo, argCols, threads);
        }
        for (int i = 0; i < base.size(); i++) {
            lines.add(base.get(i) + "\t" + argCols[i]);
        }
        int rows = lines.size() - 1;
        if (rows != expected) {
            throw new IllegalStateException(NAME + ": wrote " + rows + " rows, the program has "
                + expected + " non-external functions");
        }
        try (BufferedWriter w = Files.newBufferedWriter(out, StandardCharsets.UTF_8)) {
            for (String l : lines) {
                w.write(l);
                w.write('\n');
            }
        }
        println(NAME + ": " + rows + " functions written to " + out);
    }

    private String row(Function f, FunctionManager fm, Listing listing) {
        AddressSetView body = f.getBody();
        boolean ebpFrame = hasEbpFrame(listing, f.getEntryPoint());
        long retPop = -1;
        List<String> calls = new ArrayList<>();
        List<String> tails = new ArrayList<>();
        int indirect = 0, imports = 0;
        int wStack = 0, wGlobal = 0, wPointer = 0, wImplicit = 0, rPointer = 0, fs = 0, hidden = 0;
        java.util.LinkedHashSet<Long> intConsts = new java.util.LinkedHashSet<>();
        java.util.LinkedHashSet<String> floatConsts = new java.util.LinkedHashSet<>();

        for (Instruction ins : listing.getInstructions(body, true)) {
            String mnem = ins.getMnemonicString().toUpperCase();
            FlowType ft = ins.getFlowType();
            collectConstants(ins, mnem, intConsts, floatConsts);
            if (HIDDEN_INPUTS.contains(mnem)) {
                hidden++;
            }

            if (mnem.startsWith("RET")) {
                long pop = 0;
                if (ins.getNumOperands() > 0) {
                    Scalar s = ins.getScalar(0);
                    if (s != null) {
                        pop = s.getUnsignedValue();
                    }
                }
                retPop = Math.max(retPop, pop);
            }

            boolean call = ft.isCall();
            boolean jump = ft.isJump() && !ft.isConditional();
            if (call || jump) {
                boolean viaImport = false;
                for (Reference r : ins.getReferencesFrom()) {
                    if (r.isExternalReference()) {
                        viaImport = true;
                    }
                    else if (r.getReferenceType().isData() && r.getToAddress().isMemoryAddress()) {
                        for (Reference r2 : getReferencesFrom(r.getToAddress())) {
                            if (r2.isExternalReference()) {
                                viaImport = true;
                            }
                        }
                    }
                }
                Address[] flows = ins.getFlows();
                boolean resolved = false;
                for (Address t : flows) {
                    if (jump && body.contains(t)) {
                        resolved = true; // switch or loop inside the body
                        continue;
                    }
                    Function g = fm.getFunctionAt(t);
                    if (g == null) {
                        continue;
                    }
                    resolved = true;
                    if (g.isExternal()) {
                        viaImport = true;
                    }
                    else if (call) {
                        calls.add(NvCommon.addr(g.getEntryPoint()));
                    }
                    else {
                        tails.add(NvCommon.addr(g.getEntryPoint()));
                    }
                }
                if (viaImport) {
                    imports++;
                }
                else if (!resolved && (call || ft.isComputed())) {
                    indirect++;
                }
            }

            // Operand-level loads and stores.
            boolean explicitStore = false;
            for (int i = 0; i < ins.getNumOperands(); i++) {
                // A memory operand is written in brackets ([ESP+0x4], dword ptr [0x011dea3c],
                // FS:[0x0]); register and immediate operands are not.
                if (!isBracketed(ins, i)) {
                    continue;
                }
                if (ins.getDefaultOperandRepresentation(i).contains("FS:")) {
                    fs++;
                }
                // Ghidra has no reference type for some operands; then the p-code decides
                // (a STORE in the instruction means its memory operand is written).
                RefType rt = ins.getOperandRefType(i);
                boolean write = rt != null ? rt.isWrite() : hasStore(ins);
                boolean read = rt != null ? rt.isRead() : !write;
                Base base = base(ins, i, ebpFrame);
                if (write) {
                    explicitStore = true;
                    if (base == Base.STACK) {
                        wStack++;
                    }
                    else if (base == Base.ABSOLUTE) {
                        wGlobal++;
                    }
                    else {
                        wPointer++;
                    }
                }
                else if (read && base == Base.POINTER && !mnem.equals("LEA")) {
                    rPointer++;
                }
            }
            if (!explicitStore && hasStore(ins)) {
                if (mnem.startsWith("PUSH") || mnem.equals("CALL") || mnem.equals("ENTER")) {
                    wStack++;
                }
                else {
                    wImplicit++;
                }
            }
        }

        return NvCommon.addr(f.getEntryPoint()) + "\t" + body.getNumAddresses() + "\t" + retPop + "\t"
            + (ebpFrame ? 1 : 0) + "\t" + String.join(",", calls) + "\t" + String.join(",", tails) + "\t"
            + indirect + "\t" + imports + "\t" + wStack + "\t" + wGlobal + "\t" + wPointer + "\t"
            + wImplicit + "\t" + rPointer + "\t" + fs + "\t" + registersIn(listing, f.getEntryPoint(), body)
            + "\t" + stackArgBytes(listing, body, ebpFrame)
            + "\t" + joinLongs(intConsts) + "\t" + String.join(",", floatConsts) + "\t" + hidden;
    }

    /** Instructions whose result no input determines: the clock, counters, random numbers, CPUID. */
    private static final java.util.Set<String> HIDDEN_INPUTS =
        java.util.Set.of("RDTSC", "RDTSCP", "RDPMC", "RDRAND", "RDSEED", "CPUID");

    // ---- per-argument accesses from the decompiler's data flow ----

    private final List<ghidra.app.decompiler.DecompInterface> decompilers =
        java.util.Collections.synchronizedList(new ArrayList<>());
    private final ThreadLocal<ghidra.app.decompiler.DecompInterface> localDecompiler =
        new ThreadLocal<>();
    private ghidra.app.decompiler.DecompileOptions decompOptions;

    /**
     * The data-flow columns for the functions in {@code todo}. The decompiler only keeps
     * computations that reach the return value, and it never infers a return in x87 ST0: a
     * function returning a float there (with the usual DEFAULT prototype) is decompiled as
     * returning EAX, and every load that only feeds the float is dropped as dead (00e37820 reads
     * {@code this->p->v} for its result and showed only the read of {@code p}). So functions with
     * x87 code are decompiled a second time with the return stored in ST0, inside a transaction
     * that is always aborted, so the program is left as it was. Accesses are the union of the two
     * passes, untracked counts the larger, and a function either pass failed on gets no result.
     */
    private void argAccessAll(List<Function> order, List<Integer> todo, String[] out, int threads)
            throws Exception {
        decompOptions = new ghidra.app.decompiler.DecompileOptions();
        decompOptions.grabFromProgram(currentProgram);
        String[] stored = new String[out.length];
        argAccessPass(order, todo, stored, threads, "stored prototype");
        String[] x87 = new String[out.length];
        List<Integer> withX87 = new ArrayList<>();
        for (int i : todo) {
            if (usesX87(order.get(i))) {
                withX87.add(i);
            }
        }
        int tx = currentProgram.startTransaction(NAME + ": return overrides (always aborted)");
        try {
            returnInSt0(order, withX87);
            argAccessPass(order, withX87, x87, threads, "return in ST0");
        }
        finally {
            currentProgram.endTransaction(tx, false);
        }
        for (int i : todo) {
            out[i] = mergeAccess(stored[i], x87[i]);
        }
    }

    private void argAccessPass(List<Function> order, List<Integer> todo, String[] out, int threads,
            String what) throws Exception {
        println(NAME + ": data flow (" + what + ") for " + todo.size() + " functions");
        java.util.concurrent.ExecutorService pool =
            java.util.concurrent.Executors.newFixedThreadPool(threads);
        try {
            List<java.util.concurrent.Future<String>> futures = new ArrayList<>();
            for (int i : todo) {
                Function f = order.get(i);
                futures.add(pool.submit(() -> argAccess(f)));
            }
            int next = Math.max(1, todo.size() / 10);
            for (int k = 0; k < todo.size(); k++) {
                monitor.checkCancelled();
                out[todo.get(k)] = futures.get(k).get();
                if (k + 1 >= next) {
                    println(NAME + ": data flow (" + what + ") " + (k + 1) + "/" + todo.size());
                    next += Math.max(1, todo.size() / 10);
                }
            }
        }
        finally {
            pool.shutdownNow();
            pool.awaitTermination(1, java.util.concurrent.TimeUnit.MINUTES);
            // Fresh decompilers for the next pass, so none keeps a prototype from this one.
            synchronized (decompilers) {
                for (ghidra.app.decompiler.DecompInterface d : decompilers) {
                    d.dispose();
                }
                decompilers.clear();
            }
        }
    }

    /**
     * Gives the functions a {@code float10} return stored in ST0 (explicit storage: with the
     * "unknown" convention most functions have, a float10 return would otherwise go through a
     * hidden pointer).
     */
    private void returnInSt0(List<Function> order, List<Integer> which) throws Exception {
        ghidra.program.model.listing.VariableStorage st0 = new ghidra.program.model.listing.VariableStorage(
            currentProgram, currentProgram.getLanguage().getRegister("ST0"));
        for (int i : which) {
            Function f = order.get(i);
            try {
                f.setCustomVariableStorage(true);
                f.setReturn(ghidra.program.model.data.Float10DataType.dataType, st0,
                    ghidra.program.model.symbol.SourceType.USER_DEFINED);
            }
            catch (Exception ex) {
                // left as stored; that pass then repeats the stored-prototype result
            }
        }
    }

    /** Whether the body has any x87 instruction (only then can it return in ST0). */
    private boolean usesX87(Function f) {
        for (Instruction ins : currentProgram.getListing().getInstructions(f.getBody(), true)) {
            if (ins.getMnemonicString().toUpperCase().startsWith("F")) {
                return true;
            }
        }
        return false;
    }

    /** Union of the passes' arg_access items, the largest untracked counts; "-" if any failed. */
    private static String mergeAccess(String... cols) {
        java.util.TreeMap<String, java.util.TreeSet<String>> acc = new java.util.TreeMap<>();
        long loads = 0, stores = 0;
        for (String col : cols) {
            if (col == null) {
                continue; // not in that pass (no x87 code)
            }
            String[] c = col.split("\t", -1);
            if (c[0].equals("-")) {
                return "-\t-1\t-1";
            }
            for (String part : c[0].split("\\|")) {
                if (part.isEmpty()) {
                    continue;
                }
                int eq = part.indexOf('=');
                for (String item : part.substring(eq + 1).split(",")) {
                    add(acc, part.substring(0, eq), item);
                }
            }
            loads = Math.max(loads, Long.parseLong(c[1]));
            stores = Math.max(stores, Long.parseLong(c[2]));
        }
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, java.util.TreeSet<String>> e : acc.entrySet()) {
            if (sb.length() > 0) {
                sb.append('|');
            }
            sb.append(e.getKey()).append('=').append(String.join(",", e.getValue()));
        }
        return sb + "\t" + loads + "\t" + stores;
    }

    private ghidra.app.decompiler.DecompInterface decompilerForThisThread() {
        ghidra.app.decompiler.DecompInterface di = localDecompiler.get();
        if (di == null) {
            di = new ghidra.app.decompiler.DecompInterface();
            di.setOptions(decompOptions);
            di.toggleCCode(false);
            di.toggleSyntaxTree(true);
            di.setSimplificationStyle("decompile");
            if (!di.openProgram(currentProgram)) {
                String msg = di.getLastMessage();
                di.dispose();
                throw new IllegalStateException(NAME + ": cannot open decompiler: " + msg);
            }
            decompilers.add(di);
            localDecompiler.set(di);
        }
        return di;
    }

    /**
     * The three data-flow columns for one function: arg_access, untracked_loads, untracked_stores.
     *
     * arg_access lists, per input that is used as a pointer, what is done through it, as
     * {@code <input>=<item>,<item>...} joined by {@code |}. Inputs: {@code ecx}, {@code edx},
     * {@code s<n>} (the stack argument n bytes past the return address), and {@code eax},
     * {@code ebx}, {@code esi}, {@code edi}, {@code ebp} (registers no standard convention passes
     * arguments in, with the item {@code u} when the value is used at all; ECX and EDX get
     * {@code u} when used other than as a call argument). Items:
     * {@code r<off>.<size>} / {@code w<off>.<size>} a load / store at a byte offset from the
     * argument; {@code r?} / {@code w?} at an offset the data flow can't pin down; {@code c<off>}
     * the pointer loaded from {@code <off>} is itself dereferenced; {@code p<callee>} the pointer
     * (or one derived from it) is passed to a call; {@code e} it is stored somewhere. An input that
     * is never used as an address is not listed. untracked_loads / untracked_stores count LOAD /
     * STORE operations whose address comes from none of the inputs (pointers loaded from memory,
     * returned by calls). "-" and -1 when the decompiler gave no result.
     */
    private String argAccess(Function f) {
        ghidra.app.decompiler.DecompileResults res;
        try {
            res = decompilerForThisThread().decompileFunction(f, 60, ghidra.util.task.TaskMonitor.DUMMY);
        }
        catch (Exception ex) {
            return "-\t-1\t-1";
        }
        ghidra.program.model.pcode.HighFunction hf = res == null ? null : res.getHighFunction();
        if (hf == null) {
            return "-\t-1\t-1";
        }
        // varnode -> (input name, offset from it; null when unknown)
        Map<ghidra.program.model.pcode.Varnode, String> owner = new java.util.HashMap<>();
        Map<ghidra.program.model.pcode.Varnode, Long> offset = new java.util.HashMap<>();
        java.util.ArrayDeque<ghidra.program.model.pcode.Varnode> work = new java.util.ArrayDeque<>();
        java.util.TreeMap<String, java.util.TreeSet<String>> acc = new java.util.TreeMap<>();

        java.util.Iterator<ghidra.program.model.pcode.PcodeOpAST> it = hf.getPcodeOps();
        List<ghidra.program.model.pcode.PcodeOpAST> all = new ArrayList<>();
        while (it.hasNext()) {
            ghidra.program.model.pcode.PcodeOpAST op = it.next();
            all.add(op);
            for (ghidra.program.model.pcode.Varnode in : op.getInputs()) {
                if (in == null || !in.isInput() || owner.containsKey(in)) {
                    continue;
                }
                String name = inputName(in);
                if (name != null) {
                    owner.put(in, name);
                    offset.put(in, 0L);
                    work.add(in);
                    // The decompiler drops idioms that name a register without using its value
                    // (MOV EDI,EDI; LEA EBX,[EBX]; OR EAX,-1) and saved registers, so an input
                    // left here is an argument (00df9ae0 takes its seed in EAX, #202).
                    if (OTHER_REGS.contains(name.toUpperCase()) && in.getDescendants().hasNext()) {
                        add(acc, name, "u");
                    }
                    // ECX/EDX used at all, as a number too (LEA [ECX*4] in 00d49c20), but not
                    // when they only reach calls: the decompiler lets ECX flow into any callee
                    // whose convention it doesn't know (_ftol2 and the like).
                    if ((name.equals("ecx") || name.equals("edx")) && usedBeyondCalls(in)) {
                        add(acc, name, "u");
                    }
                }
            }
        }
        // Owner and offset of every varnode derived from an input, to a fixed point. A merge
        // (MULTIEQUAL) keeps a known offset only when all its inputs come from the same input at
        // the same offset: `p = arg; while (..) p = p->next;` merges the argument with a value
        // loaded through it, so loads through p are not loads of the argument at a known offset.
        int guard = 0;
        boolean changed = true;
        while (changed && guard < 100_000) {
            while (!work.isEmpty() && guard++ < 100_000) {
                ghidra.program.model.pcode.Varnode vn = work.poll();
                String name = owner.get(vn);
                Long o = offset.get(vn);
                java.util.Iterator<ghidra.program.model.pcode.PcodeOp> ds = vn.getDescendants();
                while (ds.hasNext()) {
                    ghidra.program.model.pcode.PcodeOp d = ds.next();
                    ghidra.program.model.pcode.Varnode outV = d.getOutput();
                    switch (d.getOpcode()) {
                        case PcodeOp.COPY:
                        case PcodeOp.CAST:
                        case PcodeOp.MULTIEQUAL:
                            flow(outV, name, o, owner, offset, work);
                            break;
                        case PcodeOp.INDIRECT:
                            if (d.getInput(0) == vn) {
                                flow(outV, name, o, owner, offset, work);
                            }
                            break;
                        case PcodeOp.INT_ADD:
                        case PcodeOp.PTRSUB: {
                            ghidra.program.model.pcode.Varnode other =
                                d.getInput(0) == vn ? d.getInput(1) : d.getInput(0);
                            Long n = other.isConstant() && o != null ? o + signed(other) : null;
                            flow(outV, name, n, owner, offset, work);
                            break;
                        }
                        case PcodeOp.INT_SUB:
                            if (d.getInput(0) == vn && d.getInput(1).isConstant() && o != null) {
                                flow(outV, name, o - signed(d.getInput(1)), owner, offset, work);
                            }
                            break;
                        case PcodeOp.PTRADD: {
                            if (d.getInput(0) != vn) {
                                break;
                            }
                            ghidra.program.model.pcode.Varnode idx = d.getInput(1);
                            ghidra.program.model.pcode.Varnode sz = d.getInput(2);
                            Long n = idx.isConstant() && sz.isConstant() && o != null
                                ? o + signed(idx) * signed(sz) : null;
                            flow(outV, name, n, owner, offset, work);
                            break;
                        }
                        default:
                            break; // loads, stores, calls: recorded below from the final state
                    }
                }
            }
            changed = false;
            for (ghidra.program.model.pcode.PcodeOpAST op : all) {
                ghidra.program.model.pcode.Varnode outV = op.getOutput();
                if (op.getOpcode() != PcodeOp.MULTIEQUAL || outV == null || offset.get(outV) == null) {
                    continue;
                }
                String name = owner.get(outV);
                Long o = offset.get(outV);
                for (ghidra.program.model.pcode.Varnode in : op.getInputs()) {
                    if (!name.equals(owner.get(in)) || !o.equals(offset.get(in))) {
                        offset.put(outV, null);
                        work.add(outV);
                        changed = true;
                        break;
                    }
                }
            }
        }
        if (guard >= 100_000) {
            return "-\t-1\t-1";
        }
        // What is done through each input, from the final owners and offsets.
        java.util.Map<ghidra.program.model.pcode.Varnode, String> loadedAt = new java.util.HashMap<>();
        for (ghidra.program.model.pcode.PcodeOpAST d : all) {
            ghidra.program.model.pcode.Varnode outV = d.getOutput();
            switch (d.getOpcode()) {
                case PcodeOp.LOAD: {
                    ghidra.program.model.pcode.Varnode a = d.getInput(1);
                    String name = owner.get(a);
                    if (name == null) {
                        break;
                    }
                    Long o = offset.get(a);
                    add(acc, name, o == null ? "r?" : "r" + o + "." + outV.getSize());
                    if (outV != null && o != null) {
                        loadedAt.put(outV, name + "\u0000" + o);
                    }
                    break;
                }
                case PcodeOp.STORE: {
                    ghidra.program.model.pcode.Varnode a = d.getInput(1);
                    String name = owner.get(a);
                    if (name != null) {
                        Long o = offset.get(a);
                        add(acc, name, o == null ? "w?" : "w" + o + "." + d.getInput(2).getSize());
                    }
                    String stored = owner.get(d.getInput(2));
                    if (stored != null) {
                        add(acc, stored, "e");
                    }
                    break;
                }
                case PcodeOp.CALL:
                case PcodeOp.CALLIND:
                    for (int k = 1; k < d.getNumInputs(); k++) {
                        String name = owner.get(d.getInput(k));
                        if (name != null) {
                            ghidra.program.model.pcode.Varnode t = d.getInput(0);
                            add(acc, name, "p" + (d.getOpcode() == PcodeOp.CALL
                                ? String.format("%08x", t.getOffset()) : "?"));
                        }
                    }
                    break;
                default:
                    break; // compares, null checks, returns: no memory access
            }
        }
        // Pointers loaded from an argument and then dereferenced (one level), and the LOADs and
        // STOREs whose address no input explains.
        int untrackedLoads = 0, untrackedStores = 0;
        for (ghidra.program.model.pcode.PcodeOpAST op : all) {
            int oc = op.getOpcode();
            if (oc != PcodeOp.LOAD && oc != PcodeOp.STORE) {
                continue;
            }
            ghidra.program.model.pcode.Varnode a = op.getInput(1);
            if (owner.containsKey(a)) {
                continue;
            }
            String from = loadedAt.get(root(a));
            if (from != null) {
                String[] parts = from.split("\u0000");
                add(acc, parts[0], "c" + parts[1]);
            }
            if (oc == PcodeOp.LOAD) {
                untrackedLoads++;
            }
            else {
                untrackedStores++;
            }
        }
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, java.util.TreeSet<String>> e : acc.entrySet()) {
            if (sb.length() > 0) {
                sb.append('|');
            }
            sb.append(e.getKey()).append('=').append(String.join(",", e.getValue()));
        }
        return sb + "\t" + untrackedLoads + "\t" + untrackedStores;
    }

    /**
     * Whether a value reaches any operation other than a call argument, following copies, casts,
     * merges and the INDIRECTs around calls.
     */
    private static boolean usedBeyondCalls(ghidra.program.model.pcode.Varnode start) {
        java.util.ArrayDeque<ghidra.program.model.pcode.Varnode> todo = new java.util.ArrayDeque<>();
        java.util.Set<ghidra.program.model.pcode.Varnode> seen = new java.util.HashSet<>();
        todo.add(start);
        while (!todo.isEmpty()) {
            ghidra.program.model.pcode.Varnode v = todo.poll();
            if (!seen.add(v)) {
                continue;
            }
            java.util.Iterator<ghidra.program.model.pcode.PcodeOp> ds = v.getDescendants();
            while (ds.hasNext()) {
                ghidra.program.model.pcode.PcodeOp d = ds.next();
                switch (d.getOpcode()) {
                    case PcodeOp.COPY:
                    case PcodeOp.CAST:
                    case PcodeOp.MULTIEQUAL:
                    case PcodeOp.INDIRECT:
                        if (d.getOutput() != null) {
                            todo.add(d.getOutput());
                        }
                        break;
                    case PcodeOp.CALL:
                    case PcodeOp.CALLIND:
                        if (d.getInput(0) == v) {
                            return true; // the call target itself
                        }
                        break;
                    default:
                        return true;
                }
            }
        }
        return false;
    }

    /** The varnode an address was derived from by copies, casts and constant adds. */
    private static ghidra.program.model.pcode.Varnode root(ghidra.program.model.pcode.Varnode v) {
        for (int i = 0; i < 16 && v != null; i++) {
            ghidra.program.model.pcode.PcodeOp def = v.getDef();
            if (def == null) {
                return v;
            }
            int oc = def.getOpcode();
            if (oc == PcodeOp.COPY || oc == PcodeOp.CAST) {
                v = def.getInput(0);
            }
            else if ((oc == PcodeOp.INT_ADD || oc == PcodeOp.PTRSUB) && def.getInput(1).isConstant()) {
                v = def.getInput(0);
            }
            else {
                return v;
            }
        }
        return v;
    }

    private static void flow(ghidra.program.model.pcode.Varnode outV, String name, Long o,
            Map<ghidra.program.model.pcode.Varnode, String> owner,
            Map<ghidra.program.model.pcode.Varnode, Long> offset,
            java.util.ArrayDeque<ghidra.program.model.pcode.Varnode> work) {
        if (outV == null) {
            return;
        }
        if (owner.containsKey(outV)) {
            Long have = offset.get(outV);
            if (have == null || (owner.get(outV).equals(name) && have.equals(o))) {
                return; // already unknown, or reached again the same way
            }
            // two offsets (or two inputs) meet in a merge or a loop: the offset is unknown
            offset.put(outV, null);
            work.add(outV);
            return;
        }
        owner.put(outV, name);
        offset.put(outV, o);
        work.add(outV);
    }

    private static void add(java.util.TreeMap<String, java.util.TreeSet<String>> acc, String name,
            String item) {
        acc.computeIfAbsent(name, k -> new java.util.TreeSet<>()).add(item);
    }

    private static long signed(ghidra.program.model.pcode.Varnode c) {
        long v = c.getOffset();
        int bits = 8 * c.getSize();
        return bits >= 64 ? v : (v << (64 - bits)) >> (64 - bits);
    }

    /**
     * ecx, edx, s<n> for a stack argument n bytes past the return address, or eax, ebx, esi, edi,
     * ebp; null otherwise.
     */
    private String inputName(ghidra.program.model.pcode.Varnode in) {
        Address a = in.getAddress();
        if (a.isStackAddress()) {
            long off = a.getOffset();
            return off >= 4 ? "s" + (off - 4) : null;
        }
        if (a.isRegisterAddress()) {
            Register r = currentProgram.getRegister(a, in.getSize());
            if (r == null) {
                return null;
            }
            Register b = r.getBaseRegister();
            String n = (b == null ? r : b).getName().toUpperCase();
            if (n.equals("ECX") || n.equals("RCX")) {
                return "ecx";
            }
            if (n.equals("EDX") || n.equals("RDX")) {
                return "edx";
            }
            if (OTHER_REGS.contains(n)) {
                return n.toLowerCase();
            }
        }
        return null;
    }

    /** Registers no standard convention passes arguments in (whole-program optimized code does). */
    private static final java.util.Set<String> OTHER_REGS = java.util.Set.of("EAX", "EBX", "ESI", "EDI", "EBP");

    private static final int MAX_CONSTS = 32;

    private static String joinLongs(java.util.Collection<Long> values) {
        List<String> out = new ArrayList<>();
        for (Long v : values) {
            out.add(Long.toString(v));
        }
        return String.join(",", out);
    }

    /**
     * The constants a function compares and computes with, as test inputs for the oracle check:
     * immediate operands that are not addresses or stack adjustments (CMP EAX,0x14; AND ECX,0x1FF),
     * and the float/double values that x87 and SSE instructions load from absolute addresses
     * (FCOMP double ptr [0x01012060] -> its 8 bytes as a double). Floats are written f&lt;8 hex
     * digits&gt;, doubles d&lt;16 hex digits&gt;; at most 32 of each kind.
     */
    private void collectConstants(Instruction ins, String mnem, java.util.Set<Long> ints,
            java.util.Set<String> floats) {
        // ADD/SUB ESP,n and the like: ESP or EBP as a bare register operand (not inside a
        // memory operand such as CMP dword ptr [EBP+0x8],0xa, whose 0xa is wanted).
        boolean stackAdjust = false;
        for (int i = 0; i < ins.getNumOperands(); i++) {
            if (isBracketed(ins, i)) {
                continue;
            }
            for (Object o : ins.getOpObjects(i)) {
                if (o instanceof Register) {
                    String n = ((Register) o).getName().toUpperCase();
                    if (n.equals("ESP") || n.equals("EBP")) {
                        stackAdjust = true;
                    }
                }
            }
        }
        boolean x87 = mnem.startsWith("F");
        boolean sse = mnem.endsWith("SS") || mnem.endsWith("SD") || mnem.startsWith("COMIS")
            || mnem.startsWith("UCOMIS") || mnem.startsWith("CVT");
        for (int i = 0; i < ins.getNumOperands(); i++) {
            String rep = ins.getDefaultOperandRepresentation(i);
            if (!isBracketed(ins, i)) {
                if (stackAdjust || mnem.startsWith("RET") || ins.getFlowType().isCall()
                    || ins.getFlowType().isJump()) {
                    continue;
                }
                for (Object o : ins.getOpObjects(i)) {
                    if (o instanceof Scalar) {
                        long v = ((Scalar) o).getSignedValue();
                        if (Math.abs(v) < imageBase && ints.size() < MAX_CONSTS) {
                            ints.add(v);
                        }
                    }
                }
                continue;
            }
            if (!(x87 || sse)) {
                continue;
            }
            // Ghidra writes x87 memory operands as "float ptr"/"double ptr", SSE ones as
            // "dword ptr"/"qword ptr"; 80-bit ("float10") operands are skipped.
            int size = rep.startsWith("dword ptr") || rep.startsWith("float ptr") ? 4
                : rep.startsWith("qword ptr") || rep.startsWith("double ptr") ? 8 : 0;
            if (size == 0) {
                continue;
            }
            Long abs = null;
            boolean indexed = false;
            for (Object o : ins.getOpObjects(i)) {
                if (o instanceof Register) {
                    indexed = true;
                }
                else if (o instanceof Scalar && ((Scalar) o).getUnsignedValue() >= imageBase) {
                    abs = ((Scalar) o).getUnsignedValue();
                }
                else if (o instanceof Address && ((Address) o).getOffset() >= imageBase) {
                    abs = ((Address) o).getOffset();
                }
            }
            if (abs == null || indexed || floats.size() >= MAX_CONSTS) {
                continue;
            }
            Address a = toAddr(abs);
            if (!NvCommon.inInitializedMemory(currentProgram, a)) {
                continue;
            }
            byte[] b = NvCommon.readBytes(currentProgram, a, size);
            if (b == null || b.length != size) {
                continue;
            }
            long bits = NvCommon.le(b, 0, size);
            floats.add(size == 4 ? String.format("f%08x", bits) : String.format("d%016x", bits));
        }
    }

    /**
     * ECX and EDX among the registers the entry block reads before it writes them, following
     * fall-through only (the first CALL, jump or return ends the block). Sub-registers count as
     * their full register (CL reads ECX).
     */
    private String registersIn(Listing listing, Address entry, AddressSetView body) {
        String[] watched = { "ECX", "EDX" };
        boolean[] written = new boolean[2];
        boolean[] readFirst = new boolean[2];
        Instruction ins = listing.getInstructionAt(entry);
        for (int n = 0; ins != null && body.contains(ins.getAddress()) && n < 64; n++) {
            String mnem = ins.getMnemonicString().toUpperCase();
            // Idioms that name a register without using its value: PUSH ECX in a prologue
            // reserves a local slot, XOR/SUB ECX,ECX zeroes it.
            boolean push = mnem.equals("PUSH");
            boolean zeroing = (mnem.equals("XOR") || mnem.equals("SUB")) && ins.getNumOperands() == 2
                && ins.getDefaultOperandRepresentation(0).equals(ins.getDefaultOperandRepresentation(1));
            for (int k = 0; k < 2; k++) {
                if (written[k] || readFirst[k]) {
                    continue;
                }
                if (push) {
                    continue;
                }
                if (zeroing && touches(ins.getResultObjects(), watched[k])) {
                    written[k] = true;
                }
                else if (touches(ins.getInputObjects(), watched[k])) {
                    readFirst[k] = true;
                }
                else if (touches(ins.getResultObjects(), watched[k])) {
                    written[k] = true;
                }
            }
            FlowType ft = ins.getFlowType();
            if (!ft.isFallthrough() || ft.isCall()) {
                break;
            }
            ins = ins.getNext();
        }
        List<String> out = new ArrayList<>();
        for (int k = 0; k < 2; k++) {
            if (readFirst[k]) {
                out.add(watched[k]);
            }
        }
        return String.join(",", out);
    }

    private static boolean touches(Object[] objects, String full) {
        for (Object o : objects) {
            if (o instanceof Register) {
                Register r = (Register) o;
                Register base = r.getBaseRegister();
                String n = r.getName().toUpperCase();
                String b = base == null ? n : base.getName().toUpperCase();
                if (n.equals(full) || b.equals(full) || b.equals("R" + full.substring(1))) {
                    return true;
                }
            }
        }
        return false;
    }

    /**
     * End of the stack arguments the body reads, in bytes past the return address: the largest
     * [EBP+d] with d >= 8, as d - 8 + 4 (one dword; an 8-byte FLD QWORD at the last slot reads as
     * 4 bytes short, so this is a lower bound). -1 without an EBP frame (ESP-relative offsets move
     * with every PUSH, so they are not measured).
     */
    private static long stackArgBytes(Listing listing, AddressSetView body, boolean ebpFrame) {
        if (!ebpFrame) {
            return -1;
        }
        long end = 0;
        for (Instruction ins : listing.getInstructions(body, true)) {
            for (int i = 0; i < ins.getNumOperands(); i++) {
                if (!isBracketed(ins, i)) {
                    continue;
                }
                boolean ebp = false;
                long disp = Long.MIN_VALUE;
                for (Object o : ins.getOpObjects(i)) {
                    if (o instanceof Register && ((Register) o).getName().equalsIgnoreCase("EBP")) {
                        ebp = true;
                    }
                    else if (o instanceof Scalar) {
                        disp = ((Scalar) o).getSignedValue();
                    }
                }
                if (ebp && disp >= 8 && disp < 0x400) {
                    end = Math.max(end, disp - 8 + 4);
                }
            }
        }
        return end;
    }

    private enum Base { STACK, ABSOLUTE, POINTER }

    /** Where a memory operand points: the frame, an absolute address (optionally indexed), or elsewhere. */
    private Base base(Instruction ins, int i, boolean ebpFrame) {
        boolean stack = false, absolute = false, otherReg = false;
        for (Object o : ins.getOpObjects(i)) {
            if (o instanceof Register) {
                String n = ((Register) o).getName().toUpperCase();
                if (n.equals("ESP") || (ebpFrame && n.equals("EBP"))) {
                    stack = true;
                }
                else if (n.equals("FS") || n.equals("DS") || n.equals("SS") || n.equals("ES")) {
                    // segment prefix: FS is reported separately; the others are the flat model
                    if (n.equals("FS")) {
                        otherReg = true;
                    }
                }
                else {
                    otherReg = true;
                }
            }
            else if (o instanceof Scalar) {
                if (((Scalar) o).getUnsignedValue() >= imageBase) {
                    absolute = true;
                }
            }
            else if (o instanceof Address) {
                if (((Address) o).getOffset() >= imageBase) {
                    absolute = true;
                }
            }
        }
        if (stack) {
            return Base.STACK;
        }
        if (absolute) {
            return Base.ABSOLUTE; // [abs] or [abs + index*scale]
        }
        return otherReg ? Base.POINTER : Base.ABSOLUTE;
    }

    private static boolean isBracketed(Instruction ins, int i) {
        String s = ins.getDefaultOperandRepresentation(i);
        return s.contains("[");
    }

    private static boolean hasStore(Instruction ins) {
        for (PcodeOp op : ins.getPcode()) {
            if (op.getOpcode() == PcodeOp.STORE) {
                return true;
            }
        }
        return false;
    }

    private static boolean hasEbpFrame(Listing listing, Address entry) {
        Instruction a = listing.getInstructionAt(entry);
        if (a == null || !a.toString().equalsIgnoreCase("PUSH EBP")) {
            return false;
        }
        Instruction b = a.getNext();
        return b != null && b.toString().replace(" ", "").equalsIgnoreCase("MOVEBP,ESP");
    }
}

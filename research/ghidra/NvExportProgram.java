// Complete-by-construction export of every function in a Ghidra program.
//
// Args: out=<dir> [threads=N] [timeout=<sec per function>] [decompile=1|0]
//       [disasm=1|0] [start=<hex> end=<hex>]
//
// Writes <dir>/functions.jsonl (one JSON object per function, sorted by
// entry address) and <dir>/manifest.json. The script walks
// FunctionManager.getFunctions(true), so a function can only be missing
// from the export if the walk itself skips it. The manifest gate checks
// that the number of records written equals the function count Ghidra
// reports through a different call (getFunctionCount minus external
// functions); if they differ the script throws and writes
// manifest.failed.json instead of manifest.json.
//
// Each record also holds the vtable slots that point at the function, its
// class (the parent namespace, when Ghidra made it a class) and, for every
// data reference whose target carries an nv-names bookmark, where that name
// came from. The function's own provenance is in its src: and pin: tags.
// The disassembly is part of each record by default (disasm=0 leaves it out
// to make the file smaller).
//
// A matching function count does not prove that all code is covered:
// code Ghidra never turned into a function is invisible to the walk. The
// manifest therefore also lists executable bytes outside every function
// and vtable slots whose target is not a function entry.
//
// The output holds decompiled text and disassembly of a program you own.
// Keep it in the private research tree. Never commit it.
//
// @category NV

import java.io.BufferedOutputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.security.MessageDigest;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileOptions;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.framework.Application;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressRange;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.listing.Bookmark;
import ghidra.program.model.listing.BookmarkType;
import ghidra.program.model.listing.CodeUnit;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionIterator;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.GhidraClass;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.listing.Program;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.Namespace;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolIterator;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.program.model.symbol.SymbolType;
import ghidra.util.task.TaskMonitor;

public class NvExportProgram extends GhidraScript {
    private static final String ME = "NvExportProgram";
    private static final int STRING_MAX_CHARS = 200;
    private static final int MAX_UNCOVERED_LISTED = 20000;
    private static final int MAX_VTABLE_SLOTS = 4096;

    private Program prog;
    private Listing listing;
    private SymbolTable symbols;
    private FunctionManager fm;
    private DecompileOptions decompOptions;
    private int timeoutSec;
    private boolean doDecompile;
    private boolean doDisasm;
    // Vtables found before the records are built; slotsByTarget lets a record
    // list the slots that point at its function.
    private List<VTable> vtables;
    private final Map<Long, List<Map<String, Object>>> slotsByTarget = new HashMap<>();

    private final List<DecompInterface> decompilers =
        Collections.synchronizedList(new ArrayList<>());
    private final ThreadLocal<DecompInterface> localDecompiler = new ThreadLocal<>();

    /** One finished record: the JSON line and its decompile status. */
    private static final class Rec {
        final String line;
        final String status;

        Rec(String line, String status) {
            this.line = line;
            this.status = status;
        }
    }

    @Override
    public void run() throws Exception {
        long t0 = System.nanoTime();
        Map<String, String> args = NvCommon.parseArgs(ME, getScriptArgs(),
            Set.of("out", "threads", "timeout", "decompile", "disasm", "start", "end"));
        Path outDir = Paths.get(NvCommon.require(ME, args, "out"));
        int cpus = Runtime.getRuntime().availableProcessors();
        int threads = NvCommon.intArg(ME, args, "threads", Math.min(8, cpus), 1, 64);
        timeoutSec = NvCommon.intArg(ME, args, "timeout", 60, 1, 86400);
        doDecompile = NvCommon.flag(ME, args, "decompile", true);
        doDisasm = NvCommon.flag(ME, args, "disasm", true);
        if (args.containsKey("start") != args.containsKey("end")) {
            throw new IllegalArgumentException(ME + ": start and end must be given together");
        }
        if (currentProgram == null) {
            throw new IllegalStateException(ME + ": no program is open");
        }
        prog = currentProgram;
        NvCommon.exeSha256(prog, ME);
        listing = prog.getListing();
        symbols = prog.getSymbolTable();
        fm = prog.getFunctionManager();

        AddressSet range = null;
        if (args.containsKey("start")) {
            long s = NvCommon.parseHex(ME, "start", args.get("start"));
            long e = NvCommon.parseHex(ME, "end", args.get("end"));
            if (e < s) {
                throw new IllegalArgumentException(ME + ": end is below start");
            }
            range = new AddressSet(toAddr(s), toAddr(e));
        }

        Files.createDirectories(outDir);
        Path jsonl = outDir.resolve("functions.jsonl");
        Path manifestPath = outDir.resolve("manifest.json");
        Path failedPath = outDir.resolve("manifest.failed.json");
        Files.deleteIfExists(manifestPath);
        Files.deleteIfExists(failedPath);
        Files.deleteIfExists(jsonl);

        // Counts from two independent calls: the gate compares them.
        int apiCount = fm.getFunctionCount();
        int externalCount = 0;
        for (Function f : fm.getExternalFunctions()) {
            externalCount++;
        }

        List<Function> functions = new ArrayList<>();
        FunctionIterator it = range == null ? fm.getFunctions(true) : fm.getFunctions(range, true);
        while (it.hasNext()) {
            functions.add(it.next());
        }
        int expected;
        if (range == null) {
            expected = apiCount - externalCount;
        }
        else {
            // A range export has no whole-program API count to compare with, so
            // count the functions in the range again by walking entry points.
            expected = 0;
            for (Function f : fm.getFunctions(true)) {
                if (range.contains(f.getEntryPoint())) {
                    expected++;
                }
            }
        }
        println(ME + ": " + functions.size() + " functions to export (API count " + apiCount + ", " +
            externalCount + " external), threads=" + threads + ", decompile=" + doDecompile);

        if (doDecompile) {
            decompOptions = new DecompileOptions();
            decompOptions.grabFromProgram(prog);
        }
        vtables = walkVtables();
        for (VTable t : vtables) {
            for (int slot = 0; slot < t.targets.size(); slot++) {
                Map<String, Object> e = NvCommon.obj();
                e.put("table", t.symbol.getName(true));
                e.put("table_address", NvCommon.addr(t.at));
                Namespace tns = t.symbol.getParentNamespace();
                e.put("table_class", tns instanceof GhidraClass ? tns.getName(true) : "");
                e.put("slot", slot);
                slotsByTarget.computeIfAbsent(t.targets.get(slot), k -> new ArrayList<>()).add(e);
            }
        }

        Map<String, Integer> status = new TreeMap<>();
        for (String s : new String[] { "ok", "timeout", "error", "skipped" }) {
            status.put(s, 0);
        }
        MessageDigest sha = MessageDigest.getInstance("SHA-256");
        long bytesWritten = 0;
        int written = 0;

        ExecutorService pool = Executors.newFixedThreadPool(threads);
        try (OutputStream os = new BufferedOutputStream(Files.newOutputStream(jsonl), 1 << 16)) {
            ArrayDeque<Future<Rec>> window = new ArrayDeque<>();
            int windowMax = threads * 8;
            int nextReport = Math.max(1, functions.size() / 10);
            int idx = 0;
            while (idx < functions.size() || !window.isEmpty()) {
                monitor.checkCancelled();
                while (idx < functions.size() && window.size() < windowMax) {
                    final Function f = functions.get(idx++);
                    window.add(pool.submit((Callable<Rec>) () -> buildRecord(f)));
                }
                Rec r = window.poll().get();
                byte[] bytes = (r.line + "\n").getBytes(StandardCharsets.UTF_8);
                os.write(bytes);
                sha.update(bytes);
                bytesWritten += bytes.length;
                written++;
                status.merge(r.status, 1, Integer::sum);
                if (written >= nextReport) {
                    println(ME + ": " + written + "/" + functions.size() + " records");
                    nextReport += Math.max(1, functions.size() / 10);
                }
            }
        }
        finally {
            pool.shutdownNow();
            synchronized (decompilers) {
                for (DecompInterface d : decompilers) {
                    d.dispose();
                }
            }
        }

        // ---- manifest ----
        Map<String, Object> m = NvCommon.obj();
        m.put("tool", ME);
        m.put("exe_sha256", NvCommon.exeSha256(prog, ME));
        m.put("exe_md5", prog.getExecutableMD5());
        m.put("ghidra_version", Application.getApplicationVersion());
        m.put("program_name", prog.getName());
        m.put("image_base", NvCommon.addr(prog.getImageBase()));
        m.put("function_count_api", apiCount);
        m.put("function_count_external_excluded", externalCount);
        m.put("records_expected", expected);
        m.put("records_written", written);
        m.put("partial", range != null);
        if (range != null) {
            m.put("range", NvCommon.rangesToString(range, 1));
        }
        m.put("status_counts", status);
        m.put("functions_jsonl", "functions.jsonl");
        m.put("functions_jsonl_bytes", bytesWritten);
        m.put("functions_jsonl_sha256", NvCommon.bytesToHex(sha.digest()));
        Map<String, Object> opts = NvCommon.obj();
        opts.put("threads", threads);
        opts.put("timeout_sec", timeoutSec);
        opts.put("decompile", doDecompile);
        opts.put("disasm", doDisasm);
        m.put("options", opts);
        m.put("analysis_options", new TreeMap<>(getCurrentAnalysisOptionsAndValues(prog)));

        m.put("uncovered_executable", uncoveredExecutable());
        m.put("vtable_checks", vtableChecks(vtables));
        m.put("elapsed_seconds", Math.round((System.nanoTime() - t0) / 1e6) / 1000.0);

        boolean gate = written == expected;
        Map<String, Object> g = NvCommon.obj();
        g.put("rule", "records_written == function_count_api - external functions " +
            "(for a range export: functions with an entry in the range)");
        g.put("passed", gate);
        m.put("gate", g);

        if (!gate) {
            NvCommon.writeText(failedPath, NvCommon.toPrettyJson(m));
            throw new IllegalStateException(ME + ": GATE FAILED: wrote " + written +
                " records but expected " + expected + " (API count " + apiCount + ", external " +
                externalCount + "). See " + failedPath);
        }
        if (doDecompile && !functions.isEmpty() && status.get("ok") == 0) {
            NvCommon.writeText(failedPath, NvCommon.toPrettyJson(m));
            throw new IllegalStateException(ME + ": no function decompiled successfully " +
                "(is the decompiler available?). See " + failedPath);
        }
        NvCommon.writeText(manifestPath, NvCommon.toPrettyJson(m));
        println(ME + ": wrote " + written + " records to " + jsonl + " (" + status + "), " +
            "manifest " + manifestPath);
    }

    // ------------------------------------------------------------------
    // One function record
    // ------------------------------------------------------------------

    private Rec buildRecord(Function f) {
        Map<String, Object> r = NvCommon.obj();
        String status = "error";
        try {
            r.put("entry", NvCommon.addr(f.getEntryPoint()));
            r.put("name", f.getName());
            Namespace ns = f.getParentNamespace();
            r.put("namespace", ns == null || ns.isGlobal() ? "" : ns.getName(true));
            r.put("class", ns instanceof GhidraClass ? ns.getName(true) : "");
            r.put("name_source", f.getSymbol().getSource().name());
            List<String> tags = new ArrayList<>();
            f.getTags().forEach(t -> tags.add(t.getName()));
            Collections.sort(tags);
            r.put("tags", tags);
            AddressSetView body = f.getBody();
            List<Object> ranges = new ArrayList<>();
            for (AddressRange rg : body.getAddressRanges()) {
                ranges.add(List.of(NvCommon.addr(rg.getMinAddress()), NvCommon.addr(rg.getMaxAddress())));
            }
            r.put("body", ranges);
            r.put("size", body.getNumAddresses());
            r.put("prototype", f.getPrototypeString(false, true));
            r.put("calling_convention", f.getCallingConventionName());
            r.put("param_count", f.getParameterCount());
            int purge = f.getStackPurgeSize();
            boolean purgeKnown = purge != Function.UNKNOWN_STACK_DEPTH_CHANGE &&
                purge != Function.INVALID_STACK_DEPTH_CHANGE;
            r.put("stack_purge", purgeKnown ? (Object) purge : null);
            r.put("is_thunk", f.isThunk());
            Function thunked = f.isThunk() ? f.getThunkedFunction(false) : null;
            r.put("thunk_target", thunked == null ? null : functionRef(thunked));
            List<Map<String, Object>> slots = slotsByTarget.get(f.getEntryPoint().getOffset());
            r.put("vtable_slots", slots == null ? new ArrayList<Object>() : new ArrayList<Object>(slots));
            r.put("callers", sortedRefs(f.getCallingFunctions(TaskMonitor.DUMMY)));
            r.put("callees", sortedRefs(f.getCalledFunctions(TaskMonitor.DUMMY)));
            dataRefs(f, body, r);
            status = decompile(f, r);
            if (doDisasm) {
                r.put("disasm", disassembly(body));
            }
        }
        catch (Throwable t) {
            // A record is always produced: complete by construction.
            r.put("entry", NvCommon.addr(f.getEntryPoint()));
            r.putIfAbsent("name", f.getName());
            r.put("decompile_status", "error");
            r.put("decompile_error", "record build failed: " + t);
            status = "error";
        }
        return new Rec(NvCommon.toJson(r), status);
    }

    private static String functionRef(Function f) {
        if (f.isExternal()) {
            return "ext:" + f.getName(true);
        }
        return NvCommon.addr(f.getEntryPoint());
    }

    private static List<String> sortedRefs(Set<Function> fs) {
        TreeSet<String> out = new TreeSet<>();
        for (Function x : fs) {
            out.add(functionRef(x));
        }
        return new ArrayList<>(out);
    }

    private void dataRefs(Function f, AddressSetView body, Map<String, Object> r) {
        // key: address + type so the output is sorted and de-duplicated
        TreeMap<String, Map<String, Object>> refs = new TreeMap<>();
        TreeMap<String, Map<String, Object>> strings = new TreeMap<>();
        for (Instruction ins : listing.getInstructions(body, true)) {
            for (Reference ref : ins.getReferencesFrom()) {
                if (!ref.isMemoryReference() || ref.getReferenceType().isFlow()) {
                    continue;
                }
                Address to = ref.getToAddress();
                String addr = NvCommon.addr(to);
                String type = ref.getReferenceType().getName();
                String key = addr + " " + type;
                if (!refs.containsKey(key)) {
                    Map<String, Object> e = NvCommon.obj();
                    e.put("address", addr);
                    e.put("type", type);
                    // Only real names: Ghidra's DAT_/FUN_/LAB_ placeholders add nothing.
                    Symbol s = symbols.getPrimarySymbol(to);
                    boolean named = s != null && s.getSource() != SourceType.DEFAULT;
                    e.put("label", named ? s.getName(true) : "");
                    // Where the name came from, as NvImportNameMap recorded it.
                    Bookmark bm = prog.getBookmarkManager().getBookmark(to, BookmarkType.INFO,
                        NvCommon.NV_NAMES_BOOKMARK);
                    e.put("provenance", bm == null ? "" : bm.getComment());
                    refs.put(key, e);
                }
                if (!strings.containsKey(addr)) {
                    Map<String, Object> s = stringAt(to);
                    if (s != null) {
                        strings.put(addr, s);
                    }
                }
            }
        }
        r.put("data_refs", new ArrayList<>(refs.values()));
        r.put("strings", new ArrayList<>(strings.values()));
    }

    /** A string at this address, defined by Ghidra or read raw from memory; else null. */
    private Map<String, Object> stringAt(Address a) {
        String value = null;
        boolean defined = false;
        Data d = listing.getDataAt(a);
        if (d != null && d.hasStringValue()) {
            Object v = d.getValue();
            if (v instanceof String) {
                value = (String) v;
                defined = true;
            }
        }
        else if (d == null || !d.isDefined()) {
            if (!NvCommon.inExecutableMemory(prog, a)) {
                value = NvCommon.readAsciiZ(prog, a, 4, 1024);
            }
        }
        if (value == null) {
            return null;
        }
        Map<String, Object> s = NvCommon.obj();
        s.put("address", NvCommon.addr(a));
        s.put("value", NvCommon.truncate(value, STRING_MAX_CHARS));
        s.put("truncated", value.length() > STRING_MAX_CHARS);
        s.put("defined_by_ghidra", defined);
        return s;
    }

    private String decompile(Function f, Map<String, Object> r) {
        if (!doDecompile) {
            r.put("decompile_status", "skipped");
            r.put("decompile_error", null);
            return "skipped";
        }
        String status;
        String error = null;
        String c = null;
        try {
            DecompInterface di = decompilerForThisThread();
            DecompileResults res = di.decompileFunction(f, timeoutSec, TaskMonitor.DUMMY);
            if (res == null) {
                status = "error";
                error = "decompiler returned no result";
            }
            else if (res.decompileCompleted() && res.getDecompiledFunction() != null) {
                status = "ok";
                c = res.getDecompiledFunction().getC();
            }
            else if (res.isTimedOut()) {
                status = "timeout";
                error = "no result within " + timeoutSec + " s";
            }
            else {
                status = "error";
                error = res.getErrorMessage();
                if (error == null || error.isEmpty()) {
                    error = res.failedToStart() ? "decompiler failed to start" : "decompile did not complete";
                }
            }
        }
        catch (Exception e) {
            status = "error";
            error = e.toString();
        }
        r.put("decompile_status", status);
        r.put("decompile_error", error == null ? null : NvCommon.truncate(error.trim(), 2000));
        r.put("c", c);
        return status;
    }

    private DecompInterface decompilerForThisThread() {
        DecompInterface di = localDecompiler.get();
        if (di == null) {
            di = new DecompInterface();
            di.setOptions(decompOptions);
            di.toggleCCode(true);
            di.toggleSyntaxTree(false);
            di.setSimplificationStyle("decompile");
            if (!di.openProgram(prog)) {
                String msg = di.getLastMessage();
                di.dispose();
                throw new IllegalStateException("cannot open decompiler: " + msg);
            }
            decompilers.add(di);
            localDecompiler.set(di);
        }
        return di;
    }

    private List<String> disassembly(AddressSetView body) {
        List<String> lines = new ArrayList<>();
        for (CodeUnit cu : listing.getCodeUnits(body, true)) {
            lines.add(NvCommon.addr(cu.getAddress()) + "  " + cu.toString());
        }
        return lines;
    }

    // ------------------------------------------------------------------
    // Coverage and vtable checks (whole program, whatever the range)
    // ------------------------------------------------------------------

    private Map<String, Object> uncoveredExecutable() throws Exception {
        AddressSet exec = new AddressSet();
        for (MemoryBlock b : prog.getMemory().getBlocks()) {
            if (b.isExecute() && b.isInitialized()) {
                exec.add(b.getStart(), b.getEnd());
            }
        }
        AddressSet covered = new AddressSet();
        for (Function f : fm.getFunctions(true)) {
            covered.add(f.getBody());
        }
        AddressSet uncovered = exec.subtract(covered);

        long total = 0;
        long paddingBytes = 0;
        int paddingRuns = 0;
        int nonPaddingRuns = 0;
        List<Object> listed = new ArrayList<>();
        for (AddressRange rg : uncovered) {
            long size = rg.getLength();
            total += size;
            boolean padding = isPadding(rg);
            if (padding) {
                paddingBytes += size;
                paddingRuns++;
                continue;
            }
            nonPaddingRuns++;
            if (listed.size() < MAX_UNCOVERED_LISTED) {
                Map<String, Object> e = NvCommon.obj();
                e.put("start", NvCommon.addr(rg.getMinAddress()));
                e.put("end", NvCommon.addr(rg.getMaxAddress()));
                e.put("size", size);
                AddressSet one = new AddressSet(rg);
                e.put("has_instructions", listing.getInstructions(one, true).hasNext());
                e.put("first_bytes", NvCommon.bytesToHex(NvCommon.readBytes(prog, rg.getMinAddress(), 8)));
                MemoryBlock b = prog.getMemory().getBlock(rg.getMinAddress());
                e.put("block", b == null ? "" : b.getName());
                listed.add(e);
            }
        }
        Map<String, Object> u = NvCommon.obj();
        u.put("executable_bytes", exec.getNumAddresses());
        u.put("covered_bytes", exec.getNumAddresses() - total);
        u.put("uncovered_bytes_total", total);
        u.put("uncovered_padding_only_bytes", paddingBytes);
        u.put("uncovered_padding_only_runs", paddingRuns);
        u.put("uncovered_non_padding_bytes", total - paddingBytes);
        u.put("uncovered_non_padding_runs", nonPaddingRuns);
        u.put("padding_definition",
            "runs made only of 0x00, 0x90 and 0xCC bytes or multi-byte x86 NOPs " +
                "(0F 1F forms, lea reg,[reg+0], mov reg,reg, with 66/2E prefixes)");
        u.put("ranges_listed_are", "non-padding runs");
        u.put("ranges_truncated", nonPaddingRuns > listed.size());
        u.put("ranges", listed);
        return u;
    }

    /**
     * True when every byte of the range is alignment padding: 0x00, 0x90 or
     * 0xCC bytes, or the multi-byte NOP forms compilers emit (66/2E
     * prefixed NOP, 0F 1F /0, lea reg,[reg+0], mov reg,reg).
     */
    private boolean isPadding(AddressRange rg) {
        long len = rg.getLength();
        if (len > (1 << 20)) {
            return false; // far too large to be alignment padding
        }
        byte[] b = NvCommon.readBytes(prog, rg.getMinAddress(), (int) len);
        if (b.length < len) {
            return false; // unreadable: treat as real content
        }
        int i = 0;
        while (i < b.length) {
            int n = nopLength(b, i);
            if (n == 0) {
                return false;
            }
            i += n;
        }
        return i == b.length;
    }

    /** Length of the padding instruction at b[i], or 0 when it is not padding. */
    static int nopLength(byte[] b, int i) {
        int p = i;
        while (p < b.length && (b[p] == 0x66 || b[p] == 0x2e || b[p] == 0x3e)) {
            p++;
        }
        if (p >= b.length) {
            return 0;
        }
        int op = b[p] & 0xff;
        if (op == 0x90 || op == 0xcc) {
            return p - i + 1;
        }
        if (op == 0x00) {
            return p == i ? 1 : 0;
        }
        int modrmAt;
        if (op == 0x0f && p + 2 < b.length && (b[p + 1] & 0xff) == 0x1f) {
            modrmAt = p + 2; // multi-byte NOP: 0F 1F /0
        }
        else if (op == 0x8d || op == 0x89 || op == 0x8b) {
            modrmAt = p + 1;
        }
        else {
            return 0;
        }
        if (modrmAt >= b.length) {
            return 0;
        }
        int modrm = b[modrmAt] & 0xff;
        int mod = modrm >> 6;
        int reg = (modrm >> 3) & 7;
        int rm = modrm & 7;
        int len = modrmAt + 1;
        int base = rm;
        if (mod != 3 && rm == 4) {
            if (len >= b.length) {
                return 0;
            }
            int sib = b[len++] & 0xff;
            base = sib & 7;
            if (((sib >> 3) & 7) != 4 && op != 0x0f) {
                return 0; // lea with a real index register is not a no-op
            }
        }
        int disp = mod == 1 ? 1 : mod == 2 ? 4 : (mod == 0 && base == 5 ? 4 : 0);
        if (len + disp > b.length) {
            return 0;
        }
        if (op == 0x0f) {
            return reg == 0 ? len + disp - i : 0;
        }
        if (op == 0x8d) {
            // lea reg,[reg+0]: same register, zero displacement, a memory form
            if (mod == 3 || reg != base || (mod == 0 && base == 5)) {
                return 0;
            }
            for (int k = 0; k < disp; k++) {
                if (b[len + k] != 0) {
                    return 0;
                }
            }
            return len + disp - i;
        }
        // mov reg,reg with the same register on both sides
        return mod == 3 && reg == rm ? len - i : 0;
    }

    /** A vtable found by name, with the target of each slot that points into code. */
    private static final class VTable {
        Symbol symbol;
        Address at;
        final List<Long> targets = new ArrayList<>();
        final List<String> otherNames = new ArrayList<>();
    }

    /**
     * Finds every vtable label (see NvCommon.isVtableName) and walks its consecutive
     * 4-byte slots while they point into executable memory. Two labels at one
     * address give one table, with the second name in "also_named".
     */
    private List<VTable> walkVtables() {
        List<Symbol> found = new ArrayList<>();
        Set<Long> starts = new HashSet<>();
        SymbolIterator si = symbols.getSymbolIterator("*vftable*", true);
        while (si.hasNext()) {
            Symbol s = si.next();
            if (s.getSymbolType() != SymbolType.LABEL || !s.getAddress().isMemoryAddress() ||
                !NvCommon.isVtableName(s.getName())) {
                continue;
            }
            found.add(s);
            starts.add(s.getAddress().getOffset());
        }
        found.sort((x, y) -> {
            int c = x.getAddress().compareTo(y.getAddress());
            return c != 0 ? c : x.getName(true).compareTo(y.getName(true));
        });

        List<VTable> tables = new ArrayList<>();
        for (Symbol s : found) {
            if (!tables.isEmpty() && tables.get(tables.size() - 1).at.equals(s.getAddress())) {
                tables.get(tables.size() - 1).otherNames.add(s.getName(true));
                continue;
            }
            VTable t = new VTable();
            t.symbol = s;
            t.at = s.getAddress();
            int slot = 0;
            while (slot < MAX_VTABLE_SLOTS) {
                Address slotAddr;
                try {
                    slotAddr = t.at.add(4L * slot);
                }
                catch (Exception e) {
                    break;
                }
                if (slot > 0 && starts.contains(slotAddr.getOffset())) {
                    break; // the next vtable begins here
                }
                long v = NvCommon.readU32(prog, slotAddr);
                if (v < 0) {
                    break;
                }
                if (!NvCommon.inExecutableMemory(prog, toAddr(v))) {
                    break;
                }
                t.targets.add(v);
                slot++;
            }
            tables.add(t);
        }
        return tables;
    }

    /** Lists the slots of each vtable whose target is not the entry of a defined function. */
    private Map<String, Object> vtableChecks(List<VTable> found) {
        List<Object> tables = new ArrayList<>();
        long totalSlots = 0;
        long totalBad = 0;
        for (VTable vt : found) {
            List<Object> bad = new ArrayList<>();
            for (int slot = 0; slot < vt.targets.size(); slot++) {
                Address slotAddr = vt.at.add(4L * slot);
                Address target = toAddr(vt.targets.get(slot));
                if (fm.getFunctionAt(target) == null) {
                    Map<String, Object> b = NvCommon.obj();
                    b.put("slot", slot);
                    b.put("slot_address", NvCommon.addr(slotAddr));
                    b.put("target", NvCommon.addr(target));
                    Function inside = fm.getFunctionContaining(target);
                    if (inside != null) {
                        b.put("problem", "inside_function");
                        b.put("function_entry", NvCommon.addr(inside.getEntryPoint()));
                    }
                    else if (listing.getInstructionAt(target) != null) {
                        b.put("problem", "code_without_function");
                    }
                    else {
                        b.put("problem", "undefined_code");
                    }
                    bad.add(b);
                }
            }
            totalSlots += vt.targets.size();
            totalBad += bad.size();
            Map<String, Object> t = NvCommon.obj();
            t.put("symbol", vt.symbol.getName(true));
            t.put("also_named", new ArrayList<Object>(vt.otherNames));
            t.put("address", NvCommon.addr(vt.at));
            t.put("slots", vt.targets.size());
            t.put("bad_slot_count", bad.size());
            t.put("bad_slots", bad);
            tables.add(t);
        }
        Map<String, Object> v = NvCommon.obj();
        v.put("symbol_rule", NvCommon.VTABLE_NAME_RULE);
        v.put("vtable_count", tables.size());
        v.put("slots_walked", totalSlots);
        v.put("bad_slots_total", totalBad);
        v.put("vtables", tables);
        return v;
    }
}

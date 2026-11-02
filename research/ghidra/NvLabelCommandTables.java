// Names the handler functions of a script-command table.
//
// Args: table=<hex> (count=<n> | end=<hex>) [stride=40] [prefix=Cmd_]
//       [dry=1] [force=0] report=<csv>
//
// Each table entry is one command. Layout for a 40-byte entry, an ABI fact
// documented by xNVSE's CommandInfo structure:
//   +0  long-name pointer        +4  short-name pointer
//   +8  opcode (u32)             +12 help-text pointer
//   +16 needs-parent (u16)       +18 parameter count (u16)
//   +20 parameter-list pointer   +24 execute function
//   +28 parse function           +32 eval function
//   +36 flags (u32)
//
// The stride is an argument because the notes in this repo disagree about
// the entry size: one note (docs/ENGINE_REFERENCE.md, console commands)
// says 48 bytes, while the public xNVSE structure is 40 bytes. Run the
// script with dry=1 and each stride, and compare how many entries have a
// valid name; the right stride gives valid names for every entry up to the
// end of the table. The field offsets above are used for any stride, so a
// stride other than 40 is only a probe, not a different layout.
//
// Validation: the long-name pointer must lead to a NUL-terminated printable
// ASCII string; an entry that fails is reported and skipped. For the
// execute, eval and parse pointers that are not null, the script creates a
// function when none exists there and, if the current name is a Ghidra
// default (SourceType.DEFAULT) or force=1, names it
// <prefix><LongName>_Execute, _Eval or _Parse. A pointer shared by more
// than one entry is not renamed (a shared stub has no single command name)
// and is reported. A pointer that one entry uses for two roles is named for
// the first role and keeps that name. Functions the script creates or
// renames get the tag "src:cmdtable"; a function it only recognizes (shared
// or already named as wanted) does not. When force=1 replaces a name that was
// not a Ghidra default, the function's old src: and pin: tags are removed
// first, so the tags say where the current name came from.
//
// A pointer is only turned into a function when code can be made there: an
// instruction exists at it or the bytes at it decode as one (checked with a
// pseudo-disassembly, which changes nothing), it is not inside another
// function and holds no defined data. Otherwise the pointer is reported as
// "not code" and skipped, in a dry run and a real run alike. Only the first
// instruction is checked. If Ghidra still cannot create a function after
// these checks, the script throws and its transaction is aborted.
//
// dry=1 changes nothing and reports what a real run on the same program
// would do: it keeps track of the functions it would create and the names it
// would give, so its counts and report match the real run. One thing it
// cannot know is that a function created earlier in the run may cover a
// later handler address.
//
// @category NV

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.app.util.PseudoDisassembler;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Data;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Program;
import ghidra.program.model.symbol.SourceType;

public class NvLabelCommandTables extends GhidraScript {
    private static final String ME = "NvLabelCommandTables";
    private static final String TAG = "src:cmdtable";
    private static final int MAX_NAME = 255;

    private static final class Entry {
        int index;
        Address at;
        String longName;
        String shortName = "";
        long opcode;
        int needsParent;
        int paramCount;
        long execute;
        long parse;
        long eval;
        long flags;
        String problem; // null when the entry is valid
        final List<String> notes = new ArrayList<>();
    }

    private Program prog;
    // Counts of what was done (or, in a dry run, what would be done).
    private int created;
    private int renamed;
    private int kept;
    private int shared;
    private int notCode;
    // What this run has already done, so a pointer or name is handled once.
    // The same code runs for a dry run, which changes nothing in the program
    // and so cannot rely on the program to remember.
    private final Set<Long> createdHere = new HashSet<>();
    private final Map<Long, String[]> namedHere = new HashMap<>(); // pointer -> {name, role}
    private final Set<String> namesTakenHere = new HashSet<>();
    private final Set<Long> sharedHere = new HashSet<>();
    private final Set<Long> notCodeHere = new HashSet<>();

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(ME, getScriptArgs(),
            Set.of("table", "count", "end", "stride", "prefix", "dry", "force", "report"));
        long tableOff = NvCommon.parseHex(ME, "table", NvCommon.require(ME, args, "table"));
        int stride = NvCommon.intArg(ME, args, "stride", 40, 40, 4096);
        String prefix = args.getOrDefault("prefix", "Cmd_");
        if (!NvCommon.identifierFrom(prefix).equals(prefix)) {
            throw new IllegalArgumentException(ME + ": prefix may only use letters, digits and '_'");
        }
        boolean dry = NvCommon.flag(ME, args, "dry", false);
        boolean force = NvCommon.flag(ME, args, "force", false);
        Path report = Paths.get(NvCommon.require(ME, args, "report"));
        if (args.containsKey("count") == args.containsKey("end")) {
            throw new IllegalArgumentException(ME + ": give exactly one of count=<n> or end=<hex>");
        }
        int count;
        if (args.containsKey("count")) {
            count = NvCommon.intArg(ME, args, "count", 0, 1, 100000);
        }
        else {
            long end = NvCommon.parseHex(ME, "end", args.get("end"));
            long span = end - tableOff;
            if (span <= 0 || span % stride != 0) {
                throw new IllegalArgumentException(ME + ": end (exclusive) minus table is " + span +
                    ", which is not a positive multiple of stride " + stride);
            }
            if (span / stride > 100000) {
                throw new IllegalArgumentException(ME + ": more than 100000 entries");
            }
            count = (int) (span / stride);
        }
        if (currentProgram == null) {
            throw new IllegalStateException(ME + ": no program is open");
        }
        prog = currentProgram;
        Address table = toAddr(tableOff);
        if (!NvCommon.inInitializedMemory(prog, table)) {
            throw new IllegalArgumentException(ME + ": table address " + NvCommon.addr(table) +
                " is not in initialized memory");
        }
        if (stride != 40) {
            println(ME + ": WARNING stride is " + stride + "; the field offsets are still those of the " +
                "40-byte layout, so treat this run as a probe.");
        }

        // ---- read and validate the entries ----
        List<Entry> entries = new ArrayList<>();
        for (int i = 0; i < count; i++) {
            entries.add(readEntry(i, table.add((long) i * stride)));
        }
        int valid = 0;
        for (Entry e : entries) {
            if (e.problem == null) {
                valid++;
            }
        }

        // How many valid entries use each code pointer?
        Map<Long, Set<Integer>> users = new HashMap<>();
        for (Entry e : entries) {
            if (e.problem != null) {
                continue;
            }
            for (long p : new long[] { e.execute, e.parse, e.eval }) {
                if (p != 0) {
                    users.computeIfAbsent(p, k -> new LinkedHashSet<>()).add(e.index);
                }
            }
        }

        // ---- act ----
        created = 0;
        renamed = 0;
        kept = 0;
        shared = 0;
        notCode = 0;
        List<String> lines = new ArrayList<>();
        lines.add(NvCommon.csvLine(List.of("index", "address", "opcode", "long", "short", "params",
            "execute", "parse", "eval", "flags", "action")));
        Files.deleteIfExists(report);
        // Own transaction: if anything throws it is aborted, and Ghidra then
        // discards all changes this script run made (no half-labeled table).
        int tx = prog.startTransaction(ME);
        boolean ok = false;
        try {
            processEntries(entries, users, prefix, dry, force, lines);
            ok = true;
        }
        finally {
            prog.endTransaction(tx, ok);
        }
        NvCommon.writeText(report, String.join("\n", lines) + "\n");

        println(ME + ": table " + NvCommon.addr(table) + " stride " + stride + ": " + count +
            " entries, " + valid + " valid, " + (count - valid) + " skipped");
        if (dry) {
            println(ME + ": dry run, nothing changed. Would create " + created + " functions, rename " +
                renamed + "; names kept " + kept + ", shared pointers " + shared + ", not code " + notCode +
                ". Report: " + report);
        }
        else {
            println(ME + ": functions created " + created + ", renamed " + renamed + ", names kept " +
                kept + ", shared pointers " + shared + ", not code " + notCode + ". Report: " + report);
        }
    }

    private void processEntries(List<Entry> entries, Map<Long, Set<Integer>> users, String prefix,
            boolean dry, boolean force, List<String> lines) throws Exception {
        for (Entry e : entries) {
            List<String> actions = new ArrayList<>();
            if (e.problem != null) {
                actions.add("skipped: " + e.problem);
            }
            else {
                String ident = NvCommon.identifierFrom(e.longName);
                actions.add(handle("execute", e.execute, prefix + ident + "_Execute", users, dry, force));
                actions.add(handle("parse", e.parse, prefix + ident + "_Parse", users, dry, force));
                actions.add(handle("eval", e.eval, prefix + ident + "_Eval", users, dry, force));
                actions.removeIf(String::isEmpty);
                actions.addAll(e.notes);
                if (actions.isEmpty()) {
                    actions.add("nothing to do (no handler pointers)");
                }
            }
            lines.add(NvCommon.csvLine(List.of(Integer.toString(e.index), NvCommon.addr(e.at),
                String.format("0x%04x", e.opcode), e.longName == null ? "" : e.longName, e.shortName,
                Integer.toString(e.paramCount), ptr(e.execute), ptr(e.parse), ptr(e.eval),
                String.format("0x%x", e.flags), String.join("; ", actions))));
        }
    }

    private static String ptr(long p) {
        return p == 0 ? "" : NvCommon.hex8(p);
    }

    private Entry readEntry(int index, Address at) {
        Entry e = new Entry();
        e.index = index;
        e.at = at;
        byte[] raw = NvCommon.readBytes(prog, at, 40);
        if (raw.length < 40) {
            e.problem = "entry runs past initialized memory";
            return e;
        }
        long namePtr = NvCommon.le(raw, 0, 4);
        long shortPtr = NvCommon.le(raw, 4, 4);
        e.opcode = NvCommon.le(raw, 8, 4);
        e.needsParent = (int) NvCommon.le(raw, 16, 2);
        e.paramCount = (int) NvCommon.le(raw, 18, 2);
        e.execute = NvCommon.le(raw, 24, 4);
        e.parse = NvCommon.le(raw, 28, 4);
        e.eval = NvCommon.le(raw, 32, 4);
        e.flags = NvCommon.le(raw, 36, 4);

        if (namePtr == 0) {
            e.problem = "long-name pointer is null";
            return e;
        }
        String name = NvCommon.readAsciiZ(prog, toAddr(namePtr), 1, MAX_NAME);
        if (name == null) {
            e.problem = "long-name pointer " + NvCommon.hex8(namePtr) +
                " does not lead to a NUL-terminated printable ASCII string";
            return e;
        }
        e.longName = name;
        if (shortPtr != 0) {
            String s = NvCommon.readAsciiZ(prog, toAddr(shortPtr), 1, MAX_NAME);
            if (s == null) {
                e.notes.add("short-name pointer " + NvCommon.hex8(shortPtr) + " is not a valid string");
            }
            else {
                e.shortName = s;
            }
        }
        for (String[] role : new String[][] { { "execute" }, { "parse" }, { "eval" } }) {
            long p = role[0].equals("execute") ? e.execute : role[0].equals("parse") ? e.parse : e.eval;
            if (p != 0 && !NvCommon.inExecutableMemory(prog, toAddr(p))) {
                e.notes.add(role[0] + " pointer " + NvCommon.hex8(p) + " is not in executable memory");
                if (role[0].equals("execute")) {
                    e.execute = 0;
                }
                else if (role[0].equals("parse")) {
                    e.parse = 0;
                }
                else {
                    e.eval = 0;
                }
            }
        }
        return e;
    }

    /** Creates and names the function behind one handler pointer; returns a short action text. */
    private String handle(String role, long p, String wanted, Map<Long, Set<Integer>> users,
            boolean dry, boolean force) throws Exception {
        if (p == 0) {
            return "";
        }
        Address a = toAddr(p);
        Function fn = getFunctionAt(a);
        int sharedBy = users.get(p).size();
        if (fn == null) {
            String why = notCodeReason(a);
            if (why != null) {
                if (notCodeHere.add(p)) {
                    notCode++;
                }
                return role + " " + NvCommon.hex8(p) + ": not code (" + why + "): skipped";
            }
        }
        // A pointer used again later in this run (by another entry or another
        // role) is created once. A dry run never creates, so it remembers.
        boolean willCreate = fn == null && createdHere.add(p);
        if (willCreate && !dry) {
            if (getInstructionAt(a) == null) {
                disassemble(a);
            }
            fn = createFunction(a, null);
            if (fn == null) {
                // The checks above passed, so this is unexpected. Abort rather than leave a
                // partly labeled table: the transaction around this script is rolled back.
                throw new IllegalStateException(ME + ": Ghidra could not create a function at " +
                    NvCommon.hex8(p) + " (" + role + " pointer); nothing was changed");
            }
        }
        StringBuilder act = new StringBuilder(role).append(' ').append(NvCommon.hex8(p)).append(": ");
        if (willCreate) {
            act.append(dry ? "would create function, " : "created function, ");
            created++;
        }
        if (sharedBy > 1) {
            if (sharedHere.add(p)) {
                shared++;
            }
            act.append("shared by ").append(sharedBy).append(" entries, not renamed");
            if (!dry && willCreate) {
                fn.addTag(TAG);
            }
            return act.toString();
        }
        String[] already = namedHere.get(p);
        if (already != null) {
            act.append("also used as ").append(already[1]).append(", keeps ").append(already[0]);
            return act.toString();
        }
        String current = fn == null ? "(new)" : fn.getName();
        boolean isDefault = fn == null || fn.getSymbol().getSource() == SourceType.DEFAULT;
        if (fn != null && current.equals(wanted)) {
            namedHere.put(p, new String[] { wanted, role });
            namesTakenHere.add(wanted);
            act.append("already named ").append(wanted);
            return act.toString();
        }
        if (!isDefault && !force) {
            kept++;
            namedHere.put(p, new String[] { current, role });
            act.append("kept existing name ").append(current);
            return act.toString();
        }
        String finalName = wanted;
        if (namesTakenHere.contains(wanted) || nameInUse(wanted, a)) {
            finalName = wanted + "_" + NvCommon.hex8(p);
        }
        namedHere.put(p, new String[] { finalName, role });
        namesTakenHere.add(finalName);
        renamed++;
        if (dry) {
            act.append("would rename to ").append(finalName);
            return act.toString();
        }
        if (!isDefault) {
            NvCommon.clearProvenanceTags(fn);
        }
        fn.setName(finalName, SourceType.USER_DEFINED);
        fn.addTag(TAG);
        act.append("renamed to ").append(finalName);
        if (!finalName.equals(wanted)) {
            act.append(" (").append(wanted).append(" was taken)");
        }
        return act.toString();
    }

    /**
     * Why code cannot be made at a, or null when it can: an instruction is already
     * there, or the bytes decode as one. The pseudo-disassembly reads the program and
     * changes nothing, so a dry run and a real run decide alike.
     */
    private String notCodeReason(Address a) {
        Function in = getFunctionContaining(a);
        if (in != null) {
            return "inside function " + in.getName() + " at " + NvCommon.addr(in.getEntryPoint());
        }
        if (getInstructionAt(a) != null) {
            return null;
        }
        Data d = getDataAt(a);
        if (d != null && d.isDefined()) {
            return "defined data " + d.getDataType().getName();
        }
        try {
            if (new PseudoDisassembler(prog).disassemble(a) != null) {
                return null;
            }
        }
        catch (Exception e) {
            // InsufficientBytes, UnknownInstruction or UnknownContext: no valid instruction here
        }
        return "the bytes do not decode as an instruction";
    }

    /** True when another function in the global namespace already has this name. */
    private boolean nameInUse(String name, Address except) {
        for (ghidra.program.model.symbol.Symbol s : prog.getSymbolTable().getGlobalSymbols(name)) {
            if (!s.getAddress().equals(except)) {
                return true;
            }
        }
        return false;
    }
}

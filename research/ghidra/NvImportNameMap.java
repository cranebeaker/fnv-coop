// Imports names from a CSV into a Ghidra program, with provenance.
//
// Args: csv=<file> [rva=0|1] [dry=1] [force=0] report=<csv>
//
// CSV header: address,name,source,pin[,kind]
//   address  hex virtual address (or RVA with rva=1; the image base is added)
//   name     the name, optionally qualified: Class::Method, Foo<int>::Bar
//   source   where the name came from: rtti, xnvse, jip, jg, bgs, llm, own, ...
//            (letters, digits, '_', '.', '-'; at most 32 characters)
//   pin      commit hash (or other version stamp) of that source
//   kind     function or label. If empty: function when a function exists or
//            can be created at the address (code is there or its bytes decode as
//            an instruction), otherwise label. An address inside a function, not
//            at its entry, is a conflict until kind=label is given.
// Lines whose first cell starts with '#' are comments.
//
// Name sanitization: surrounding whitespace is trimmed and runs of
// whitespace inside become one '_'. After that the name may only use
// [A-Za-z0-9_:~<>]. Any other character rejects the row. "::" separates
// namespaces (a "::" inside <...> does not).
//
// Rules:
//  - A name that is not a Ghidra default is never overwritten unless
//    force=1; the row is reported as a conflict and left alone.
//  - Duplicate names are refused. Ghidra itself would accept two functions
//    with the same name, and a lookup by name would then be ambiguous.
//    A name that two rows give to different addresses rejects those rows.
//    A name that the program already uses at another address (same
//    namespace) is a conflict and the row is left alone. force=1 allows
//    both; the program then has two symbols with that name. One address
//    given two different names in the file is always rejected.
//  - Functions get the tags "src:<source>" and "pin:<first 12 chars of pin>".
//    When force=1 replaces a name that was not a Ghidra default, the old
//    src: and pin: tags are removed first, so the tags say where the current
//    name came from. Several rows that agree on a name add their tags up.
//  - Labels get an Info bookmark in category "nv-names" holding source and
//    pin. Rows that agree on a label's name add their source and pin to it.
//  - Every row is checked before anything changes: first its syntax
//    (address, name, source, pin, kind) and the rows against each other,
//    then a simulated run of the whole file (which finds, for example, a
//    function that cannot be created at its address). If any row fails, the
//    report is written, the script throws and nothing is applied; the other
//    rows are marked "not-applied". As a last safety net the real run is
//    wrapped in its own transaction, which is aborted if anything still goes
//    wrong; Ghidra then discards all of the script's changes. Conflicts
//    (an existing non-default name, a name used elsewhere) are not errors.
//  - dry=1 reports what would happen and changes nothing.
//
// Run it on a copy of the project. It changes the program.
//
// @category NV

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;
import java.util.regex.Pattern;

import ghidra.app.script.GhidraScript;
import ghidra.app.util.PseudoDisassembler;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Bookmark;
import ghidra.program.model.listing.BookmarkType;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Program;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.Namespace;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.program.model.symbol.SymbolType;

public class NvImportNameMap extends GhidraScript {
    private static final String ME = "NvImportNameMap";
    private static final String BOOKMARK_CATEGORY = NvCommon.NV_NAMES_BOOKMARK;
    private static final Pattern SOURCE_OK = Pattern.compile("[A-Za-z0-9_.-]{1,32}");
    private static final Pattern PIN_OK = Pattern.compile("[A-Za-z0-9_.-]+");

    private Program prog;
    private SymbolTable symbols;
    private boolean dry;
    private boolean force;
    private int applied;
    private int unchanged;
    private int conflicts;
    private int createdFunctions;
    private int rejected;

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(ME, getScriptArgs(),
            Set.of("csv", "rva", "dry", "force", "report"));
        Path csv = Paths.get(NvCommon.require(ME, args, "csv"));
        boolean rva = NvCommon.flag(ME, args, "rva", false);
        dry = NvCommon.flag(ME, args, "dry", false);
        force = NvCommon.flag(ME, args, "force", false);
        Path report = Paths.get(NvCommon.require(ME, args, "report"));
        Files.deleteIfExists(report); // never leave a stale report from an earlier run
        if (currentProgram == null) {
            throw new IllegalStateException(ME + ": no program is open");
        }
        prog = currentProgram;
        symbols = prog.getSymbolTable();

        String text = new String(Files.readAllBytes(csv), StandardCharsets.UTF_8);
        if (text.startsWith("\uFEFF")) {
            text = text.substring(1); // Windows PowerShell writes a byte-order mark
        }
        List<List<String>> rows = NvCommon.parseCsv(text);
        if (rows.isEmpty()) {
            throw new IllegalArgumentException(ME + ": " + csv + " is empty");
        }
        List<String> header = new ArrayList<>();
        for (String h : rows.get(0)) {
            header.add(h.trim().toLowerCase());
        }
        for (String h : header) {
            if (!List.of("address", "name", "source", "pin", "kind").contains(h)) {
                throw new IllegalArgumentException(ME + ": unknown CSV column '" + h +
                    "'. Header must be address,name,source,pin[,kind]");
            }
        }
        int cAddr = header.indexOf("address");
        int cName = header.indexOf("name");
        int cSource = header.indexOf("source");
        int cPin = header.indexOf("pin");
        int cKind = header.indexOf("kind");
        if (cAddr < 0 || cName < 0 || cSource < 0 || cPin < 0) {
            throw new IllegalArgumentException(
                ME + ": header must contain address,name,source,pin (and optionally kind)");
        }

        // ---- phase 1: validate every row, change nothing ----
        List<Row> parsed = new ArrayList<>();
        for (int i = 1; i < rows.size(); i++) {
            List<String> r = rows.get(i);
            if (!r.isEmpty() && r.get(0).trim().startsWith("#")) {
                continue;
            }
            parsed.add(validate(parsed.size() + 1, rva, cell(r, cAddr), cell(r, cName),
                cell(r, cSource).trim(), cell(r, cPin).trim(), cell(r, cKind).trim().toLowerCase()));
        }
        checkRowsAgainstEachOther(parsed);
        for (Row row : parsed) {
            if (row.error != null) {
                rejected++;
            }
        }

        // ---- phase 2: simulate the whole file, change nothing ----
        boolean realDry = dry;
        if (rejected == 0) {
            dry = true;
            applyAll(parsed);
            dry = realDry;
        }
        // ---- phase 3: apply for real, only if everything above passed ----
        if (rejected == 0 && !dry) {
            resetCounters();
            int tx = prog.startTransaction(ME);
            boolean ok = false;
            try {
                applyAll(parsed);
                ok = rejected == 0;
            }
            finally {
                // Aborting a nested transaction makes Ghidra discard everything
                // this script run changed.
                prog.endTransaction(tx, ok);
            }
        }
        for (Row row : parsed) {
            if (row.error != null) {
                row.result = new String[] { row.addrOut, "", "", "rejected", row.error };
            }
            else if (rejected > 0 && (row.result == null || !row.result[3].equals("rejected"))) {
                row.result = new String[] { row.addrOut, "", "", "not-applied",
                    "other rows were rejected, so nothing was applied" };
            }
        }

        List<String> out = new ArrayList<>();
        out.add(NvCommon.csvLine(List.of("row", "address", "requested_name", "applied_name", "kind",
            "source", "pin", "action", "detail")));
        for (Row row : parsed) {
            out.add(NvCommon.csvLine(List.of(Integer.toString(row.no), row.result[0], row.rawName.trim(),
                row.result[1], row.result[2], row.source, row.pin, row.result[3], row.result[4])));
        }
        NvCommon.writeText(report, String.join("\n", out) + "\n");

        if (rejected > 0) {
            println(ME + ": " + parsed.size() + " rows, " + rejected + " rejected. Nothing is applied. " +
                "Report: " + report);
        }
        else {
            println(ME + ": " + parsed.size() + " rows" + (dry ? " (dry run, nothing changed)" : "") +
                ": " + (dry ? "would apply " : "applied ") + applied + ", unchanged " + unchanged +
                ", conflicts " + conflicts + ", functions " + (dry ? "to create " : "created ") +
                createdFunctions + ". Report: " + report);
        }
        if (rejected > 0) {
            throw new IllegalArgumentException(ME + ": " + rejected +
                " row(s) rejected, so nothing was applied. See " + report);
        }
    }

    /** One CSV row after validation. */
    private static final class Row {
        int no;
        String rawName;
        String source;
        String pin;
        String addrOut = "";
        Address addr;
        String name;
        String shortName;
        List<String> nsPath;
        String kind; // "", "function" or "label"
        String error; // null when valid
        String[] result; // address, applied name, kind, action, detail
    }

    private static String cell(List<String> row, int idx) {
        return idx >= 0 && idx < row.size() ? row.get(idx) : "";
    }

    private void resetCounters() {
        applied = 0;
        unchanged = 0;
        conflicts = 0;
        createdFunctions = 0;
    }

    /** Applies every row (or, when dry is set, only reports what would happen). */
    private void applyAll(List<Row> rows) throws Exception {
        for (Row row : rows) {
            if (row.error == null) {
                row.result = apply(row);
                if (row.result[3].equals("rejected")) {
                    row.error = row.result[4];
                    return; // stop at the first row that cannot be applied
                }
            }
        }
    }

    /**
     * Cross-row checks that need the whole file. A simulated run cannot find
     * these, because a dry run does not change the program, so a later row
     * would never see an earlier row's name.
     */
    private void checkRowsAgainstEachOther(List<Row> parsed) {
        Map<String, List<Row>> byName = new LinkedHashMap<>();
        Map<Long, List<Row>> byAddress = new LinkedHashMap<>();
        for (Row row : parsed) {
            if (row.error == null) {
                byName.computeIfAbsent(row.name, k -> new ArrayList<>()).add(row);
                byAddress.computeIfAbsent(row.addr.getOffset(), k -> new ArrayList<>()).add(row);
            }
        }
        for (List<Row> group : byAddress.values()) {
            TreeSet<String> names = new TreeSet<>();
            for (Row r : group) {
                names.add(r.name);
            }
            if (names.size() > 1) {
                for (Row r : group) {
                    r.error = "address " + r.addrOut + " is given different names in rows " + rowNumbers(group);
                }
            }
        }
        if (force) {
            return;
        }
        for (List<Row> group : byName.values()) {
            TreeSet<Long> addresses = new TreeSet<>();
            for (Row r : group) {
                addresses.add(r.addr.getOffset());
            }
            if (addresses.size() > 1) {
                for (Row r : group) {
                    if (r.error == null) {
                        r.error = "name '" + r.name + "' is given to " + addresses.size() +
                            " different addresses in rows " + rowNumbers(group) + " (force=1 allows it)";
                    }
                }
            }
        }
    }

    private static String rowNumbers(List<Row> rows) {
        List<String> n = new ArrayList<>();
        for (Row r : rows) {
            n.add(Integer.toString(r.no));
        }
        return String.join(", ", n);
    }

    private String[] reject(String address, String why) {
        rejected++;
        return new String[] { address, "", "", "rejected", why };
    }

    private Row validate(int no, boolean rva, String addrText, String rawName, String source, String pin,
            String kindText) {
        Row row = new Row();
        row.no = no;
        row.rawName = rawName;
        row.source = source;
        row.pin = pin;
        row.addrOut = addrText.trim();
        long off;
        try {
            off = NvCommon.parseHex(ME, "address", addrText);
        }
        catch (IllegalArgumentException e) {
            row.error = "bad hex address";
            return row;
        }
        if (rva) {
            off += prog.getImageBase().getOffset();
        }
        row.addr = toAddr(off);
        row.addrOut = NvCommon.addr(row.addr);
        if (!prog.getMemory().contains(row.addr)) {
            String hint = !rva && prog.getMemory().contains(toAddr(off + prog.getImageBase().getOffset()))
                ? " (looks like an RVA: use rva=1)" : "";
            row.error = "address is not in the program's memory" + hint;
            return row;
        }
        String[] why = new String[1];
        row.name = NvCommon.sanitizeQualifiedName(rawName, why);
        if (row.name == null) {
            row.error = "name '" + rawName.trim() + "': " + why[0];
            return row;
        }
        if (!SOURCE_OK.matcher(source).matches()) {
            row.error = "source '" + source + "' must be 1-32 characters of [A-Za-z0-9_.-]";
            return row;
        }
        if (row.pin.isEmpty()) {
            row.pin = "none";
        }
        if (!PIN_OK.matcher(row.pin).matches()) {
            row.error = "pin '" + row.pin + "' must use only [A-Za-z0-9_.-]";
            return row;
        }
        if (!kindText.isEmpty() && !kindText.equals("function") && !kindText.equals("label")) {
            row.error = "kind '" + kindText + "' must be function or label";
            return row;
        }
        row.kind = kindText;
        List<String> parts = NvCommon.splitQualified(row.name);
        row.shortName = parts.get(parts.size() - 1);
        row.nsPath = parts.subList(0, parts.size() - 1);
        return row;
    }

    private String[] apply(Row row) throws Exception {
        Address addr = row.addr;
        String addrOut = row.addrOut;
        Function existing = getFunctionAt(addr);
        String kind = row.kind;
        if (kind.isEmpty()) {
            // An address inside a function but not at its entry is most likely a source address
            // that is slightly off, or a function Ghidra merged with another one. Turning it into
            // a label would hide the name from the export, so the row needs a decision.
            Function inside = existing == null ? getFunctionContaining(addr) : null;
            if (inside != null) {
                conflicts++;
                return new String[] { addrOut, "", "", "conflict", "address is inside function " +
                    inside.getName(true) + " at " + NvCommon.addr(inside.getEntryPoint()) +
                    ", not at its entry; use the entry address, or kind=label for a label inside it" };
            }
            kind = existing != null || canCreateFunction(addr) ? "function" : "label";
        }
        if (kind.equals("label") && existing != null) {
            return reject(addrOut, "a function exists at this address; use kind=function");
        }
        if (kind.equals("function")) {
            return applyFunction(addr, addrOut, existing, row.name, row.shortName, row.nsPath, row.source,
                row.pin);
        }
        return applyLabel(addr, addrOut, row.name, row.shortName, row.nsPath, row.source, row.pin);
    }

    /**
     * True when a function could be created at addr: executable memory, not inside
     * another function, no defined data, and an instruction there or bytes that decode
     * as one (a pseudo-disassembly, which changes nothing, so a dry run decides like
     * a real run). Only the first instruction is checked.
     */
    private boolean canCreateFunction(Address addr) {
        MemoryBlock b = prog.getMemory().getBlock(addr);
        if (b == null || !b.isExecute() || !b.isInitialized()) {
            return false;
        }
        Function in = getFunctionContaining(addr);
        if (in != null) {
            return false;
        }
        if (getDataAt(addr) != null && getDataAt(addr).isDefined()) {
            return false;
        }
        if (getInstructionAt(addr) != null) {
            return true;
        }
        try {
            return new PseudoDisassembler(prog).disassemble(addr) != null;
        }
        catch (Exception e) {
            return false; // InsufficientBytes, UnknownInstruction or UnknownContext
        }
    }

    private Namespace findNamespace(List<String> path, boolean create) throws Exception {
        Namespace ns = prog.getGlobalNamespace();
        for (String part : path) {
            Namespace next = symbols.getNamespace(part, ns);
            if (next == null) {
                if (!create) {
                    return null;
                }
                next = symbols.getOrCreateNameSpace(ns, part, SourceType.USER_DEFINED);
            }
            ns = next;
        }
        return ns;
    }

    private String[] applyFunction(Address addr, String addrOut, Function fn, String name,
            String shortName, List<String> nsPath, String source, String pin) throws Exception {
        boolean create = fn == null;
        if (create && !canCreateFunction(addr)) {
            return reject(addrOut, "cannot create a function here (not executable memory, inside another " +
                "function, defined data, or bytes that do not decode as an instruction)");
        }
        String detail = create ? (dry ? "would create function" : "created function") : "";
        boolean replacesUserName = false;
        if (!create) {
            String current = fn.getName(true);
            boolean isDefault = fn.getSymbol().getSource() == SourceType.DEFAULT;
            if (current.equals(name)) {
                unchanged++;
                if (!dry) {
                    tagFunction(fn, source, pin);
                }
                return new String[] { addrOut, current, "function", "unchanged", "name already set; tags added" };
            }
            if (!isDefault && !force) {
                conflicts++;
                return new String[] { addrOut, current, "function", "conflict",
                    "existing " + fn.getSymbol().getSource() + " name kept; use force=1 to replace" };
            }
            replacesUserName = !isDefault;
            detail = isDefault ? "default name replaced" : "name replaced (force)";
        }
        String other = nameUsedElsewhere(nsPath, shortName, addr);
        if (other != null) {
            if (!force) {
                conflicts++;
                return new String[] { addrOut, create ? "" : fn.getName(true), "function", "conflict",
                    "name already used at " + other + "; use force=1 to allow two symbols with one name" };
            }
            detail += (detail.isEmpty() ? "" : "; ") + "duplicate of the name at " + other + " (force)";
        }
        if (create) {
            if (!dry) {
                if (getInstructionAt(addr) == null) {
                    disassemble(addr);
                }
                fn = createFunction(addr, null);
                if (fn == null) {
                    return reject(addrOut, "Ghidra could not create a function here");
                }
            }
            createdFunctions++;
        }
        if (dry) {
            applied++;
            return new String[] { addrOut, name, "function", "would-apply", detail };
        }
        Namespace ns = findNamespace(nsPath, true);
        fn.getSymbol().setNameAndNamespace(shortName, ns, SourceType.USER_DEFINED);
        if (replacesUserName) {
            NvCommon.clearProvenanceTags(fn);
        }
        tagFunction(fn, source, pin);
        applied++;
        return new String[] { addrOut, fn.getName(true), "function", "applied", detail };
    }

    /**
     * The address of another function or label that already has this name in
     * the same namespace, or null. Ghidra allows such duplicates; this script
     * does not create them without force=1.
     */
    private String nameUsedElsewhere(List<String> nsPath, String shortName, Address except)
            throws Exception {
        Namespace ns = findNamespace(nsPath, false);
        if (ns == null) {
            return null;
        }
        for (Symbol s : symbols.getSymbols(shortName, ns)) {
            SymbolType t = s.getSymbolType();
            if ((t == SymbolType.FUNCTION || t == SymbolType.LABEL) && s.getAddress().isMemoryAddress() &&
                !s.getAddress().equals(except)) {
                return NvCommon.addr(s.getAddress());
            }
        }
        return null;
    }

    private void tagFunction(Function fn, String source, String pin) {
        fn.addTag("src:" + source);
        fn.addTag("pin:" + pin.substring(0, Math.min(12, pin.length())));
    }

    private String[] applyLabel(Address addr, String addrOut, String name, String shortName,
            List<String> nsPath, String source, String pin) throws Exception {
        Symbol primary = symbols.getPrimarySymbol(addr);
        boolean isDefault = primary == null || primary.getSource() == SourceType.DEFAULT;
        String detail = "";
        if (!isDefault) {
            String current = primary.getName(true);
            if (current.equals(name)) {
                unchanged++;
                if (!dry) {
                    bookmark(addr, source, pin, true);
                }
                return new String[] { addrOut, current, "label", "unchanged", "name already set; bookmark updated" };
            }
            if (!force) {
                conflicts++;
                return new String[] { addrOut, current, "label", "conflict",
                    "existing " + primary.getSource() + " name kept; use force=1 to replace" };
            }
            detail = "name replaced (force)";
        }
        String other = nameUsedElsewhere(nsPath, shortName, addr);
        if (other != null) {
            if (!force) {
                conflicts++;
                return new String[] { addrOut, isDefault ? "" : primary.getName(true), "label", "conflict",
                    "name already used at " + other + "; use force=1 to allow two symbols with one name" };
            }
            detail += (detail.isEmpty() ? "" : "; ") + "duplicate of the name at " + other + " (force)";
        }
        if (dry) {
            applied++;
            return new String[] { addrOut, name, "label", "would-apply", detail };
        }
        Namespace ns = findNamespace(nsPath, true);
        createLabel(addr, shortName, ns, true, SourceType.USER_DEFINED);
        bookmark(addr, source, pin, false);
        applied++;
        return new String[] { addrOut, name, "label", "applied", detail };
    }

    /**
     * An Info bookmark in category nv-names holding "src=<source> pin=<pin>".
     * It replaces an earlier nv-names Info bookmark at the address. With
     * keepEarlier (the name was already set, so another source agrees with
     * it) the new source and pin are added to the existing text instead.
     */
    private void bookmark(Address addr, String source, String pin, boolean keepEarlier) {
        String text = "src=" + source + " pin=" + pin;
        if (keepEarlier) {
            Bookmark old = prog.getBookmarkManager().getBookmark(addr, BookmarkType.INFO, BOOKMARK_CATEGORY);
            if (old != null && !old.getComment().isEmpty()) {
                List<String> parts = List.of(old.getComment().split("; "));
                if (parts.contains(text)) {
                    return;
                }
                text = old.getComment() + "; " + text;
            }
        }
        prog.getBookmarkManager().setBookmark(addr, BookmarkType.INFO, BOOKMARK_CATEGORY, text);
    }
}

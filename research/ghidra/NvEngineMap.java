// Engine-map export: the raw PC facts the function ledger is built from.
//
// Args: out=<dir>
//
// Writes, for the program as it is (read-only; run with -readOnly):
//
//   <dir>/functions.tsv  one row per function, sorted by entry:
//       entry, size (bytes in the body), name (qualified), name_source
//       (Ghidra SourceType), tags (src:/pin: provenance, ';'-separated),
//       thunk_target, calls, imports, strings
//     calls   the entries of the functions this one reaches, in the order of
//             the call sites (direct CALLs, and unconditional JMPs to the
//             entry of another function, i.e. tail calls), ','-separated,
//             repeats kept. A call through an import slot is not here but in
//             imports.
//     imports the imported routines it calls (library::name), in call order.
//     strings the strings it references: every reference to non-executable
//             memory that holds a NUL-terminated printable ASCII string of 4
//             or more characters (Ghidra's own string definitions are not
//             used, so the Xbox map can apply the same rule), in reference
//             order, each cut to 60 characters with tab, newline, ',', '|'
//             and control characters replaced by '_'; '|'-separated.
//     callers   distinct functions that call the entry.
//     code_refs other references to the entry from instructions (a pointer
//             pushed or moved, e.g. a destructor handed to atexit).
//     data_refs references to the entry from data (vtables, init tables).
//   <dir>/vtables.tsv   one row per virtual function table that MSVC RTTI
//       describes: vtable (address of slot 0), col (its complete object
//       locator), type_name (the mangled TypeDescriptor name, '.?AV...'),
//       offset (the COL's subobject offset), slots (slot targets, ','),
//       slot_is_function (1/0 per slot: whether a function starts there)
//   <dir>/manifest.txt  counts and the gate.
//
// The RTTI walk does not depend on Ghidra's RTTI analyzer (the project may
// not have run it). An MSVC x86 CompleteObjectLocator is
// {u32 signature = 0, u32 offset, u32 cdOffset, TypeDescriptor*,
// ClassHierarchyDescriptor*}; a TypeDescriptor is {vftable*, spare, char
// name[]} with a name that starts ".?A". A vtable is the address after an
// aligned pointer to a COL in read-only data; its slots run while they
// point into executable memory and stop at the next COL pointer.
//
// Gate: rows written == getFunctionCount() minus external functions,
// otherwise the script throws and leaves manifest.failed.txt.
//
// The output describes a program you own. It holds strings from the
// executable, so keep it in the private research tree. Never commit it.
//
// @category NV

import java.io.BufferedWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.mem.Memory;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.ExternalLocation;
import ghidra.program.model.symbol.FlowType;
import ghidra.program.model.symbol.Reference;
import ghidra.program.model.symbol.Symbol;

public class NvEngineMap extends GhidraScript {
    private static final String NAME = "NvEngineMap";

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(NAME, getScriptArgs(), Set.of("out"));
        Path out = Paths.get(NvCommon.require(NAME, args, "out"));
        Files.createDirectories(out);
        Files.deleteIfExists(out.resolve("manifest.txt"));
        Files.deleteIfExists(out.resolve("manifest.failed.txt"));

        FunctionManager fm = currentProgram.getFunctionManager();
        Listing listing = currentProgram.getListing();
        int externals = 0;
        for (Function f : fm.getExternalFunctions()) {
            externals++;
        }
        int expected = fm.getFunctionCount() - externals;

        int rows = 0;
        try (BufferedWriter w = Files.newBufferedWriter(out.resolve("functions.tsv"), StandardCharsets.UTF_8)) {
            w.write("entry\tsize\tname\tname_source\ttags\tthunk_target\tcalls\timports\tstrings\tcallers\tcode_refs\tdata_refs\n");
            for (Function f : fm.getFunctions(true)) {
                monitor.checkCancelled();
                AddressSetView body = f.getBody();
                List<String> calls = new ArrayList<>();
                List<String> imports = new ArrayList<>();
                List<String> strings = new ArrayList<>();
                for (Instruction ins : listing.getInstructions(body, true)) {
                    FlowType ft = ins.getFlowType();
                    boolean call = ft.isCall();
                    boolean jump = ft.isJump() && !ft.isConditional();
                    if (call || jump) {
                        for (Address t : ins.getFlows()) {
                            Function g = fm.getFunctionAt(t);
                            if (g == null || (jump && body.contains(t))) {
                                continue;
                            }
                            if (g.isExternal()) {
                                imports.add(extName(g));
                            }
                            else {
                                calls.add(NvCommon.addr(g.getEntryPoint()));
                            }
                        }
                    }
                    for (Reference r : ins.getReferencesFrom()) {
                        Address to = r.getToAddress();
                        if (r.isExternalReference() && (call || jump)) {
                            Symbol s = currentProgram.getSymbolTable().getPrimarySymbol(to);
                            Function g = s == null ? null : fm.getFunctionAt(to);
                            imports.add(g != null ? extName(g) : (s == null ? "?" : s.getName(true)));
                            continue;
                        }
                        if (!to.isMemoryAddress()) {
                            continue;
                        }
                        if ((call || jump) && r.getReferenceType().isData()) {
                            // call/jmp dword ptr [slot]: the slot may hold an import.
                            for (Reference r2 : getReferencesFrom(to)) {
                                if (r2.isExternalReference()) {
                                    Function g = fm.getFunctionAt(r2.getToAddress());
                                    imports.add(g != null ? extName(g)
                                            : currentProgram.getSymbolTable().getPrimarySymbol(r2.getToAddress()).getName(true));
                                }
                            }
                        }
                        if (!NvCommon.inExecutableMemory(currentProgram, to) &&
                            NvCommon.inInitializedMemory(currentProgram, to)) {
                            String v = NvCommon.readAsciiZ(currentProgram, to, 4, 1024);
                            if (v != null) {
                                strings.add(clean(v));
                            }
                        }
                    }
                }
                Function thunked = f.isThunk() ? f.getThunkedFunction(false) : null;
                StringBuilder tags = new StringBuilder();
                f.getTags().stream().map(t -> t.getName()).sorted().forEach(t -> {
                    if (tags.length() > 0) {
                        tags.append(';');
                    }
                    tags.append(t);
                });
                w.write(NvCommon.addr(f.getEntryPoint()));
                w.write('\t');
                w.write(Long.toString(body.getNumAddresses()));
                w.write('\t');
                w.write(clean(f.getName(true), 4096));
                w.write('\t');
                w.write(f.getSymbol().getSource().toString());
                w.write('\t');
                w.write(tags.toString());
                w.write('\t');
                w.write(thunked == null ? "" : (thunked.isExternal() ? extName(thunked)
                        : NvCommon.addr(thunked.getEntryPoint())));
                w.write('\t');
                w.write(String.join(",", calls));
                w.write('\t');
                w.write(String.join(",", imports));
                w.write('\t');
                w.write(String.join("|", strings));
                // References to the entry: distinct calling functions, other
                // references from code (push imm, mov reg, imm: a function
                // pointer handed on), and references from data (tables).
                Set<Address> callers = new HashSet<>();
                int codeRefs = 0;
                int dataRefs = 0;
                for (Reference r : getReferencesTo(f.getEntryPoint())) {
                    Address from = r.getFromAddress();
                    Function cf = fm.getFunctionContaining(from);
                    boolean fromCode = listing.getInstructionAt(from) != null;
                    if (fromCode && r.getReferenceType().isCall()) {
                        callers.add(cf == null ? from : cf.getEntryPoint());
                    }
                    else if (fromCode) {
                        codeRefs++;
                    }
                    else if (from.isMemoryAddress()) {
                        dataRefs++;
                    }
                }
                w.write('\t');
                w.write(Integer.toString(callers.size()));
                w.write('\t');
                w.write(Integer.toString(codeRefs));
                w.write('\t');
                w.write(Integer.toString(dataRefs));
                w.write('\n');
                rows++;
            }
        }

        int vtables = writeVtables(out.resolve("vtables.tsv"));

        String manifest = "program\t" + currentProgram.getName() + "\n" +
            "exe_sha256\t" + NvCommon.exeSha256(currentProgram, NAME) + "\n" +
            "function_count_api\t" + fm.getFunctionCount() + "\n" +
            "external_functions\t" + externals + "\n" +
            "rows_written\t" + rows + "\n" +
            "vtables\t" + vtables + "\n" +
            "gate_passed\t" + (rows == expected) + "\n";
        if (rows != expected) {
            Files.writeString(out.resolve("manifest.failed.txt"), manifest);
            throw new IllegalStateException(NAME + ": wrote " + rows + " rows, expected " + expected);
        }
        Files.writeString(out.resolve("manifest.txt"), manifest);
        println(NAME + ": " + rows + " functions, " + vtables + " vtables");
    }

    private int writeVtables(Path file) throws Exception {
        Memory mem = currentProgram.getMemory();
        FunctionManager fm = currentProgram.getFunctionManager();
        // 1. Complete object locators.
        Map<Long, String> colType = new TreeMap<>();
        Map<Long, Long> colOffset = new HashMap<>();
        List<MemoryBlock> ro = new ArrayList<>();
        for (MemoryBlock b : mem.getBlocks()) {
            if (b.isInitialized() && !b.isExecute() && b.getName().startsWith(".rdata")) {
                ro.add(b);
            }
        }
        for (MemoryBlock b : ro) {
            byte[] bytes = new byte[(int) b.getSize()];
            b.getBytes(b.getStart(), bytes);
            long base = b.getStart().getOffset();
            for (int i = 0; i + 20 <= bytes.length; i += 4) {
                if (NvCommon.le(bytes, i, 4) != 0) {
                    continue;
                }
                long td = NvCommon.le(bytes, i + 12, 4);
                long chd = NvCommon.le(bytes, i + 16, 4);
                if (!inBlocks(ro, chd)) {
                    continue;
                }
                Address tdName = toAddr(td + 8);
                if (!NvCommon.inInitializedMemory(currentProgram, tdName)) {
                    continue;
                }
                String name = NvCommon.readAsciiZ(currentProgram, tdName, 4, 4096);
                if (name == null || !name.startsWith(".?A")) {
                    continue;
                }
                colType.put(base + i, name);
                colOffset.put(base + i, NvCommon.le(bytes, i + 4, 4));
            }
        }
        // 2. Pointers to a COL mark the word before a vtable.
        Set<Long> meta = new HashSet<>();
        Map<Long, Long> vtCol = new TreeMap<>();
        for (MemoryBlock b : ro) {
            byte[] bytes = new byte[(int) b.getSize()];
            b.getBytes(b.getStart(), bytes);
            long base = b.getStart().getOffset();
            for (int i = 0; i + 8 <= bytes.length; i += 4) {
                long v = NvCommon.le(bytes, i, 4);
                if (colType.containsKey(v) && NvCommon.inExecutableMemory(currentProgram, toAddr(NvCommon.le(bytes, i + 4, 4)))) {
                    meta.add(base + i);
                    vtCol.put(base + i + 4, v);
                }
            }
        }
        // 3. Slots.
        int n = 0;
        try (BufferedWriter w = Files.newBufferedWriter(file, StandardCharsets.UTF_8)) {
            w.write("vtable\tcol\ttype_name\toffset\tslots\tslot_is_function\n");
            for (Map.Entry<Long, Long> e : vtCol.entrySet()) {
                long vt = e.getKey();
                List<String> slots = new ArrayList<>();
                StringBuilder isFn = new StringBuilder();
                for (long a = vt; ; a += 4) {
                    if (a != vt && meta.contains(a)) {
                        break;
                    }
                    Address sa = toAddr(a);
                    if (!NvCommon.inInitializedMemory(currentProgram, sa)) {
                        break;
                    }
                    long t = NvCommon.readU32(currentProgram, sa);
                    Address ta = toAddr(t);
                    if (!NvCommon.inExecutableMemory(currentProgram, ta)) {
                        break;
                    }
                    slots.add(NvCommon.hex8(t));
                    isFn.append(fm.getFunctionAt(ta) != null ? '1' : '0');
                }
                w.write(NvCommon.hex8(vt) + "\t" + NvCommon.hex8(e.getValue()) + "\t" +
                    clean(colType.get(e.getValue()), 4096) + "\t" + colOffset.get(e.getValue()) + "\t" +
                    String.join(",", slots) + "\t" + isFn + "\n");
                n++;
            }
        }
        return n;
    }

    private static boolean inBlocks(List<MemoryBlock> blocks, long a) {
        for (MemoryBlock b : blocks) {
            if (a >= b.getStart().getOffset() && a <= b.getEnd().getOffset()) {
                return true;
            }
        }
        return false;
    }

    private static String extName(Function g) {
        ExternalLocation loc = g.getExternalLocation();
        String lib = loc == null ? "" : loc.getLibraryName();
        return clean(lib + "::" + g.getName(), 4096);
    }

    private static String clean(String s) {
        return clean(s, 60);
    }

    private static String clean(String s, int max) {
        StringBuilder b = new StringBuilder();
        for (int i = 0; i < s.length() && b.length() < max; i++) {
            char c = s.charAt(i);
            b.append(c < 0x20 || c == '|' || c == ',' || c == 0x7f ? '_' : c);
        }
        return b.toString();
    }
}

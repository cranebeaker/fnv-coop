// Records which executable a Ghidra program was built from.
//
// Args: out=<file.json>
// Run it against the analyzed program (read-only is fine). The JSON holds
// the file hashes, the image layout and the PE header facts, so every
// later export or name map can name the exact build it describes.
//
// Hashes come from Ghidra's own import record (Program.getExecutableSHA256
// and getExecutableMD5), which are hashes of the file as it was imported.
// The PE section table is read from the header bytes in the program. The
// SHA-256 of each section's raw bytes needs the file itself: it is read from
// the path Ghidra recorded, and only if that file still has the recorded
// SHA-256. The recorded path and the PDB path of the CodeView record are
// written with the user-profile folder replaced by %USERPROFILE% (and with
// forward slashes); the file name is kept.
//
// @category NV

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.framework.Application;
import ghidra.framework.options.Options;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Program;
import ghidra.program.model.mem.MemoryBlock;

public class NvExeIdentity extends GhidraScript {
    private static final String ME = "NvExeIdentity";

    private static final String[] DEBUG_TYPE_NAMES = { "UNKNOWN", "COFF", "CODEVIEW", "FPO", "MISC",
        "EXCEPTION", "FIXUP", "OMAP_TO_SRC", "OMAP_FROM_SRC", "BORLAND", "RESERVED10", "CLSID",
        "VC_FEATURE", "POGO", "ILTCG", "MPX", "REPRO", "EMBEDDED_PORTABLE_PDB", "SPGO",
        "PDBCHECKSUM", "EX_DLLCHARACTERISTICS" };

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(ME, getScriptArgs(), Set.of("out"));
        Path out = Paths.get(NvCommon.require(ME, args, "out"));
        if (currentProgram == null) {
            throw new IllegalStateException(ME + ": no program is open");
        }
        Program p = currentProgram;

        Map<String, Object> doc = NvCommon.obj();
        doc.put("tool", ME);
        doc.put("program_name", p.getName());
        doc.put("executable_path", NvCommon.redactUserPath(p.getExecutablePath()));
        doc.put("executable_file_name", NvCommon.fileNameOf(p.getExecutablePath()));
        doc.put("executable_format", p.getExecutableFormat());
        doc.put("sha256", NvCommon.exeSha256(p, ME));
        doc.put("md5", p.getExecutableMD5());
        doc.put("language_id", p.getLanguageID().getIdAsString());
        doc.put("compiler_spec_id", p.getCompilerSpec().getCompilerSpecID().getIdAsString());
        doc.put("image_base", NvCommon.addr(p.getImageBase()));
        doc.put("ghidra_version", Application.getApplicationVersion());
        doc.put("ghidra_created_with", p.getOptions(Program.PROGRAM_INFO)
            .getString("Created With Ghidra Version", ""));

        FunctionManager fm = p.getFunctionManager();
        int external = 0;
        for (Function f : fm.getExternalFunctions()) {
            external++;
        }
        doc.put("function_count", fm.getFunctionCount());
        doc.put("function_count_external", external);
        doc.put("function_count_internal", fm.getFunctionCount() - external);

        doc.put("memory_blocks", memoryBlocks(p));
        doc.put("pe_header", peHeader(p));
        doc.put("codeview_from_options", codeViewFromOptions(p));

        NvCommon.writeText(out, NvCommon.toPrettyJson(doc));
        println(ME + ": wrote " + out + " (sha256 " + p.getExecutableSHA256() + ")");
    }

    private List<Object> memoryBlocks(Program p) {
        List<Object> list = new ArrayList<>();
        for (MemoryBlock b : p.getMemory().getBlocks()) {
            Map<String, Object> m = NvCommon.obj();
            m.put("name", b.getName());
            m.put("start", NvCommon.addr(b.getStart()));
            m.put("end", NvCommon.addr(b.getEnd()));
            m.put("size", b.getSize());
            m.put("flags", (b.isRead() ? "r" : "-") + (b.isWrite() ? "w" : "-") + (b.isExecute() ? "x" : "-"));
            m.put("read", b.isRead());
            m.put("write", b.isWrite());
            m.put("execute", b.isExecute());
            m.put("initialized", b.isInitialized());
            m.put("overlay", b.isOverlay());
            list.add(m);
        }
        return list;
    }

    private Map<String, Object> codeViewFromOptions(Program p) {
        Options o = p.getOptions(Program.PROGRAM_INFO);
        Map<String, Object> m = NvCommon.obj();
        boolean any = false;
        // Option names are the ones Ghidra's PE loader writes (PdbParserConstants).
        String[][] keys = { { "pdb_file", "PDB File" }, { "pdb_guid", "PDB GUID" },
            { "pdb_signature", "PDB Signature" }, { "pdb_age", "PDB Age" },
            { "pdb_version", "PDB Version" } };
        for (String[] k : keys) {
            if (o.contains(k[1])) {
                String v = o.getString(k[1], "");
                // The PDB path of a module you built is often under your profile folder.
                m.put(k[0], k[0].equals("pdb_file") ? NvCommon.redactUserPath(v) : v);
                any = true;
            }
        }
        if (o.contains("PDB Age")) {
            // Ghidra stores the age as hex text; also give it as a plain number.
            try {
                m.put("pdb_age_decimal", Long.parseLong(o.getString("PDB Age", ""), 16));
            }
            catch (NumberFormatException e) {
                // leave it out when the text is not hex
            }
        }
        if (o.contains("PDB Loaded")) {
            m.put("pdb_loaded", o.getBoolean("PDB Loaded", false));
            any = true;
        }
        m.put("present", any);
        return m;
    }

    /** PE facts read from the program's own header memory (the "Headers" block). */
    private Map<String, Object> peHeader(Program p) throws Exception {
        Map<String, Object> m = NvCommon.obj();
        Address base = p.getImageBase();
        byte[] dos = NvCommon.readBytes(p, base, 0x40);
        if (dos.length < 0x40 || dos[0] != 'M' || dos[1] != 'Z') {
            m.put("present", false);
            return m;
        }
        int lfanew = (int) NvCommon.le(dos, 0x3c, 4);
        byte[] nt = NvCommon.readBytes(p, base.add(lfanew), 24 + 0xf0);
        if (nt.length < 24 + 0x60 || nt[0] != 'P' || nt[1] != 'E' || nt[2] != 0 || nt[3] != 0) {
            m.put("present", false);
            return m;
        }
        m.put("present", true);
        m.put("pe_header_offset", lfanew);
        int machine = (int) NvCommon.le(nt, 4, 2);
        int sections = (int) NvCommon.le(nt, 6, 2);
        long stamp = NvCommon.le(nt, 8, 4);
        int optSize = (int) NvCommon.le(nt, 20, 2);
        int characteristics = (int) NvCommon.le(nt, 22, 2);
        m.put("machine", String.format("0x%04x", machine));
        m.put("number_of_sections", sections);
        m.put("time_date_stamp", stamp);
        m.put("time_date_stamp_utc", Instant.ofEpochSecond(stamp).toString());
        m.put("characteristics", String.format("0x%04x", characteristics));
        m.put("large_address_aware", (characteristics & 0x0020) != 0);
        m.put("is_dll", (characteristics & 0x2000) != 0);
        m.put("relocations_stripped", (characteristics & 0x0001) != 0);

        int opt = 24;
        int magic = (int) NvCommon.le(nt, opt, 2);
        boolean pe32plus = magic == 0x20b;
        m.put("optional_magic", String.format("0x%04x", magic));
        m.put("format", pe32plus ? "PE32+" : (magic == 0x10b ? "PE32" : "unknown"));
        m.put("entry_point_rva", String.format("0x%08x", NvCommon.le(nt, opt + 16, 4)));
        m.put("image_base_in_header",
            String.format("0x%x", pe32plus ? NvCommon.le(nt, opt + 24, 8) : NvCommon.le(nt, opt + 28, 4)));
        m.put("section_alignment", NvCommon.le(nt, opt + 32, 4));
        m.put("file_alignment", NvCommon.le(nt, opt + 36, 4));
        m.put("size_of_image", NvCommon.le(nt, opt + 56, 4));
        m.put("size_of_headers", NvCommon.le(nt, opt + 60, 4));
        m.put("checksum", String.format("0x%08x", NvCommon.le(nt, opt + 64, 4)));
        m.put("subsystem", (int) NvCommon.le(nt, opt + 68, 2));
        m.put("dll_characteristics", String.format("0x%04x", NvCommon.le(nt, opt + 70, 2)));
        m.put("size_of_optional_header", optSize);

        // Debug directory (data directory index 6) and the CodeView record in it.
        int dirOff = opt + (pe32plus ? 112 : 96) + 6 * 8;
        List<Object> entries = new ArrayList<>();
        Map<String, Object> codeview = NvCommon.obj();
        codeview.put("present", false);
        if (dirOff + 8 <= nt.length) {
            long dirRva = NvCommon.le(nt, dirOff, 4);
            long dirSize = NvCommon.le(nt, dirOff + 4, 4);
            if (dirRva != 0 && dirSize >= 28 && dirSize < 0x10000) {
                byte[] dir = NvCommon.readBytes(p, base.add(dirRva), (int) dirSize);
                for (int off = 0; off + 28 <= dir.length; off += 28) {
                    Map<String, Object> e = NvCommon.obj();
                    int type = (int) NvCommon.le(dir, off + 12, 4);
                    long dataSize = NvCommon.le(dir, off + 16, 4);
                    long dataRva = NvCommon.le(dir, off + 20, 4);
                    e.put("type", type);
                    e.put("type_name", type < DEBUG_TYPE_NAMES.length ? DEBUG_TYPE_NAMES[type] : "OTHER");
                    e.put("time_date_stamp", NvCommon.le(dir, off + 4, 4));
                    e.put("size_of_data", dataSize);
                    e.put("rva", String.format("0x%08x", dataRva));
                    entries.add(e);
                    if (type == 2 && dataRva != 0 && dataSize >= 24 && dataSize < 0x1000) {
                        readCodeView(p, base.add(dataRva), (int) dataSize, codeview);
                    }
                }
            }
        }
        m.put("debug_directory", entries);
        m.put("codeview_from_headers", codeview);

        // Rich header: the bytes between the DOS header and the PE signature.
        m.put("rich_header", NvCommon.parseRichHeader(NvCommon.readBytes(p, base, lfanew)));

        // Section table, with the hash of each section's raw bytes from the file.
        int tableAt = lfanew + 24 + optSize;
        byte[] table = NvCommon.readBytes(p, base.add(tableAt), 40 * sections);
        byte[] file = null;
        String hashStatus;
        Path exe = NvCommon.executableFile(p.getExecutablePath());
        if (exe == null || !Files.isRegularFile(exe)) {
            hashStatus = "the executable is not at the path Ghidra recorded; raw hashes not computed";
        }
        else {
            byte[] data = Files.readAllBytes(exe);
            if (NvCommon.sha256Hex(data, 0, data.length).equalsIgnoreCase(p.getExecutableSHA256())) {
                file = data;
                hashStatus = "ok: file at the recorded path has the recorded SHA-256";
            }
            else {
                hashStatus = "the file at the recorded path differs from the imported file; raw hashes not computed";
            }
        }
        m.put("section_hash_status", hashStatus);
        List<Object> secs = new ArrayList<>();
        for (int i = 0; i < sections && (i + 1) * 40 <= table.length; i++) {
            int o = i * 40;
            int nameEnd = 0;
            while (nameEnd < 8 && table[o + nameEnd] != 0) {
                nameEnd++;
            }
            long rawSize = NvCommon.le(table, o + 16, 4);
            long rawPtr = NvCommon.le(table, o + 20, 4);
            Map<String, Object> s = NvCommon.obj();
            s.put("name", new String(table, o, nameEnd, java.nio.charset.StandardCharsets.ISO_8859_1));
            s.put("virtual_size", NvCommon.le(table, o + 8, 4));
            s.put("virtual_address", String.format("0x%08x", NvCommon.le(table, o + 12, 4)));
            s.put("size_of_raw_data", rawSize);
            s.put("pointer_to_raw_data", String.format("0x%08x", rawPtr));
            s.put("characteristics", String.format("0x%08x", NvCommon.le(table, o + 36, 4)));
            if (file != null) {
                boolean inFile = rawPtr + rawSize <= file.length;
                int from = (int) Math.min(rawPtr, file.length);
                int len = (int) Math.min(rawSize, file.length - from);
                s.put("raw_in_file", inFile);
                s.put("raw_sha256", NvCommon.sha256Hex(file, from, len));
            }
            else {
                s.put("raw_in_file", null);
                s.put("raw_sha256", null);
            }
            secs.add(s);
        }
        m.put("section_table", secs);
        return m;
    }

    private void readCodeView(Program p, Address at, int size, Map<String, Object> out) {
        byte[] cv = NvCommon.readBytes(p, at, size);
        if (cv.length >= 24 && cv[0] == 'R' && cv[1] == 'S' && cv[2] == 'D' && cv[3] == 'S') {
            out.put("present", true);
            out.put("format", "RSDS");
            // GUID stored as Data1 (u32), Data2 (u16), Data3 (u16), Data4 (8 bytes), little-endian fields.
            long d1 = NvCommon.le(cv, 4, 4);
            long d2 = NvCommon.le(cv, 8, 2);
            long d3 = NvCommon.le(cv, 10, 2);
            StringBuilder g = new StringBuilder(String.format("%08x-%04x-%04x-", d1, d2, d3));
            for (int i = 0; i < 8; i++) {
                g.append(String.format("%02x", cv[12 + i] & 0xff));
                if (i == 1) {
                    g.append('-');
                }
            }
            out.put("guid", g.toString());
            out.put("age", NvCommon.le(cv, 20, 4));
            int end = 24;
            while (end < cv.length && cv[end] != 0) {
                end++;
            }
            out.put("pdb_path", NvCommon.redactUserPath(
                new String(cv, 24, end - 24, java.nio.charset.StandardCharsets.UTF_8)));
        }
        else if (cv.length >= 16 && cv[0] == 'N' && cv[1] == 'B' && cv[2] == '1' && cv[3] == '0') {
            out.put("present", true);
            out.put("format", "NB10");
            out.put("signature", String.format("%08x", NvCommon.le(cv, 8, 4)));
            out.put("age", NvCommon.le(cv, 12, 4));
            int end = 16;
            while (end < cv.length && cv[end] != 0) {
                end++;
            }
            out.put("pdb_path", NvCommon.redactUserPath(
                new String(cv, 16, end - 16, java.nio.charset.StandardCharsets.UTF_8)));
        }
    }
}

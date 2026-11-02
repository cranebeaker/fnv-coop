// Shared helpers for the Nv* Ghidra scripts in this directory.
//
// This is not a runnable script. Ghidra compiles every .java file in a
// script directory together, so the scripts can call these static methods.
// Everything here is plain Java plus Ghidra's own API: no third-party
// libraries, so the scripts run on a stock Ghidra install.

import java.io.IOException;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.math.MathContext;
import java.nio.charset.StandardCharsets;
import java.nio.file.InvalidPathException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.security.MessageDigest;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSetView;
import ghidra.program.model.address.AddressRange;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionTag;
import ghidra.program.model.listing.Program;
import ghidra.program.model.mem.Memory;
import ghidra.program.model.mem.MemoryAccessException;
import ghidra.program.model.mem.MemoryBlock;

public class NvCommon {
    private NvCommon() {}

    /** Bookmark category that NvImportNameMap uses for the provenance of a label. */
    public static final String NV_NAMES_BOOKMARK = "nv-names";

    // ------------------------------------------------------------------
    // Arguments: key=value pairs from getScriptArgs()
    // ------------------------------------------------------------------

    /**
     * Parses key=value arguments. Throws on a token without '=', on a key
     * that is not in {@code allowed}, and on a repeated key. Keys are
     * lower-cased. A value may be empty.
     */
    public static Map<String, String> parseArgs(String scriptName, String[] raw, Set<String> allowed) {
        Map<String, String> out = new LinkedHashMap<>();
        for (String token : raw) {
            int eq = token.indexOf('=');
            if (eq <= 0) {
                throw new IllegalArgumentException(
                    scriptName + ": argument '" + token + "' is not key=value");
            }
            String key = token.substring(0, eq).toLowerCase();
            String value = token.substring(eq + 1);
            if (!allowed.contains(key)) {
                throw new IllegalArgumentException(scriptName + ": unknown argument '" + key +
                    "'. Allowed: " + String.join(", ", new java.util.TreeSet<>(allowed)));
            }
            if (out.containsKey(key)) {
                throw new IllegalArgumentException(scriptName + ": argument '" + key + "' given twice");
            }
            out.put(key, value);
        }
        return out;
    }

    public static String require(String scriptName, Map<String, String> args, String key) {
        String v = args.get(key);
        if (v == null || v.isEmpty()) {
            throw new IllegalArgumentException(scriptName + ": missing required argument " + key + "=...");
        }
        return v;
    }

    public static boolean flag(String scriptName, Map<String, String> args, String key, boolean dflt) {
        String v = args.get(key);
        if (v == null) {
            return dflt;
        }
        switch (v) {
            case "1":
            case "true":
                return true;
            case "0":
            case "false":
                return false;
            default:
                throw new IllegalArgumentException(
                    scriptName + ": " + key + " must be 0 or 1, got '" + v + "'");
        }
    }

    public static int intArg(String scriptName, Map<String, String> args, String key, int dflt, int min,
            int max) {
        String v = args.get(key);
        if (v == null) {
            return dflt;
        }
        int n;
        try {
            n = Integer.parseInt(v.trim());
        }
        catch (NumberFormatException e) {
            throw new IllegalArgumentException(scriptName + ": " + key + " must be an integer, got '" + v + "'");
        }
        if (n < min || n > max) {
            throw new IllegalArgumentException(
                scriptName + ": " + key + " must be in " + min + ".." + max + ", got " + n);
        }
        return n;
    }

    /** Parses hex with an optional 0x prefix. */
    public static long parseHex(String scriptName, String what, String text) {
        String t = text.trim();
        if (t.startsWith("0x") || t.startsWith("0X")) {
            t = t.substring(2);
        }
        if (t.isEmpty() || t.length() > 16) {
            throw new IllegalArgumentException(scriptName + ": bad hex value for " + what + ": '" + text + "'");
        }
        try {
            return Long.parseUnsignedLong(t, 16);
        }
        catch (NumberFormatException e) {
            throw new IllegalArgumentException(scriptName + ": bad hex value for " + what + ": '" + text + "'");
        }
    }

    // ------------------------------------------------------------------
    // Text helpers
    // ------------------------------------------------------------------

    public static String hex8(long v) {
        return String.format("%08x", v & 0xffffffffL);
    }

    /** Address as 8+ lowercase hex digits, no prefix (the form used in repo notes). */
    public static String addr(Address a) {
        long off = a.getOffset();
        return a.getAddressSpace().getSize() <= 32 ? hex8(off) : Long.toHexString(off);
    }

    public static String bytesToHex(byte[] b) {
        StringBuilder sb = new StringBuilder(b.length * 2);
        for (byte x : b) {
            sb.append(String.format("%02x", x & 0xff));
        }
        return sb.toString();
    }

    public static String truncate(String s, int max) {
        if (s.length() <= max) {
            return s;
        }
        return s.substring(0, max);
    }

    // ------------------------------------------------------------------
    // JSON writer: Map / List / String / Number / Boolean / null / arrays
    // ------------------------------------------------------------------

    public static Map<String, Object> obj() {
        return new LinkedHashMap<>();
    }

    public static String toJson(Object o) {
        StringBuilder sb = new StringBuilder();
        writeJson(sb, o, -1, 0);
        return sb.toString();
    }

    /** Pretty form with two-space indent. */
    public static String toPrettyJson(Object o) {
        StringBuilder sb = new StringBuilder();
        writeJson(sb, o, 2, 0);
        sb.append('\n');
        return sb.toString();
    }

    @SuppressWarnings("unchecked")
    private static void writeJson(StringBuilder sb, Object o, int indent, int depth) {
        if (o == null) {
            sb.append("null");
        }
        else if (o instanceof String) {
            jsonString(sb, (String) o);
        }
        else if (o instanceof Boolean) {
            sb.append(o.toString());
        }
        else if (o instanceof Float) {
            float f = (Float) o;
            if (Float.isNaN(f) || Float.isInfinite(f)) {
                jsonString(sb, Float.toString(f));
            }
            else {
                sb.append(Float.toString(f));
            }
        }
        else if (o instanceof Double) {
            double d = (Double) o;
            if (Double.isNaN(d) || Double.isInfinite(d)) {
                jsonString(sb, Double.toString(d));
            }
            else {
                sb.append(Double.toString(d));
            }
        }
        else if (o instanceof Number) {
            sb.append(o.toString());
        }
        else if (o instanceof Map) {
            Map<String, Object> m = (Map<String, Object>) o;
            if (m.isEmpty()) {
                sb.append("{}");
                return;
            }
            sb.append('{');
            boolean first = true;
            for (Map.Entry<String, Object> e : m.entrySet()) {
                if (!first) {
                    sb.append(',');
                }
                first = false;
                newline(sb, indent, depth + 1);
                jsonString(sb, e.getKey());
                sb.append(indent >= 0 ? ": " : ":");
                writeJson(sb, e.getValue(), indent, depth + 1);
            }
            newline(sb, indent, depth);
            sb.append('}');
        }
        else if (o instanceof Collection) {
            Collection<Object> c = (Collection<Object>) o;
            if (c.isEmpty()) {
                sb.append("[]");
                return;
            }
            // Short scalar lists stay on one line even in pretty mode.
            boolean scalars = true;
            for (Object x : c) {
                if (x instanceof Map || x instanceof Collection) {
                    scalars = false;
                    break;
                }
            }
            sb.append('[');
            boolean first = true;
            for (Object x : c) {
                if (!first) {
                    sb.append(indent >= 0 && scalars ? ", " : ",");
                }
                first = false;
                if (!scalars) {
                    newline(sb, indent, depth + 1);
                }
                writeJson(sb, x, indent, depth + 1);
            }
            if (!scalars) {
                newline(sb, indent, depth);
            }
            sb.append(']');
        }
        else if (o instanceof int[]) {
            List<Object> l = new ArrayList<>();
            for (int x : (int[]) o) {
                l.add(x);
            }
            writeJson(sb, l, indent, depth);
        }
        else if (o instanceof Object[]) {
            writeJson(sb, Arrays.asList((Object[]) o), indent, depth);
        }
        else {
            jsonString(sb, o.toString());
        }
    }

    private static void newline(StringBuilder sb, int indent, int depth) {
        if (indent < 0) {
            return;
        }
        sb.append('\n');
        for (int i = 0; i < depth * indent; i++) {
            sb.append(' ');
        }
    }

    public static void jsonString(StringBuilder sb, String s) {
        sb.append('"');
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            switch (c) {
                case '"':
                    sb.append("\\\"");
                    break;
                case '\\':
                    sb.append("\\\\");
                    break;
                case '\n':
                    sb.append("\\n");
                    break;
                case '\r':
                    sb.append("\\r");
                    break;
                case '\t':
                    sb.append("\\t");
                    break;
                default:
                    if (c < 0x20 || c == 0x7f || (c >= 0x80 && c < 0xa0)) {
                        sb.append(String.format("\\u%04x", (int) c));
                    }
                    else {
                        sb.append(c);
                    }
            }
        }
        sb.append('"');
    }

    // ------------------------------------------------------------------
    // CSV
    // ------------------------------------------------------------------

    public static String csvLine(List<String> cells) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < cells.size(); i++) {
            if (i > 0) {
                sb.append(',');
            }
            String c = cells.get(i) == null ? "" : cells.get(i);
            if (c.indexOf(',') >= 0 || c.indexOf('"') >= 0 || c.indexOf('\n') >= 0 || c.indexOf('\r') >= 0) {
                sb.append('"').append(c.replace("\"", "\"\"")).append('"');
            }
            else {
                sb.append(c);
            }
        }
        return sb.toString();
    }

    /**
     * Splits CSV text into rows of cells. Handles quoted cells with doubled
     * quotes and embedded commas or newlines. Blank lines are skipped.
     */
    public static List<List<String>> parseCsv(String text) {
        List<List<String>> rows = new ArrayList<>();
        List<String> row = new ArrayList<>();
        StringBuilder cell = new StringBuilder();
        boolean inQuotes = false;
        boolean cellWasQuoted = false;
        int n = text.length();
        for (int i = 0; i < n; i++) {
            char c = text.charAt(i);
            if (inQuotes) {
                if (c == '"') {
                    if (i + 1 < n && text.charAt(i + 1) == '"') {
                        cell.append('"');
                        i++;
                    }
                    else {
                        inQuotes = false;
                    }
                }
                else {
                    cell.append(c);
                }
                continue;
            }
            if (c == '"' && cell.length() == 0 && !cellWasQuoted) {
                inQuotes = true;
                cellWasQuoted = true;
            }
            else if (c == ',') {
                row.add(cell.toString());
                cell.setLength(0);
                cellWasQuoted = false;
            }
            else if (c == '\n' || c == '\r') {
                if (c == '\r' && i + 1 < n && text.charAt(i + 1) == '\n') {
                    i++;
                }
                row.add(cell.toString());
                cell.setLength(0);
                cellWasQuoted = false;
                if (!(row.size() == 1 && row.get(0).isEmpty())) {
                    rows.add(row);
                }
                row = new ArrayList<>();
            }
            else {
                cell.append(c);
            }
        }
        if (inQuotes) {
            throw new IllegalArgumentException("CSV ends inside a quoted cell");
        }
        if (cell.length() > 0 || !row.isEmpty()) {
            row.add(cell.toString());
            if (!(row.size() == 1 && row.get(0).isEmpty())) {
                rows.add(row);
            }
        }
        return rows;
    }

    // ------------------------------------------------------------------
    // Names
    // ------------------------------------------------------------------

    /** Replaces every character outside [A-Za-z0-9_] with '_'. */
    public static String identifierFrom(String text) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            boolean ok = (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') ||
                c == '_';
            sb.append(ok ? c : '_');
        }
        return sb.toString();
    }

    /**
     * Name sanitization used by NvImportNameMap: trim, then turn each run of
     * whitespace into one underscore. The result must then match
     * [A-Za-z0-9_:~&lt;&gt;]+ with no empty "::" segment, or null is returned
     * and {@code why[0]} says what was wrong.
     */
    public static String sanitizeQualifiedName(String raw, String[] why) {
        String t = raw == null ? "" : raw.trim().replaceAll("\\s+", "_");
        if (t.isEmpty()) {
            why[0] = "empty name";
            return null;
        }
        for (int i = 0; i < t.length(); i++) {
            char c = t.charAt(i);
            boolean ok = (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') ||
                c == '_' || c == ':' || c == '~' || c == '<' || c == '>';
            if (!ok) {
                why[0] = "character '" + c + "' (U+" + String.format("%04X", (int) c) +
                    ") is outside [A-Za-z0-9_:~<>]";
                return null;
            }
        }
        List<String> parts = splitQualified(t);
        if (parts == null) {
            why[0] = "malformed '::' separators";
            return null;
        }
        for (String p : parts) {
            if (p.isEmpty()) {
                why[0] = "empty '::' segment";
                return null;
            }
        }
        return t;
    }

    /**
     * Splits on "::" that is not inside angle brackets (so template
     * arguments keep their own scope operators). Returns null when a single
     * ':' is found outside brackets or the brackets do not balance.
     */
    public static List<String> splitQualified(String name) {
        List<String> parts = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        int depth = 0;
        for (int i = 0; i < name.length(); i++) {
            char c = name.charAt(i);
            if (c == '<') {
                depth++;
            }
            else if (c == '>') {
                depth--;
                if (depth < 0) {
                    return null;
                }
            }
            if (depth == 0 && c == ':') {
                if (i + 1 < name.length() && name.charAt(i + 1) == ':') {
                    parts.add(cur.toString());
                    cur.setLength(0);
                    i++;
                    continue;
                }
                return null;
            }
            cur.append(c);
        }
        if (depth != 0) {
            return null;
        }
        parts.add(cur.toString());
        return parts;
    }

    // ------------------------------------------------------------------
    // Program identity
    // ------------------------------------------------------------------

    /**
     * SHA-256 of the executable Ghidra imported. Every output carries it, so
     * a program without one (not imported from a file) is refused.
     */
    public static String exeSha256(Program p, String scriptName) {
        String sha = p.getExecutableSHA256();
        if (sha == null || sha.isEmpty()) {
            throw new IllegalStateException(scriptName + ": this program has no executable SHA-256 " +
                "(it was not imported from a file). Import the exe into the project first.");
        }
        return sha;
    }

    // ------------------------------------------------------------------
    // Provenance tags
    // ------------------------------------------------------------------

    /**
     * Removes every "src:" and "pin:" tag from a function. A script calls
     * this before it replaces a name that was not a Ghidra default, so the
     * tags that remain describe where the current name came from and not
     * the name that was thrown away.
     */
    public static void clearProvenanceTags(Function fn) {
        List<String> old = new ArrayList<>();
        for (FunctionTag t : fn.getTags()) {
            String n = t.getName();
            if (n.startsWith("src:") || n.startsWith("pin:")) {
                old.add(n);
            }
        }
        for (String n : old) {
            fn.removeTag(n);
        }
    }

    // ------------------------------------------------------------------
    // Memory helpers
    // ------------------------------------------------------------------

    public static boolean inInitializedMemory(Program p, Address a) {
        MemoryBlock b = p.getMemory().getBlock(a);
        return b != null && b.isInitialized();
    }

    public static boolean inExecutableMemory(Program p, Address a) {
        MemoryBlock b = p.getMemory().getBlock(a);
        return b != null && b.isExecute();
    }

    /** Reads up to len bytes; returns fewer (possibly 0) if memory ends or is uninitialized. */
    public static byte[] readBytes(Program p, Address a, int len) {
        Memory mem = p.getMemory();
        byte[] buf = new byte[len];
        int got = 0;
        try {
            while (got < len) {
                Address at = a.add(got);
                MemoryBlock b = mem.getBlock(at);
                if (b == null || !b.isInitialized()) {
                    break;
                }
                long room = b.getEnd().subtract(at) + 1;
                int chunk = (int) Math.min(room, len - got);
                byte[] tmp = new byte[chunk];
                int n = mem.getBytes(at, tmp);
                System.arraycopy(tmp, 0, buf, got, n);
                got += n;
                if (n < chunk) {
                    break;
                }
            }
        }
        catch (MemoryAccessException | ghidra.program.model.address.AddressOutOfBoundsException e) {
            // return what we have
        }
        return got == len ? buf : Arrays.copyOf(buf, got);
    }

    public static long readU32(Program p, Address a) {
        byte[] b = readBytes(p, a, 4);
        if (b.length < 4) {
            return -1;
        }
        return (b[0] & 0xffL) | ((b[1] & 0xffL) << 8) | ((b[2] & 0xffL) << 16) | ((b[3] & 0xffL) << 24);
    }

    /**
     * Reads a NUL-terminated printable-ASCII string (bytes 0x20-0x7e, plus
     * tab, CR and LF). Returns null if the text is not terminated within
     * maxLen bytes, is shorter than minLen, or holds anything else.
     */
    public static String readAsciiZ(Program p, Address a, int minLen, int maxLen) {
        byte[] b = readBytes(p, a, maxLen + 1);
        int i = 0;
        while (i < b.length && b[i] != 0) {
            int c = b[i] & 0xff;
            boolean printable = (c >= 0x20 && c <= 0x7e) || c == 9 || c == 10 || c == 13;
            if (!printable) {
                return null;
            }
            i++;
        }
        if (i >= b.length || i < minLen) {
            return null; // ran off the end without a NUL, or too short
        }
        return new String(b, 0, i, StandardCharsets.US_ASCII);
    }

    // ------------------------------------------------------------------
    // Number decoding
    // ------------------------------------------------------------------

    public static long le(byte[] b, int off, int len) {
        long v = 0;
        for (int i = len - 1; i >= 0; i--) {
            v = (v << 8) | (b[off + i] & 0xffL);
        }
        return v;
    }

    /**
     * "inf", "-inf", "nan", "0" or "-0" for the special 80-bit encodings,
     * or null for a finite non-zero value.
     */
    private static String f80Special(byte[] b) {
        long mant = le(b, 0, 8);
        int se = (int) le(b, 8, 2);
        boolean neg = (se & 0x8000) != 0;
        int exp = se & 0x7fff;
        if (exp == 0x7fff) {
            if ((mant << 1) == 0) {
                return neg ? "-inf" : "inf";
            }
            return "nan";
        }
        if (mant == 0) {
            return neg ? "-0" : "0";
        }
        return null;
    }

    /** The exact value of a finite, non-zero 80-bit number (sign included). */
    private static BigDecimal f80Exact(byte[] b) {
        long mant = le(b, 0, 8);
        int se = (int) le(b, 8, 2);
        int exp = se & 0x7fff;
        BigDecimal v = new BigDecimal(new BigInteger(Long.toUnsignedString(mant)));
        int e2 = (exp == 0 ? 1 : exp) - 16383 - 63;
        if (e2 >= 0) {
            v = v.multiply(new BigDecimal(BigInteger.TWO.pow(e2)));
        }
        else {
            v = v.divide(new BigDecimal(BigInteger.TWO.pow(-e2))); // exact: divisor is a power of two
        }
        return (se & 0x8000) != 0 ? v.negate() : v;
    }

    /**
     * Decodes x87 80-bit extended precision (little-endian 10 bytes) to a
     * decimal string with 21 significant digits, or "inf"/"-inf"/"nan".
     */
    public static String f80ToString(byte[] b) {
        String special = f80Special(b);
        if (special != null) {
            return special;
        }
        return f80Exact(b).round(new MathContext(21)).stripTrailingZeros().toString();
    }

    /**
     * Decodes x87 80-bit extended to the nearest double. The exact value is
     * rounded once, so a value that lies exactly halfway between two doubles
     * goes to the even one, as the hardware does.
     */
    public static double f80ToDouble(byte[] b) {
        String special = f80Special(b);
        if (special != null) {
            switch (special) {
                case "inf":
                    return Double.POSITIVE_INFINITY;
                case "-inf":
                    return Double.NEGATIVE_INFINITY;
                case "nan":
                    return Double.NaN;
                case "-0":
                    return -0.0;
                default:
                    return 0.0;
            }
        }
        return f80Exact(b).doubleValue();
    }

    /** Short description of a range list for messages. */
    public static String rangesToString(AddressSetView set, int max) {
        StringBuilder sb = new StringBuilder();
        int n = 0;
        for (AddressRange r : set) {
            if (n++ >= max) {
                sb.append(" ...");
                break;
            }
            if (sb.length() > 0) {
                sb.append(' ');
            }
            sb.append(addr(r.getMinAddress())).append('-').append(addr(r.getMaxAddress()));
        }
        return sb.toString();
    }

    // ------------------------------------------------------------------
    // Executable path, hashing and the PE Rich header
    // ------------------------------------------------------------------

    /**
     * Replaces a user-profile prefix with %USERPROFILE%, so a path can be
     * written to a file without naming the person. Recognizes the JVM's
     * user.home, a Windows drive profile folder and the usual Unix and macOS
     * home folders. Backslashes become forward slashes.
     */
    public static String redactUserPath(String path) {
        if (path == null) {
            return "";
        }
        String p = path.replace('\\', '/');
        String home = System.getProperty("user.home");
        if (home != null && home.length() > 1) {
            String h = home.replace('\\', '/');
            if (h.endsWith("/")) {
                h = h.substring(0, h.length() - 1);
            }
            if (p.equals(h) || p.startsWith(h + "/")) {
                return "%USERPROFILE%" + p.substring(h.length());
            }
        }
        // The folder names are split so that this source does not itself look like a profile path.
        String drive = "(?i)^/?[A-Za-z]:/+" + "Us" + "ers" + "/+[^/]+";
        String unix = "^/(" + "Us" + "ers|home)/[^/]+";
        for (String re : new String[] { drive, unix }) {
            java.util.regex.Matcher m = java.util.regex.Pattern.compile(re).matcher(p);
            if (m.find()) {
                return "%USERPROFILE%" + p.substring(m.end());
            }
        }
        return p;
    }

    // ------------------------------------------------------------------
    // Vtable labels
    // ------------------------------------------------------------------

    /** What isVtableName accepts, as written into the export manifest. */
    public static final String VTABLE_NAME_RULE =
        "label name ends with 'vftable' (Ghidra's 'vftable'), or after its first character starts with " +
            "'vftable' (the PDB form '`vftable''), or contains 'vftable_for_' (what Ghidra's " +
            "RecoverClassesFromRTTIScript makes for a base class's table) or 'vftable{for'; " +
            "'vftable_meta_ptr' is not a table";

    /**
     * True for the names a vtable label has. Ghidra's RTTI class recovery names
     * the only table of a class "vftable" and, for a class with several bases,
     * the others "vftable_for_<Base>". PDB-style names are "`vftable'" and
     * "`vftable'{for `Base'}". This is the test Ghidra's own class recovery uses to
     * find its tables, plus any name that ends in "vftable". The argument is the
     * label's own name, without its namespace.
     */
    public static boolean isVtableName(String name) {
        if (name == null || name.startsWith("vftable_meta_ptr")) {
            return false;
        }
        return name.endsWith("vftable") || (name.length() > 1 && name.substring(1).startsWith("vftable")) ||
            name.contains("vftable_for_") || name.contains("vftable{for");
    }

    /** The last component of a path written with either kind of separator. */
    public static String fileNameOf(String path) {
        if (path == null) {
            return "";
        }
        int i = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));
        return path.substring(i + 1);
    }

    /**
     * The file Ghidra recorded as the import source, or null when the text
     * is not a usable path. Ghidra writes Windows paths with a leading
     * slash ("/C:/..."), which Java cannot open on Windows, so that slash
     * is dropped.
     */
    public static Path executableFile(String recorded) {
        if (recorded == null || recorded.isEmpty()) {
            return null;
        }
        String t = recorded;
        if (t.length() > 2 && t.charAt(0) == '/' && Character.isLetter(t.charAt(1)) && t.charAt(2) == ':') {
            t = t.substring(1);
        }
        try {
            return Paths.get(t);
        }
        catch (InvalidPathException e) {
            return null;
        }
    }

    public static String sha256Hex(byte[] data, int off, int len) {
        try {
            MessageDigest md = MessageDigest.getInstance("SHA-256");
            md.update(data, off, len);
            return bytesToHex(md.digest());
        }
        catch (java.security.NoSuchAlgorithmException e) {
            throw new IllegalStateException(e);
        }
    }

    private static long rol32(long v, int n) {
        int k = n & 31;
        long x = v & 0xffffffffL;
        return ((x << k) | (x >>> (32 - k))) & 0xffffffffL;
    }

    /**
     * Decodes the Rich header from the bytes between the start of the file
     * and the PE signature. The header ends with "Rich" and an XOR key; the
     * entries before it are XORed with that key and start with "DanS". Each
     * entry is a product id, a build number and a use count.
     * {@code checksum_valid} says whether the key equals the checksum the
     * linker computes over the DOS header and the entries. Returns
     * {@code present=false} when there is no well-formed header.
     */
    public static Map<String, Object> parseRichHeader(byte[] head) {
        Map<String, Object> out = obj();
        out.put("present", false);
        for (int r = 0x40; r + 8 <= head.length; r += 4) {
            if (!(head[r] == 'R' && head[r + 1] == 'i' && head[r + 2] == 'c' && head[r + 3] == 'h')) {
                continue;
            }
            long key = le(head, r + 4, 4);
            int start = -1;
            for (int i = r - 4; i >= 0x40; i -= 4) {
                if ((le(head, i, 4) ^ key) == 0x536e6144L) { // "DanS"
                    start = i;
                    break;
                }
            }
            if (start < 0 || (r - (start + 16)) % 8 != 0 || r - (start + 16) < 0) {
                continue;
            }
            boolean padded = true;
            for (int k = 1; k <= 3; k++) {
                padded &= (le(head, start + 4 * k, 4) ^ key) == 0;
            }
            if (!padded) {
                continue;
            }
            List<Object> entries = new ArrayList<>();
            long sum = start;
            for (int i = 0; i < start; i++) {
                if (i >= 0x3c && i < 0x40) {
                    continue; // e_lfanew is not part of the checksum
                }
                sum += rol32(head[i] & 0xffL, i);
            }
            for (int i = start + 16; i < r; i += 8) {
                long comp = le(head, i, 4) ^ key;
                long count = le(head, i + 4, 4) ^ key;
                Map<String, Object> e = obj();
                e.put("product_id", comp >>> 16);
                e.put("build", comp & 0xffff);
                e.put("count", count);
                e.put("comp_id", String.format("0x%08x", comp));
                entries.add(e);
                sum += rol32(comp, (int) count);
            }
            out.put("present", true);
            out.put("offset", start);
            out.put("key", String.format("0x%08x", key));
            out.put("checksum_valid", (sum & 0xffffffffL) == key);
            out.put("entries", entries);
            return out;
        }
        return out;
    }

    public static void writeText(java.nio.file.Path path, String text) throws IOException {
        java.nio.file.Files.write(path, text.getBytes(StandardCharsets.UTF_8));
    }
}

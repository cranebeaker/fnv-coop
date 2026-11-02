// Test helper, not part of the shipped scripts: dumps the parts of a program
// that the Nv* scripts change but the export does not show.
//
// Args: out=<file.json> [addr=<hex>[,<hex>...]]
// Writes: the function count, every bookmark in category "nv-names", every
// function tag in use, and for each listed address the primary symbol (name,
// namespace, source), the function's tags and how many symbols share that
// name in the same namespace.
//
// @category NV

import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.listing.Bookmark;
import ghidra.program.model.listing.BookmarkManager;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.Symbol;

public class InspectNv extends GhidraScript {
    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs("InspectNv", getScriptArgs(), Set.of("out", "addr"));
        Map<String, Object> doc = NvCommon.obj();
        doc.put("function_count", currentProgram.getFunctionManager().getFunctionCount());

        List<Object> bms = new ArrayList<>();
        BookmarkManager bm = currentProgram.getBookmarkManager();
        Iterator<Bookmark> it = bm.getBookmarksIterator();
        while (it.hasNext()) {
            Bookmark b = it.next();
            if (!NvCommon.NV_NAMES_BOOKMARK.equals(b.getCategory())) {
                continue;
            }
            Map<String, Object> m = NvCommon.obj();
            m.put("address", NvCommon.addr(b.getAddress()));
            m.put("type", b.getTypeString());
            m.put("category", b.getCategory());
            m.put("comment", b.getComment());
            bms.add(m);
        }
        doc.put("nv_names_bookmarks", bms);

        Map<String, Object> tagCounts = NvCommon.obj();
        for (Function f : currentProgram.getFunctionManager().getFunctions(true)) {
            f.getTags().forEach(t -> tagCounts.merge(t.getName(), 1, (a, b) -> (Integer) a + (Integer) b));
        }
        doc.put("function_tag_counts", tagCounts);

        List<Object> syms = new ArrayList<>();
        if (args.containsKey("addr")) {
            for (String t : args.get("addr").split(",")) {
                Address a = toAddr(NvCommon.parseHex("InspectNv", "addr", t));
                Symbol s = currentProgram.getSymbolTable().getPrimarySymbol(a);
                Function fn = getFunctionAt(a);
                Map<String, Object> m = NvCommon.obj();
                m.put("address", NvCommon.addr(a));
                m.put("name", s == null ? null : s.getName());
                m.put("qualified_name", s == null ? null : s.getName(true));
                m.put("source", s == null ? null : s.getSource().name());
                m.put("is_function", fn != null);
                List<String> tags = new ArrayList<>();
                if (fn != null) {
                    fn.getTags().forEach(tag -> tags.add(tag.getName()));
                    Collections.sort(tags);
                }
                m.put("tags", tags);
                m.put("same_name_symbols", s == null ? 0 : currentProgram.getSymbolTable()
                    .getSymbols(s.getName(), s.getParentNamespace()).size());
                syms.add(m);
            }
        }
        doc.put("symbols", syms);
        NvCommon.writeText(Paths.get(NvCommon.require("InspectNv", args, "out")), NvCommon.toPrettyJson(doc));
        println("InspectNv: wrote " + args.get("out"));
    }
}

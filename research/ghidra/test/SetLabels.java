// Test helper, not part of the shipped scripts: puts labels with chosen names
// at chosen addresses, including names that NvImportNameMap refuses (a
// backtick or an apostrophe, as in PDB-style vtable names).
//
// Args: file=<text file>
// Each line is "<hex address> <namespace or -> <label name>". A namespace
// must already exist directly under the global namespace. A "+" in front of
// the address always adds a label. Without it, the label that already
// carries a user-defined name at that address is renamed instead.
//
// @category NV

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.symbol.Namespace;
import ghidra.program.model.symbol.SourceType;
import ghidra.program.model.symbol.Symbol;
import ghidra.program.model.symbol.SymbolTable;
import ghidra.program.model.symbol.SymbolType;

public class SetLabels extends GhidraScript {
    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs("SetLabels", getScriptArgs(), Set.of("file"));
        SymbolTable st = currentProgram.getSymbolTable();
        int n = 0;
        for (String raw : Files.readAllLines(Paths.get(NvCommon.require("SetLabels", args, "file")),
            StandardCharsets.UTF_8)) {
            String line = raw.trim();
            if (line.isEmpty()) {
                continue;
            }
            String[] p = line.split("\\s+", 3);
            if (p.length != 3) {
                throw new IllegalArgumentException("SetLabels: bad line '" + line + "'");
            }
            boolean add = p[0].startsWith("+");
            Address a = toAddr(NvCommon.parseHex("SetLabels", "address", add ? p[0].substring(1) : p[0]));
            Namespace ns = currentProgram.getGlobalNamespace();
            if (!p[1].equals("-")) {
                ns = st.getNamespace(p[1], currentProgram.getGlobalNamespace());
                if (ns == null) {
                    throw new IllegalArgumentException("SetLabels: no namespace " + p[1]);
                }
            }
            Symbol prim = st.getPrimarySymbol(a);
            if (!add && prim != null && prim.getSource() != SourceType.DEFAULT &&
                prim.getSymbolType() == SymbolType.LABEL) {
                prim.setNameAndNamespace(p[2], ns, SourceType.USER_DEFINED);
            }
            else {
                st.createLabel(a, p[2], ns, SourceType.USER_DEFINED);
            }
            n++;
        }
        println("SetLabels: " + n + " label(s) set");
    }
}

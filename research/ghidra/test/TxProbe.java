// Test helper, not part of the shipped scripts: shows that aborting a nested
// transaction makes Ghidra discard everything the script run changed, which
// NvImportNameMap and NvLabelCommandTables rely on as their last safety net.
//
// Args: addr=<hex>,<hex>   (two function entries)
// It renames the first function, then renames the second inside a nested
// transaction that it aborts. run-tests.sh then checks that neither rename
// survived in the saved project.
//
// @category NV

import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.symbol.SourceType;

public class TxProbe extends GhidraScript {
    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs("TxProbe", getScriptArgs(), Set.of("addr"));
        String[] parts = NvCommon.require("TxProbe", args, "addr").split(",");
        Function a = getFunctionAt(toAddr(NvCommon.parseHex("TxProbe", "addr", parts[0])));
        Function b = getFunctionAt(toAddr(NvCommon.parseHex("TxProbe", "addr", parts[1])));
        a.setName("TxProbeOuter", SourceType.USER_DEFINED);
        int tx = currentProgram.startTransaction("TxProbe inner");
        b.setName("TxProbeInner", SourceType.USER_DEFINED);
        currentProgram.endTransaction(tx, false);
        println("TxProbe: renamed both, inner transaction aborted");
    }
}

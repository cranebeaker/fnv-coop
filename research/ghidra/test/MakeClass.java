// Test helper, not part of the shipped scripts: turns a plain namespace into
// a Ghidra class, as Ghidra's RTTI analysis does for the classes it finds.
// NvImportNameMap creates plain namespaces, so the export's "class" field
// can only be tested after this.
//
// Args: name=<namespace>   (a namespace directly under the global namespace)
//
// @category NV

import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;
import ghidra.program.model.symbol.Namespace;

public class MakeClass extends GhidraScript {
    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs("MakeClass", getScriptArgs(), Set.of("name"));
        String name = NvCommon.require("MakeClass", args, "name");
        Namespace ns = currentProgram.getSymbolTable().getNamespace(name, currentProgram.getGlobalNamespace());
        if (ns == null) {
            throw new IllegalArgumentException("MakeClass: no namespace " + name);
        }
        currentProgram.getSymbolTable().convertNamespaceToClass(ns);
        println("MakeClass: " + name + " is now a class");
    }
}

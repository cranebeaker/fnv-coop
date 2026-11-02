// Creates functions at listed entry points, then at the call targets of
// the new functions, until no new target appears. Changes the project: run
// it on a copy, without -readOnly.
//
// Args: addrs=<file> report=<csv> [dry=1]
//
// <file> holds one hex address per line ('#' starts a comment). Typical
// input: the vtable slot targets that NvEngineMap reports with
// slot_is_function 0. Analysis that never saw a vtable slot or a call into
// such code leaves those functions out, so a ledger over Ghidra's
// functions would not count them.
//
// For each address (listed or found by following calls):
//   exists   a function already starts there; nothing changes.
//   inside   the address is inside another function's body; reported with
//            that function's entry and left alone (a wrong body or a
//            mid-function pointer needs a person to look).
//   created  disassembled from the address and made a function.
//   not-code the bytes do not decode as an instruction, or the address is
//            not in executable memory.
//   failed   Ghidra refused to create the function.
// The report has one row per address: address,origin,action,detail,size.
// origin is "list" or the entry of the new function whose call led there.
//
// The script uses one transaction; if anything throws, Ghidra discards all
// of its changes.
//
// @category NV

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;

import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.cmd.function.CreateFunctionCmd;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.symbol.FlowType;

public class NvCreateFunctions extends GhidraScript {
    private static final String NAME = "NvCreateFunctions";

    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs(NAME, getScriptArgs(), Set.of("addrs", "report", "dry"));
        Path in = Paths.get(NvCommon.require(NAME, args, "addrs"));
        Path report = Paths.get(NvCommon.require(NAME, args, "report"));
        boolean dry = NvCommon.flag(NAME, args, "dry", false);

        FunctionManager fm = currentProgram.getFunctionManager();
        Listing listing = currentProgram.getListing();
        Deque<Address[]> work = new ArrayDeque<>();
        for (String line : Files.readAllLines(in, StandardCharsets.UTF_8)) {
            String t = line.replaceAll("#.*", "").trim();
            if (t.isEmpty()) {
                continue;
            }
            work.add(new Address[] { toAddr(NvCommon.parseHex(NAME, "address", t)), null });
        }
        Map<Address, String[]> rows = new LinkedHashMap<>();
        Map<String, Integer> counts = new TreeMap<>();
        while (!work.isEmpty()) {
            monitor.checkCancelled();
            Address[] item = work.poll();
            Address a = item[0];
            if (rows.containsKey(a)) {
                continue;
            }
            String origin = item[1] == null ? "list" : NvCommon.addr(item[1]);
            String action;
            String detail = "";
            long size = 0;
            Function at = fm.getFunctionAt(a);
            Function containing = fm.getFunctionContaining(a);
            if (at != null) {
                action = "exists";
            }
            else if (containing != null) {
                action = "inside";
                detail = NvCommon.addr(containing.getEntryPoint());
            }
            else if (!NvCommon.inExecutableMemory(currentProgram, a)) {
                action = "not-code";
                detail = "not executable memory";
            }
            else if (dry) {
                action = "would-create";
            }
            else {
                if (listing.getInstructionAt(a) == null) {
                    DisassembleCommand dc = new DisassembleCommand(a, null, true);
                    dc.applyTo(currentProgram, monitor);
                }
                if (listing.getInstructionAt(a) == null) {
                    action = "not-code";
                    detail = "does not disassemble";
                }
                else {
                    CreateFunctionCmd cc = new CreateFunctionCmd(a);
                    Function f = cc.applyTo(currentProgram, monitor) ? fm.getFunctionAt(a) : null;
                    if (f == null) {
                        action = "failed";
                        detail = String.valueOf(cc.getStatusMsg()).replace(',', ';');
                    }
                    else {
                        action = "created";
                        size = f.getBody().getNumAddresses();
                        for (Instruction ins : listing.getInstructions(f.getBody(), true)) {
                            FlowType ft = ins.getFlowType();
                            if (!ft.isCall() && !(ft.isJump() && !ft.isConditional())) {
                                continue;
                            }
                            for (Address t : ins.getFlows()) {
                                if (!f.getBody().contains(t) && fm.getFunctionAt(t) == null &&
                                    (ft.isCall() || fm.getFunctionContaining(t) == null)) {
                                    work.add(new Address[] { t, a });
                                }
                            }
                        }
                    }
                }
            }
            counts.merge(action, 1, Integer::sum);
            rows.put(a, new String[] { NvCommon.addr(a), origin, action, detail, Long.toString(size) });
        }
        List<String> out = new ArrayList<>();
        out.add("address,origin,action,detail,size");
        for (String[] r : rows.values()) {
            out.add(String.join(",", r));
        }
        Files.write(report, out, StandardCharsets.UTF_8);
        println(NAME + ": " + counts);
    }
}

// Test helper, not part of the shipped scripts: runs small NvCommon functions
// on the inputs in a file, so check.py can compare them with an independent
// implementation.
//
// Args: cases=<file> out=<file>
// Each line of the cases file is "f80 <20 hex digits>" (ten bytes in memory
// order), "redact <text>" or "vtname <label name>". The output has one line
// per case:
//   f80 <hex> <raw bits of the double, hex> <21-digit decimal string>
//   redact <text> => <result>
//   vtname <label name> => true|false
//
// @category NV

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Set;

import ghidra.app.script.GhidraScript;

public class UnitProbe extends GhidraScript {
    @Override
    public void run() throws Exception {
        Map<String, String> args = NvCommon.parseArgs("UnitProbe", getScriptArgs(), Set.of("cases", "out"));
        List<String> out = new ArrayList<>();
        for (String line : Files.readAllLines(Paths.get(NvCommon.require("UnitProbe", args, "cases")),
            StandardCharsets.UTF_8)) {
            if (line.startsWith("f80 ")) {
                String hex = line.substring(4).trim();
                byte[] b = new byte[10];
                for (int i = 0; i < 10; i++) {
                    b[i] = (byte) Integer.parseInt(hex.substring(2 * i, 2 * i + 2), 16);
                }
                out.add("f80 " + hex + " " +
                    Long.toHexString(Double.doubleToRawLongBits(NvCommon.f80ToDouble(b))) + " " +
                    NvCommon.f80ToString(b));
            }
            else if (line.startsWith("redact ")) {
                String text = line.substring(7);
                out.add("redact " + text + " => " + NvCommon.redactUserPath(text));
            }
            else if (line.startsWith("vtname ")) {
                String text = line.substring(7);
                out.add("vtname " + text + " => " + NvCommon.isVtableName(text));
            }
        }
        Files.write(Paths.get(NvCommon.require("UnitProbe", args, "out")),
            (String.join("\n", out) + "\n").getBytes(StandardCharsets.UTF_8));
        println("UnitProbe: " + out.size() + " cases");
    }
}

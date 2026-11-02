#!/usr/bin/env python3
"""Looks up the FNV decompilation (a read only reference) for the GAME checklist.

As a module: `Index(root).find(keywords)` returns the units and functions
whose source path or symbol names match. Only paths, function names and exe
addresses are read out; nothing from the reference is copied into the repo.

The root is FNV_DECOMP; the default is the reference folder next to the repo
(`games/reference/fnv_decomp`).
"""
import json
import os
import re
import sys

DEFAULT = os.environ.get(
    "FNV_DECOMP",
    os.path.join(os.path.expanduser("~"), "Documents", "Workspace", "games", "reference", "fnv_decomp"),
)


def demangle(sym):
    """`?Name@Class@@...` -> `Class::Name`; free functions -> `Name`; None for shell stubs."""
    if "FUN_" in sym or not sym.startswith("?"):
        return None
    m = re.match(r"\?\??([A-Za-z_]\w*)@([A-Za-z_]\w*)@@", sym)
    if m:
        return f"{m.group(2)}::{m.group(1)}"
    m = re.match(r"\?([A-Za-z_]\w*)@@", sym)
    if m:
        return m.group(1)
    m = re.match(r"\?\?[0-9A-Z]([A-Za-z_]\w*)@", sym)
    return m.group(1) if m else None


class Index:
    def __init__(self, root=DEFAULT):
        self.root = root
        path = os.path.join(root, "build", "units.json")
        self.ok = os.path.exists(path)
        self.units = []
        if not self.ok:
            return
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
        for u in data["units"]:
            named = []
            shells = 0
            for fn in u["functions"]:
                d = demangle(fn["symbol"])
                if d:
                    named.append((fn["address"], d, fn["size"]))
                else:
                    shells += 1
            self.units.append({"source": u["source"], "named": named, "shells": shells})

    def find(self, keywords, max_units=3, max_funcs=6):
        """Rank units by how many named functions (and path hits) match any keyword."""
        if not self.ok:
            return {"units": [], "named": 0, "shells": 0, "matching_named_functions": 0}
        kws = [k.lower() for k in keywords]
        scored = []
        for u in self.units:
            path = u["source"].lower()
            hits = [(a, n, s) for a, n, s in u["named"] if any(k in n.lower() for k in kws)]
            pscore = sum(1 for k in kws if k in path)
            if hits or pscore:
                scored.append((len(hits) + 4 * pscore, u, hits))
        scored.sort(key=lambda t: -t[0])
        out = []
        tn = ts = 0
        for score, u, hits in scored[:max_units]:
            hits = sorted(hits, key=lambda h: -h[2])[:max_funcs]
            out.append({
                "path": u["source"],
                "functions": [{"name": n, "address": a} for a, n, _ in hits],
                "score": score,
            })
            tn += len(u["named"])
            ts += u["shells"]
        total_hits = sum(len(h) for _, _, h in scored)
        return {"units": out, "named": tn, "shells": ts, "matching_named_functions": total_hits}


if __name__ == "__main__":
    print(json.dumps(Index().find(sys.argv[1:]), indent=1))

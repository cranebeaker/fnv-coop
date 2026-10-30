#!/usr/bin/env python
"""Re-analyse a recorded benchmark run offline (no game): python -I tools/bench/rescore.py out/bench/<time> [--json]

Reads samples.jsonl with the CURRENT analysis and thresholds in coop_bench.py, so a metric or target added later can be applied to old runs,
and the two checked-in runs of a before/after comparison are always scored by the same code.
"""
import argparse, importlib.util, json, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load_bench():
    spec = importlib.util.spec_from_file_location("coop_bench", HERE / "coop_bench.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def rescore(folder: Path):
    cb = load_bench()
    rows = [json.loads(line) for line in open(folder / "samples.jsonl", encoding="utf-8")]
    analysis = cb.analyse(rows)
    extras = {}
    rj = folder / "results.json"
    if rj.is_file():   # playbook results and the area-change marker were recorded by the live run
        try:
            extras = json.loads(rj.read_text(encoding="utf-8")).get("extras", {}) or {}
        except Exception:
            extras = {}
        for v in extras.values():
            if isinstance(v, dict):
                v.pop("recovery_s", None)   # recompute from the samples with the current code
    extras = cb.finalize_extras(rows, extras)
    par = cb.parity_extras(folder)   # parity_host.jsonl / parity_client.jsonl saved by the run: scored again with the current rules
    if par is not None:
        extras = dict(extras or {}, parity=par)
    scores, cats, comp = cb.scorecard(analysis["overall"], analysis["phases"], extras)
    return {"folder": str(folder), "composite": comp, "categories": cats, "scores": scores, "extras": extras, **analysis}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("folder")
    ap.add_argument("--json", action="store_true")
    a = ap.parse_args()
    r = rescore(Path(a.folder))
    if a.json:
        print(json.dumps(r, indent=1))
        return 0
    print(f"{r['folder']}\ncomposite {r['composite']}  " + "  ".join(f"{k} {v}" for k, v in r["categories"].items()))
    for k, v in r["scores"].items():
        print(f"  {k:28} value {str(v['value']):>10}  score {str(v['score']):>6}")
    for k in ("sync.latency_ms_p50", "sync.latency_ms_p95", "sync.latency_ms_max", "copy0.bridge_rtt_ms_p95", "copy1.bridge_rtt_ms_p95",
              "copy0.ram_growth_mb_per_min", "copy1.ram_growth_mb_per_min", "sync.ever_converged"):
        if k in r["overall"]:
            print(f"  {k:28} {r['overall'][k]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

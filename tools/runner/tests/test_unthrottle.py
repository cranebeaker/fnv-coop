"""The test copies are minimized, and Windows 11 throttles a minimized process (timer resolution ignored, EcoQoS). The runner opts every copy it launches out of that
(winproc.unthrottle, called by run.launch_instance and again after the copy's windows are minimized). Twins (RUNNER_MUTANT): the state bits are set instead of cleared; a
launched copy is not opted out."""
from __future__ import annotations

import os
import subprocess
import sys
import time
import unittest
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
RUNNER = HERE.parent
sys.path.insert(0, str(RUNNER))

TWINS = {
    "throttle_state_set": ["Unthrottle.test_the_control_bits_are_set_and_the_state_bits_are_clear", "Unthrottle.test_a_real_process_reads_back_the_opt_out"],
    "launch_skips_unthrottle": ["Launch.test_a_launched_copy_is_opted_out_of_power_throttling_at_start_and_again_after_it_is_minimized"],
}
SLEEPER = [sys.executable, "-I", "-c", "import time; time.sleep(20)"]


@unittest.skipUnless(os.name == "nt", "Windows power throttling")
class Unthrottle(unittest.TestCase):
    def setUp(self):
        import winproc
        self.w = winproc

    def test_the_control_bits_are_set_and_the_state_bits_are_clear(self):
        with mock.patch.object(self.w, "_set_throttle", wraps=self.w._set_throttle) as spy:
            p = subprocess.Popen(SLEEPER)
            try:
                r = self.w.unthrottle(p.pid)
            finally:
                p.kill()
                p.wait()
        self.assertTrue(r["ok"], r)
        control, state = spy.call_args_list[0][0][1], spy.call_args_list[0][0][2]
        self.assertEqual(state, 0, "a SET state bit would opt the process in to the throttling")
        self.assertTrue(control & self.w.THROTTLE_EXECUTION_SPEED and control & self.w.THROTTLE_IGNORE_TIMER_RESOLUTION)

    def test_a_real_process_reads_back_the_opt_out(self):
        p = subprocess.Popen(SLEEPER)
        try:
            time.sleep(0.3)
            r = self.w.unthrottle(p.pid)
            st = self.w.throttle_state(p.pid)
        finally:
            p.kill()
            p.wait()
        self.assertTrue(r["ok"], r)
        self.assertTrue(st["control"] & self.w.THROTTLE_EXECUTION_SPEED, st)
        self.assertEqual(st["state"] & self.w.THROTTLE_EXECUTION_SPEED, 0, st)

    def test_a_pid_that_does_not_exist_is_a_failed_result_not_an_exception(self):
        r = self.w.unthrottle(0x7FFFFFF0)
        self.assertFalse(r["ok"])


class P:
    pid = 4242


@unittest.skipUnless(os.name == "nt", "Windows power throttling")
class Launch(unittest.TestCase):
    def launch(self, extra_env):
        import run
        import winproc
        calls = []
        env = {k: v for k, v in os.environ.items() if k not in ("NVCOOP_UNTHROTTLE", "NVCOOP_VISIBLE")}
        env.update(extra_env)
        with mock.patch.dict(os.environ, env, clear=True), \
                mock.patch.object(run.instances, "instance_dir", return_value=HERE / "nonexistent"), \
                mock.patch.object(run, "spawn_detached", return_value=P()), \
                mock.patch.object(run, "register_pid"), \
                mock.patch.object(winproc, "unthrottle", side_effect=lambda pid: calls.append(pid) or {"ok": True}), \
                mock.patch.object(winproc, "minimize_windows", side_effect=lambda pid: calls.append(("min", pid))), \
                mock.patch.object(winproc, "list_pids", return_value=[4242]), \
                mock.patch.object(run, "load_state", return_value={"pids": {}}):
            self.assertEqual(run.launch_instance(0, via_nvse=False), 4242)
            deadline = time.time() + 3
            while time.time() < deadline and not (("min", 4242) in calls and (calls.count(4242) >= 2 or extra_env.get("NVCOOP_UNTHROTTLE") == "0")):
                time.sleep(0.05)
            time.sleep(0.1)
        return calls

    def test_a_launched_copy_is_opted_out_of_power_throttling_at_start_and_again_after_it_is_minimized(self):
        calls = self.launch({})
        self.assertEqual(calls, [4242, ("min", 4242), 4242], "opted out at once, the windows minimized, opted out again after it")

    def test_nvcoop_unthrottle_0_keeps_the_old_behaviour_for_comparing_fps(self):
        calls = self.launch({"NVCOOP_UNTHROTTLE": "0"})
        self.assertNotIn(4242, calls)


class Twins(unittest.TestCase):
    def test_each_twin_is_caught(self):
        if os.environ.get("RUNNER_MUTANT"):
            self.skipTest("inside a twin run")
        for name, tests in TWINS.items():
            env = dict(os.environ, RUNNER_MUTANT=name)
            cp = subprocess.run([sys.executable, "-I", "-m", "unittest"] + [f"test_unthrottle.{t}" for t in tests], cwd=str(HERE), env=env, capture_output=True, text=True)
            self.assertNotEqual(cp.returncode, 0, f"twin {name} was not caught:\n{cp.stdout[-400:]}{cp.stderr[-400:]}")


if __name__ == "__main__":
    unittest.main()

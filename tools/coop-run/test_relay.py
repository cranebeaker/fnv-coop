import os
import socket
import tempfile
import threading
import time
import unittest

import chaos_relay as cr


def stream(profile, seed, n, label="g2h", step_ms=10):
    imp = cr.Impairment(cr.get_profile(profile), seed, label)
    return [imp.decide(i * step_ms) for i in range(n)]


class ProfileTable(unittest.TestCase):
    def test_names_match_coop_core(self):
        self.assertEqual(set(cr.PROFILE_NAMES), {"good_wifi", "bad_wifi", "mobile", "congested", "flaky", "partition"})
        self.assertIn("perfect", cr.PROFILES)

    def test_parameters_match_coop_core(self):
        # numbers copied from crates/coop-core/src/transport/profile.rs
        b = cr.PROFILES["bad_wifi"]
        self.assertEqual((b.latency_ms, b.jitter_ms, b.loss, b.dup, b.reorder, b.reorder_extra_ms),
                         (50, 40, 0.05, 0.01, 0.05, 80))
        f = cr.PROFILES["flaky"]
        self.assertEqual((f.latency_ms, f.jitter_ms, f.loss, f.dup, f.reorder, f.reorder_extra_ms),
                         (40, 30, 0.25, 0.05, 0.15, 100))
        p = cr.PROFILES["partition"]
        self.assertEqual((p.latency_ms, p.jitter_ms, p.loss, p.partition_period_ms, p.partition_len_ms),
                         (30, 10, 0.02, 6000, 1500))
        m = cr.PROFILES["mobile"]
        self.assertEqual((m.latency_ms, m.loss, m.reorder_extra_ms), (90, 0.03, 120))
        c = cr.PROFILES["congested"]
        self.assertEqual((c.latency_ms, c.loss, c.dup, c.reorder_extra_ms), (150, 0.10, 0.02, 200))
        g = cr.PROFILES["good_wifi"]
        self.assertEqual((g.latency_ms, g.jitter_ms, g.loss), (15, 5, 0.005))

    def test_partition_windows_match_coop_core_vectors(self):
        p = cr.PROFILES["partition"]
        self.assertIsNone(p.partition_end(0))
        self.assertIsNone(p.partition_end(4499))
        self.assertEqual(p.partition_end(4500), 6000)
        self.assertEqual(p.partition_end(5999), 6000)
        self.assertIsNone(p.partition_end(6000))
        self.assertEqual(p.partition_end(10600), 12000)
        self.assertIsNone(cr.PROFILES["bad_wifi"].partition_end(5000))

    def test_unknown_profile(self):
        with self.assertRaises(KeyError):
            cr.get_profile("hell")


class Model(unittest.TestCase):
    def test_perfect_is_transparent(self):
        for v in stream("perfect", 1, 500):
            self.assertEqual(v.delays_ms, [0.0])

    def test_seed_is_reproducible_and_seeds_differ(self):
        a = [(v.reason, v.delays_ms) for v in stream("bad_wifi", 7, 2000)]
        b = [(v.reason, v.delays_ms) for v in stream("bad_wifi", 7, 2000)]
        c = [(v.reason, v.delays_ms) for v in stream("bad_wifi", 8, 2000)]
        self.assertEqual(a, b)
        self.assertNotEqual(a, c)

    def test_directions_are_independent_streams(self):
        a = [v.reason for v in stream("flaky", 3, 500, "g2h")]
        b = [v.reason for v in stream("flaky", 3, 500, "h2g")]
        self.assertNotEqual(a, b)

    def test_loss_rates(self):
        for name, lo, hi in (("good_wifi", 0.002, 0.009), ("bad_wifi", 0.04, 0.06), ("flaky", 0.22, 0.28),
                             ("congested", 0.08, 0.12), ("mobile", 0.02, 0.04)):
            vs = stream(name, 5, 30000)
            rate = sum(v.dropped for v in vs) / len(vs)
            self.assertTrue(lo < rate < hi, "%s loss %.4f" % (name, rate))

    def test_dup_and_reorder_happen(self):
        vs = stream("flaky", 9, 20000)
        self.assertGreater(sum(v.duplicated for v in vs), 300)
        self.assertGreater(sum(v.reordered for v in vs), 1000)
        self.assertTrue(all(len(v.delays_ms) == 2 for v in vs if v.duplicated))

    def test_latency_bounds(self):
        for v in stream("bad_wifi", 2, 5000):
            for d in v.delays_ms:
                self.assertTrue(50 <= d <= 50 + 40 + 80)

    def test_partition_blocks_traffic_in_window(self):
        imp = cr.Impairment(cr.get_profile("partition"), 1, "g2h")
        for t in range(4500, 6000, 50):
            self.assertEqual(imp.decide(t).reason, "partition")
        self.assertNotEqual(imp.decide(3000).reason, "partition")

    def test_outage(self):
        imp = cr.Impairment(cr.get_profile("perfect"), 1, "g2h")
        imp.outage(5000)
        self.assertEqual(imp.decide(4999).reason, "outage")
        self.assertFalse(imp.decide(5000).dropped)


class FakeClock:
    def __init__(self):
        self.t = 100.0

    def __call__(self):
        return self.t


class Schedule(unittest.TestCase):
    def test_profile_switch_and_outage_after_arming(self):
        clk = FakeClock()
        r = cr.ChaosRelay(0, ("127.0.0.1", 9), "flaky", 1, [
            {"at": 2, "profile": "perfect"}, {"at": 5, "outage": "both", "seconds": 1}], clock=clk)
        r.imp  # noqa
        # nothing happens before the first guest datagram
        clk.t += 10
        r._apply_schedule()
        self.assertEqual(r.profile_name, "flaky")
        r.handle("g2h", b"x", ("127.0.0.1", 1))
        self.assertTrue(r.stats["armed"])
        clk.t += 2.5
        r.handle("g2h", b"x", ("127.0.0.1", 1))
        self.assertEqual(r.profile_name, "perfect")
        clk.t += 3
        v = r.handle("g2h", b"x", ("127.0.0.1", 1))
        self.assertEqual(v.reason, "outage")
        clk.t += 1.2
        self.assertFalse(r.handle("h2g", b"x", ("127.0.0.1", 1)).dropped)
        evs = [e["ev"] for e in r.stats["events"]]
        self.assertEqual(evs, ["armed", "profile", "outage"])


class Sockets(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()

    def tearDown(self):
        self.tmp.cleanup()

    def start(self, profile, seed=1, schedule=None):
        host = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        host.bind(("127.0.0.1", 0))
        host.settimeout(2.0)
        stats = os.path.join(self.tmp.name, "s.json")
        r = cr.ChaosRelay(0, ("127.0.0.1", host.getsockname()[1]), profile, seed, schedule, stats_path=stats)
        port = r.open()
        ev = threading.Event()
        th = threading.Thread(target=r.run, kwargs={"stop_event": ev})
        th.start()
        self.addCleanup(lambda: (ev.set(), th.join(), host.close()))
        cli = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        cli.bind(("127.0.0.1", 0))
        cli.settimeout(2.0)
        self.addCleanup(cli.close)
        return r, host, cli, port

    def test_round_trip_perfect(self):
        r, host, cli, port = self.start("perfect")
        cli.sendto(b"hello", ("127.0.0.1", port))
        data, addr = host.recvfrom(100)
        self.assertEqual(data, b"hello")
        host.sendto(b"world", addr)
        data, _ = cli.recvfrom(100)
        self.assertEqual(data, b"world")
        self.assertEqual(r.stats["g2h"]["forwarded"], 1)
        self.assertEqual(r.stats["h2g"]["forwarded"], 1)

    def test_latency_is_applied(self):
        r, host, cli, port = self.start("congested")
        t = time.monotonic()
        cli.sendto(b"x", ("127.0.0.1", port))
        for _ in range(20):  # congested may drop; retry a few
            try:
                host.recvfrom(100)
                break
            except socket.timeout:
                cli.sendto(b"x", ("127.0.0.1", port))
        self.assertGreater(time.monotonic() - t, 0.14)

    def test_flaky_really_drops_over_udp(self):
        r, host, cli, port = self.start("flaky", seed=4)
        for i in range(300):
            cli.sendto(b"%d" % i, ("127.0.0.1", port))
            time.sleep(0.002)
        time.sleep(0.6)
        host.settimeout(0.2)
        got = 0
        try:
            while True:
                host.recvfrom(100)
                got += 1
        except socket.timeout:
            pass
        st = r.stats["g2h"]
        self.assertEqual(st["in"], 300)
        self.assertGreater(st["dropped"], 40)
        self.assertLess(st["dropped"], 120)
        self.assertGreater(got, 150)
        self.assertLess(got, 300 + st["duplicated"])


if __name__ == "__main__":
    unittest.main()

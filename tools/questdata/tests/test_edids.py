"""tools/questdata/edids.py: EDID -> 'Plugin.esm:LOCALID' over synthetic masters (read-only sweep, case-insensitive, defining plugin wins, ambiguity reported)."""
import os
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, '..'))
sys.path.insert(0, HERE)

import edids  # noqa: E402
import synth  # noqa: E402

ORDER = ['FalloutNV.esm', 'DeadMoney.esm']


def write_game(td, base_extra=(), dlc_extra=()):
    s = synth
    base = s.tes4() + s.grup('PERK', [s.rec('PERK', 0x0015D2BE, [s.sub('EDID', s.z('OldVaquero'))], flags=0x40000),
                                      s.rec('PERK', 0x0015B8E8, [s.sub('EDID', s.z('RangerTakedown'))]),
                                      s.rec('PERK', 0x00100001, [s.sub('EDID', s.z('GoneForever'))], flags=0x20),
                                      s.rec('PERK', 0x00100002, [s.sub('FULL', s.z('No Editor Id'))])]) \
        + s.grup('CELL', [s.rec('CELL', 0x00133880, [s.sub('EDID', s.z('HooverDamIntCheckPoint'))])] + list(base_extra))
    dlc = s.tes4(masters=('FalloutNV.esm',)) + s.grup('PERK', [s.rec('PERK', 0x0015B8E8, [s.sub('EDID', s.z('RangerTakedown'))]),
                                                              s.rec('PERK', 0x01000200, [s.sub('EDID', s.z('DeadPerk'))])]) \
        + s.grup('MESG', [s.rec('MESG', 0x01009C99, [s.sub('EDID', s.z('TempleBox')), s.sub('ITXT', s.z('No.')), s.sub('ITXT', s.z('Yes.'))])] + list(dlc_extra))
    with open(os.path.join(td, 'FalloutNV.esm'), 'wb') as f:
        f.write(base)
    with open(os.path.join(td, 'DeadMoney.esm'), 'wb') as f:
        f.write(dlc)


class EdidTests(unittest.TestCase):
    def find(self, names, **kw):
        with tempfile.TemporaryDirectory() as td:
            write_game(td, **kw)
            return edids.find(td, names, plugins=ORDER)

    def test_case_insensitive_compressed_override_and_dlc_forms(self):
        found, amb = self.find(['OldVaquero', 'HooverDamIntCheckpoint', 'RangerTakedown', 'DeadPerk', 'TempleBox', 'Nothing'])
        self.assertEqual(found['OldVaquero'], 'FalloutNV.esm:15D2BE')                 # a compressed record
        self.assertEqual(found['HooverDamIntCheckpoint'], 'FalloutNV.esm:133880')    # keyed by the spelling asked for
        self.assertEqual(found['RangerTakedown'], 'FalloutNV.esm:15B8E8')            # the DLC override resolves to the defining plugin
        self.assertEqual(found['DeadPerk'], 'DeadMoney.esm:000200')
        self.assertEqual(found['TempleBox'], 'DeadMoney.esm:009C99')
        self.assertNotIn('Nothing', found)
        self.assertEqual(amb, {})

    def test_deleted_records_and_records_without_an_edid_are_skipped(self):
        found, _ = self.find(['GoneForever', 'No Editor Id'])
        self.assertEqual(found, {})

    def test_two_different_forms_for_one_edid_use_the_first_in_load_order_and_are_reported(self):
        dup = synth.rec('CELL', 0x00133999, [synth.sub('EDID', synth.z('hooverdamintcheckpoint'))])
        found, amb = self.find(['HooverDamIntCheckpoint'], base_extra=[dup])
        self.assertEqual(found['HooverDamIntCheckpoint'], 'FalloutNV.esm:133880')
        self.assertEqual(amb['HooverDamIntCheckpoint'], ['FalloutNV.esm:133880', 'FalloutNV.esm:133999'])

    def test_a_missing_plugin_is_skipped_and_the_data_folder_is_never_written(self):
        with tempfile.TemporaryDirectory() as td:
            write_game(td)
            os.remove(os.path.join(td, 'DeadMoney.esm'))
            before = sorted(os.listdir(td))
            found, _ = edids.find(td, ['OldVaquero', 'DeadPerk'], plugins=ORDER)
            self.assertEqual(sorted(os.listdir(td)), before)
            self.assertIn('OldVaquero', found)
            self.assertNotIn('DeadPerk', found)

    def test_mesg_buttons_in_order(self):
        with tempfile.TemporaryDirectory() as td:
            write_game(td)
            self.assertEqual(edids.mesg_buttons(td, 'DeadMoney.esm:009C99'), ['No.', 'Yes.'])
            self.assertIsNone(edids.mesg_buttons(td, 'DeadMoney.esm:000001'))
            self.assertIsNone(edids.mesg_buttons(td, 'Nope.esm:000001'))

    def test_mutation_a_case_sensitive_sweep_misses_the_cell(self):
        real = edids.sweep
        try:
            edids.sweep = lambda path, wanted: [r for r in real(path, wanted) if r[0] == wanted[r[0].lower()]]
            found, _ = self.find(['HooverDamIntCheckpoint'])
        finally:
            edids.sweep = real
        self.assertNotIn('HooverDamIntCheckpoint', found)   # the mutant loses the name the rules spell differently: test_case_insensitive... would fail with it

    def test_mutation_an_override_that_keeps_the_dlc_ref_is_caught(self):
        found, _ = self.find(['RangerTakedown'])
        self.assertNotEqual(found['RangerTakedown'], 'DeadMoney.esm:15B8E8')


if __name__ == '__main__':
    unittest.main()

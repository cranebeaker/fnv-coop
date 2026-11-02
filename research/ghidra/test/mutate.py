#!/usr/bin/env python3
"""Copies a source file with one exact text replaced.

Usage: mutate.py <in> <out> <anchor> <replacement>

The tests use this to make a broken copy of a script (for example one that
throws halfway through) and check that the real script's safety nets hold.
It fails unless the anchor text occurs exactly once, so a script change that
moves the anchor cannot silently turn the test into a no-op.
"""

import sys


def main():
    if len(sys.argv) != 5:
        print(__doc__)
        return 2
    src, dst, anchor, replacement = sys.argv[1:]
    text = open(src, encoding="utf-8").read()
    n = text.count(anchor)
    if n != 1:
        print("mutate.py: anchor found %d times in %s (need exactly 1): %r" % (n, src, anchor))
        return 1
    open(dst, "w", encoding="utf-8").write(text.replace(anchor, replacement))
    return 0


if __name__ == "__main__":
    sys.exit(main())

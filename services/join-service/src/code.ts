// Join codes: 6 chars from a 28 symbol alphabet without look-alikes.
// Dropped: 0 O 1 I L U S 5 (digit/letter and U/V confusions). 28^6 is about 482 million codes.
export const ALPHABET = "ABCDEFGHJKMNPQRTVWXYZ2346789";
export const CODE_LENGTH = 6;

/** Rejection sampling over random bytes so every symbol is equally likely. */
export function generateCode(randomBytes: (n: number) => Uint8Array): string {
  const n = ALPHABET.length;
  const limit = 256 - (256 % n);
  let out = "";
  while (out.length < CODE_LENGTH) {
    const buf = randomBytes(16);
    for (let i = 0; i < buf.length && out.length < CODE_LENGTH; i++) {
      if (buf[i] < limit) out += ALPHABET[buf[i] % n];
    }
  }
  return out;
}

/** Uppercase and strip spaces/dashes people type; returns null if invalid. */
export function normalizeCode(input: string): string | null {
  const c = input.replace(/[\s-]/g, "").toUpperCase();
  if (c.length !== CODE_LENGTH) return null;
  for (const ch of c) if (!ALPHABET.includes(ch)) return null;
  return c;
}

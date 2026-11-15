//! The running game program, seen by address.

/// What a call returned: the integer result and the float result.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ret {
    /// `EAX`: an integer or a pointer. A `bool` is its low byte.
    pub eax: u32,
    /// `ST0`: a float result.
    pub st0: f64,
}

impl Ret {
    /// The low byte as a `bool`, as the program's compiler returns one.
    pub fn bool(self) -> bool {
        self.eax as u8 != 0
    }
}

/// The running game program (FalloutNV.exe 1.4.0.525), by address.
///
/// Use it only between the game's frames, on the thread that runs them.
pub trait Program {
    /// Reads `out.len()` bytes at `addr`.
    fn read(&self, addr: u32, out: &mut [u8]);
    /// Writes `bytes` at `addr`.
    fn write(&mut self, addr: u32, bytes: &[u8]);
    /// Calls the function at `addr`. The arguments are 32-bit words in declaration order, with
    /// `this` first for a method.
    fn call(&mut self, addr: u32, args: &[u32]) -> Ret;
    /// Program memory for an argument (a string, a small object): `len` bytes, not cleared.
    fn alloc(&mut self, len: u32) -> u32;
    /// Gives memory from [`Program::alloc`] back.
    fn free(&mut self, addr: u32);
    /// Puts the guest gate on this program with these rules, and runs its start checks
    /// (`shared_quest`: the address of a quest that the rules do not make local). `Err` tells
    /// why the program cannot take the gate: the session must not go on then.
    fn install_gate(
        &mut self,
        _rules: &crate::gate::Rules,
        _shared_quest: Option<u32>,
    ) -> Result<(), String> {
        Err("this program takes no guest gate".into())
    }

    /// The adapter starts (`true`) or ends (`false`) applying another machine's state. On a
    /// guest's game no script runs inside the bracket (the host ran them, and their effects
    /// come as state), and the adapter's own change of a quest passes the gate. Brackets nest.
    /// A program with no gate has nothing to do.
    fn applying(&mut self, _on: bool) {}

    /// One byte at `addr`.
    fn u8(&self, addr: u32) -> u8 {
        let mut b = [0; 1];
        self.read(addr, &mut b);
        b[0]
    }
    /// A 32-bit word at `addr`.
    fn u32(&self, addr: u32) -> u32 {
        let mut b = [0; 4];
        self.read(addr, &mut b);
        u32::from_le_bytes(b)
    }
    /// A float at `addr`.
    fn f32(&self, addr: u32) -> f32 {
        f32::from_bits(self.u32(addr))
    }
    /// Writes one byte at `addr`.
    fn set_u8(&mut self, addr: u32, v: u8) {
        self.write(addr, &[v]);
    }
    /// Writes a 32-bit word at `addr`.
    fn set_u32(&mut self, addr: u32, v: u32) {
        self.write(addr, &v.to_le_bytes());
    }
    /// Writes a float at `addr`.
    fn set_f32(&mut self, addr: u32, v: f32) {
        self.set_u32(addr, v.to_bits());
    }
    /// Writes `text` and a zero byte at `addr`.
    fn set_cstr(&mut self, addr: u32, text: &[u8]) {
        self.write(addr, text);
        self.set_u8(addr + text.len() as u32, 0);
    }
}

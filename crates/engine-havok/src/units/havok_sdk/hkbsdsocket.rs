//! `Havok SDK/hkbsdsocket.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00da33d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00da_3080`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00da33d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00da_3080, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The Winsock functions, reached through the import thunks (all stdcall): `closesocket(s)`,
/// `socket(af, type, protocol)`, `WSAGetLastError()`, `recv(s, buffer, length, flags)`,
/// `send(s, buffer, length, flags)`, `connect(s, address, length)`, `gethostbyname(name)`,
/// `inet_addr(text)`, `htons(value)`, `WSAAsyncSelect(s, window, message, events)`,
/// `WSAStartup(version, data)`, `gethostname(buffer, length)`, `listen(s, backlog)`,
/// `bind(s, address, length)`, `setsockopt(s, level, name, value, length)`,
/// `inet_ntoa(address)`, `ntohs(value)`, `accept(s, address, &length)`,
/// `__WSAFDIsSet(s, set)` and `select(count, read, write, except, timeout)`.
const CLOSE_SOCKET: u32 = 0x00ee_6d44;
const CREATE_SOCKET: u32 = 0x00ee_6d4a;
const LAST_ERROR: u32 = 0x00ee_6d50;
const RECEIVE: u32 = 0x00ee_6d56;
const SEND: u32 = 0x00ee_6d5c;
const CONNECT: u32 = 0x00ee_6d62;
const GET_HOST_BY_NAME: u32 = 0x00ee_6d68;
const INET_ADDR: u32 = 0x00ee_6d6e;
const HOST_TO_NET_SHORT: u32 = 0x00ee_6d74;
const ASYNC_SELECT: u32 = 0x00ee_6d7a;
const NETWORK_STARTUP: u32 = 0x00ee_6d86;
const GET_HOST_NAME: u32 = 0x00ee_6d8c;
const LISTEN: u32 = 0x00ee_6d92;
const BIND: u32 = 0x00ee_6d98;
const SET_SOCKET_OPTION: u32 = 0x00ee_6d9e;
const INET_NTOA: u32 = 0x00ee_6da4;
const NET_TO_HOST_SHORT: u32 = 0x00ee_6daa;
const ACCEPT: u32 = 0x00ee_6db0;
const IS_SET: u32 = 0x00ee_6db6;
const SELECT: u32 = 0x00ee_6dbc;
/// `memset` (`00cb1280`, cdecl `(destination, value, bytes)`) and `memcpy` (`00cb1260`, cdecl
/// `(destination, source, bytes)`).
const MEM_SET: u32 = 0x00cb_1280;
const MEM_COPY: u32 = 0x00cb_1260;
/// The base constructor `00d2b850` the constructor runs on `this` first (the class is unconfirmed).
const BASE_CONSTRUCT: u32 = 0x00d2_b850;
/// The text builder the log messages use: constructor `00cb1600` `(this, buffer, size)`,
/// `operator<<(const char*)` `00cc01c0` and `operator<<(hkStringBuf const&)` `00cc05f0`
/// (each `(this, argument)`, answering the builder), `operator<<(int)` `00cc0280` and the
/// destructor `00cc00e0`.
const TEXT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const TEXT_BUILDER_APPEND: u32 = 0x00cc_01c0;
const TEXT_BUILDER_APPEND_STRING_BUF: u32 = 0x00cc_05f0;
const TEXT_BUILDER_APPEND_INT: u32 = 0x00cc_0280;
const TEXT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
/// `hkStringBuf`: constructor `00cbbd80` and destructor `00cbbd40` (thiscall on the object),
/// the assignment from a C string `00d61c10` (`(this, text)`) and the cdecl formatter
/// `00d61d00` `(buffer, format, ...)`.
const STRING_BUF_CONSTRUCT: u32 = 0x00cb_bd80;
const STRING_BUF_DESTRUCT: u32 = 0x00cb_bd40;
const STRING_BUF_ASSIGN: u32 = 0x00d6_1c10;
const STRING_BUF_FORMAT: u32 = 0x00d6_1d00;
/// The global with the report sink whose slot `0xc` takes `(level, id, text, file, line)`, and
/// the source file name (".\System\Io\Socket\Bsd\hkBsdSocket.cpp") it is given.
const REPORT_SINK: u32 = 0x0126_8360;
const SOURCE_FILE: u32 = 0x010d_4fdc;
/// The texts of the log messages: "(Windows)WSAStartup failed with error!", "Listening on
/// host[", "] port " and "Socket got connection from [%s:%d]".
const TEXT_STARTUP_FAILED: u32 = 0x010d_5004;
const TEXT_LISTENING: u32 = 0x010d_505c;
const TEXT_PORT: u32 = 0x010d_5070;
const FORMAT_CONNECTION: u32 = 0x010d_5078;
/// The byte that is set once `WSAStartup` has run.
const NETWORK_STARTED: u32 = 0x0126_9acc;
/// The vtable of `hkBsdSocket` (RTTI name confirmed in the exe) and the base vtable the
/// destructor leaves in the object.
const VTABLE: u32 = 0x010d_5030;
const BASE_VTABLE: u32 = 0x0102_e388;
/// Field `+0x20` of the socket object: the Winsock handle, `-1` when there is none.
const HANDLE: u32 = 0x20;
const NO_SOCKET: u32 = 0xffff_ffff;
/// `WSAEWOULDBLOCK`, the one error that does not close the socket.
const WOULD_BLOCK: u32 = 0x2733;

// Translated from 00da2db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::isOk` (Xbox PDB): writes 1 to the byte at `out` when the handle at `+0x20` is
/// not `-1`, else 0, and returns `out`.
pub fn hk_bsd_socket_is_ok(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let open = e.mem.u32(this.addr() + HANDLE) != NO_SOCKET;
    e.mem.set_u8(out, u8::from(open));
    out
}

// Translated from 00da2dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::close` (Xbox PDB): when the handle at `+0x20` is not `-1` it is closed with
/// `closesocket` and set to `-1`.
pub fn hk_bsd_socket_close(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr() + HANDLE);
    if handle != NO_SOCKET {
        e.call(CLOSE_SOCKET, &args![handle]);
        e.mem.set_u32(this.addr() + HANDLE, NO_SOCKET);
    }
}

// Translated from 00da2de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual slot `0x10` of the object (`close`), then creates a TCP socket
/// (`socket(2, 1, 0)`) and stores it at `+0x20`. Answers whether the creation failed (the
/// handle is `-1`). Unnamed in the exe.
pub fn open_socket_00da2de0(e: &mut Engine, this: Ptr) -> bool {
    e.vcall(this.addr(), 0x10, &args![]);
    let handle = e.call(CREATE_SOCKET, &args![2u32, 1u32, 0u32]).u32();
    e.mem.set_u32(this.addr() + HANDLE, handle);
    handle == NO_SOCKET
}

// Translated from 00da2e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::read` (Xbox PDB): without a handle (`-1` at `+0x20`) it answers 0. Otherwise
/// `recv(handle, buffer, length, 0)`; a positive count is the result. On 0 or an error
/// `WSAGetLastError` is asked and every error but `WSAEWOULDBLOCK` (0x2733) calls virtual slot
/// `0x10` (`close`); the result is then 0.
pub fn hk_bsd_socket_read(e: &mut Engine, this: Ptr, buffer: u32, length: u32) -> i32 {
    let handle = e.mem.u32(this.addr() + HANDLE);
    if handle == NO_SOCKET {
        return 0;
    }
    let count = e.call(RECEIVE, &args![handle, buffer, length, 0u32]).i32();
    // The exe also compares with -1 after the "positive" test; that is always true there.
    if count > 0 && count != -1 {
        return count;
    }
    if e.call(LAST_ERROR, &args![]).u32() != WOULD_BLOCK {
        e.vcall(this.addr(), 0x10, &args![]);
    }
    0
}

// Translated from 00da2e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::write` (Xbox PDB): the same as [`hk_bsd_socket_read`] with `send` in place of
/// `recv`.
pub fn hk_bsd_socket_write(e: &mut Engine, this: Ptr, buffer: u32, length: u32) -> i32 {
    let handle = e.mem.u32(this.addr() + HANDLE);
    if handle == NO_SOCKET {
        return 0;
    }
    let count = e.call(SEND, &args![handle, buffer, length, 0u32]).i32();
    // The exe also compares with -1 after the "positive" test; that is always true there.
    if count > 0 && count != -1 {
        return count;
    }
    if e.call(LAST_ERROR, &args![]).u32() != WOULD_BLOCK {
        e.vcall(this.addr(), 0x10, &args![]);
    }
    0
}

// Translated from 00da2eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register-input function (unnamed in the exe): `ECX` = `character` (a character code, sign
/// extended), `EAX` = `out`. Writes 1 to the byte at `out` when the code is a decimal digit
/// (`code - 0x30` is at most 9 unsigned), else 0, and leaves `EAX` = `out` (returned).
pub fn is_digit_00da2eb0(e: &mut Engine, character: u32, out: u32) -> u32 {
    let digit = character.wrapping_sub(0x30) <= 9;
    e.mem.set_u8(out, u8::from(digit));
    out
}

// Translated from 00da2ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::connect` (Xbox PDB): `host` is a C string and `port` the stack word whose
/// low 16 bits go to `htons`. Builds a zeroed 16-byte `sockaddr_in` (family 2, the port in
/// network order); a host whose first character is a digit goes through `inet_addr`, any other
/// through `gethostbyname` (a null result answers 1; the first address of the entry, of the
/// length at entry `+0xa`, is copied into the address). Without a handle (`-1`) the socket is
/// opened first (a failure answers 1). Then `connect(handle, address, 16)`: success answers 0;
/// on `-1` the error `WSAEWOULDBLOCK` (0x2733) also answers 0, any other error calls virtual
/// slot `0x10` (`close`) and answers 1.
pub fn hk_bsd_socket_connect(e: &mut Engine, this: Ptr, host: u32, port: u32) -> u32 {
    e.with_stack(0x10, |e, address| {
        let address = address.addr();
        e.call(MEM_SET, &args![address, 0u32, 0x10u32]);
        e.mem.set_u16(address, 2);
        let network_port = e.call(HOST_TO_NET_SHORT, &args![port]).u16();
        e.mem.set_u16(address + 2, network_port);
        let first = e.mem.u8(host) as i8 as i32 as u32;
        let numeric = e.with_stack(4, |e, flag| {
            is_digit_00da2eb0(e, first, flag.addr());
            e.mem.u8(flag.addr()) != 0
        });
        if numeric {
            let value = e.call(INET_ADDR, &args![host]).u32();
            e.mem.set_u32(address + 4, value);
        } else {
            let entry = e.call(GET_HOST_BY_NAME, &args![host]).u32();
            if entry == 0 {
                return 1;
            }
            let length = e.mem.u16(entry + 0xa) as i16 as i32 as u32;
            let list = e.mem.u32(entry + 0xc);
            let first_address = e.mem.u32(list);
            e.call(MEM_COPY, &args![address + 4, first_address, length]);
        }
        if e.mem.u32(this.addr() + HANDLE) == NO_SOCKET && open_socket_00da2de0(e, this) {
            return 1;
        }
        let handle = e.mem.u32(this.addr() + HANDLE);
        let result = e.call(CONNECT, &args![handle, address, 0x10u32]).i32();
        if result != -1 {
            return 0;
        }
        if e.call(LAST_ERROR, &args![]).u32() == WOULD_BLOCK {
            return 0;
        }
        e.vcall(this.addr(), 0x10, &args![]);
        1
    })
}

// Translated from 00da2f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::asyncSelect` (Xbox PDB): `WSAAsyncSelect(handle, window, message, events)`
/// where the event word is `((events & 0xc) << 2) | (events & 3)`. Answers whether the Winsock
/// result is non-zero.
pub fn hk_bsd_socket_async_select(
    e: &mut Engine,
    this: Ptr,
    window: u32,
    message: u32,
    events: u32,
) -> bool {
    let flags = ((events & 0xc) << 2) | (events & 3);
    let handle = e.mem.u32(this.addr() + HANDLE);
    e.call(ASYNC_SELECT, &args![handle, window, message, flags])
        .u32()
        != 0
}

// Translated from 00da2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdNetworkInit` (Xbox PDB): does nothing once the byte at `01269acc` is set. Otherwise
/// `WSAStartup(0x202, data)`; a result of `-1` builds the message "(Windows)WSAStartup failed
/// with error!" in a 512-byte text builder and hands it to the report sink (slot `0xc`: level
/// 3, id `0x321825f8`, the source file, line `0x48`). A non-zero answer of the sink is an
/// `INT3` in the exe (at `00da305e`) and panics here; otherwise the builder is destroyed.
/// The byte is then set.
pub fn hk_bsd_network_init(e: &mut Engine) {
    if e.mem.u8(NETWORK_STARTED) != 0 {
        return;
    }
    // The exe's frame: the text builder at +0, the `WSADATA` (0x190 bytes) at +0xc and the
    // 512-byte message buffer at +0x19c.
    e.with_stack(0x39c, |e, frame| {
        let builder = frame.addr();
        let data = builder + 0xc;
        let message = builder + 0x19c;
        let result = e.call(NETWORK_STARTUP, &args![0x202u32, data]).i32();
        if result == -1 {
            e.call(TEXT_BUILDER_CONSTRUCT, &args![builder, message, 0x200u32]);
            e.call(TEXT_BUILDER_APPEND, &args![builder, TEXT_STARTUP_FAILED]);
            let sink = e.mem.u32(REPORT_SINK);
            let answer = e
                .vcall(
                    sink,
                    0xc,
                    &args![3u32, 0x3218_25f8u32, message, SOURCE_FILE, 0x48u32],
                )
                .u32();
            if answer != 0 {
                panic!("INT3 at 00da305e: the report sink asked for a break");
            }
            e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
        }
    });
    e.mem.set_u8(NETWORK_STARTED, 1);
}

// Translated from 00da3080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the socket object (unnamed in the exe): stores the socket vtable
/// (`010d5030`), closes the socket ([`hk_bsd_socket_close`]) and sets the fields at `+0x14`,
/// `+8` and `+0` to the base vtable `0102e388`.
pub fn destructor_00da3080(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    hk_bsd_socket_close(e, this);
    e.mem.set_u32(t + 0x14, BASE_VTABLE);
    e.mem.set_u32(t + 8, BASE_VTABLE);
    e.mem.set_u32(t, BASE_VTABLE);
}

// Translated from 00da30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the socket object (unnamed in the exe): runs the base constructor
/// (`00d2b850`) on `this`, stores the socket vtable and the `handle` at `+0x20`; with `-1`
/// the socket is opened ([`open_socket_00da2de0`], its result ignored). Returns `this`.
pub fn construct_00da30a0(e: &mut Engine, this: Ptr, handle: u32) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr() + HANDLE, handle);
    if handle == NO_SOCKET {
        open_socket_00da2de0(e, this);
    }
    this
}

// Translated from 00da30d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::listen` (Xbox PDB): opens the socket ([`open_socket_00da2de0`]; a failure
/// answers 1), builds a `sockaddr_in` for any address on `htons(port)`, sets option
/// `SOL_SOCKET`/4 (`SO_REUSEADDR`) to 1, binds and listens with a backlog of 2. When bind or
/// listen returns `-1` virtual slot `0x10` (`close`) is called and 1 is answered. Otherwise
/// the message "Listening on host[<host name>] port <port>" is built in a 512-byte text
/// builder and reported (level 0, id `-1`, line `0x16c`), the builders are destroyed and the
/// answer is 0.
pub fn hk_bsd_socket_listen(e: &mut Engine, this: Ptr, port: u32) -> u32 {
    if open_socket_00da2de0(e, this) {
        return 1;
    }
    // The exe's frame (offsets from its stack pointer after the saves): the option value at
    // +8, the address at +0xc, the text builder at +0x1c, the `hkStringBuf` at +0x28, the host
    // name (256 bytes) at +0xb4 and the 512-byte message buffer at +0x1b4.
    e.with_stack(0x3b4, |e, frame| {
        let frame = frame.addr();
        let option = frame + 8;
        let address = frame + 0xc;
        let builder = frame + 0x1c;
        let string_buf = frame + 0x28;
        let host_name = frame + 0xb4;
        let message = frame + 0x1b4;
        e.mem.set_u16(address, 2);
        e.mem.set_u32(address + 4, 0);
        let network_port = e.call(HOST_TO_NET_SHORT, &args![port]).u16();
        e.mem.set_u16(address + 2, network_port);
        e.mem.set_u32(option, 1);
        let handle = e.mem.u32(this.addr() + HANDLE);
        e.call(
            SET_SOCKET_OPTION,
            &args![handle, 0xffffu32, 4u32, option, 4u32],
        );
        let handle = e.mem.u32(this.addr() + HANDLE);
        if e.call(BIND, &args![handle, address, 0x10u32]).i32() != -1 {
            let handle = e.mem.u32(this.addr() + HANDLE);
            if e.call(LISTEN, &args![handle, 2u32]).i32() != -1 {
                e.call(STRING_BUF_CONSTRUCT, &args![string_buf]);
                e.call(GET_HOST_NAME, &args![host_name, 0x100u32]);
                e.call(STRING_BUF_ASSIGN, &args![string_buf, host_name]);
                e.call(TEXT_BUILDER_CONSTRUCT, &args![builder, message, 0x200u32]);
                let text = e
                    .call(TEXT_BUILDER_APPEND, &args![builder, TEXT_LISTENING])
                    .u32();
                let text = e
                    .call(TEXT_BUILDER_APPEND_STRING_BUF, &args![text, string_buf])
                    .u32();
                let text = e.call(TEXT_BUILDER_APPEND, &args![text, TEXT_PORT]).u32();
                e.call(TEXT_BUILDER_APPEND_INT, &args![text, port]);
                let sink = e.mem.u32(REPORT_SINK);
                e.vcall(
                    sink,
                    0xc,
                    &args![0u32, 0xffff_ffffu32, message, SOURCE_FILE, 0x16cu32],
                );
                e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
                e.call(STRING_BUF_DESTRUCT, &args![string_buf]);
                return 0;
            }
        }
        e.vcall(this.addr(), 0x10, &args![]);
        1
    })
}

// Translated from 00da3230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBsdSocket::pollForNewClient` (Xbox PDB): without a handle (`-1`) the answer is 0.
/// Otherwise `select` waits with a zero timeout on a one-socket read set and a one-socket
/// exception set; a result of zero or less, or a handle that is not in the read set, answers
/// 0. Else `accept` takes the connection, "Socket got connection from [<address>:<port>]" is
/// built (`hkStringBuf` formatter, then a 512-byte text builder) and reported (level 0, id
/// `-1`, line `0x193`). With an accepted handle of `-1` the answer is 0; else option
/// `IPPROTO_TCP`/1 (`TCP_NODELAY`) is set to 1 and a new socket object (0x24 bytes from the
/// memory router's allocator, slot 4, the size stored as the 16-bit word at `+4`) is
/// constructed ([`construct_00da30a0`]) around the accepted handle and returned.
pub fn hk_bsd_socket_poll_for_new_client(e: &mut Engine, this: Ptr) -> Ptr {
    let handle = e.mem.u32(this.addr() + HANDLE);
    if handle == NO_SOCKET {
        return Ptr::new(0);
    }
    // The exe's frame (offsets from its stack pointer after the save): the option value at
    // +4, the address length at +8, the timeout at +0xc, the text builder at +0x14, the
    // client address at +0x20, the `hkStringBuf` at +0x30, the read set at +0xbc, the
    // exception set at +0x1c0 and the 512-byte message buffer at +0x2c4.
    e.with_stack(0x4c0, |e, frame| {
        let frame = frame.addr();
        let option = frame + 4;
        let address_length = frame + 8;
        let timeout = frame + 0xc;
        let builder = frame + 0x14;
        let client = frame + 0x20;
        let string_buf = frame + 0x30;
        let read_set = frame + 0xbc;
        let except_set = frame + 0x1c0;
        let message = frame + 0x2c4;
        e.mem.set_u32(read_set + 4, handle);
        e.mem.set_u32(except_set + 4, handle);
        e.mem.set_u32(read_set, 1);
        e.mem.set_u32(except_set, 1);
        e.mem.set_u32(timeout, 0);
        e.mem.set_u32(timeout + 4, 0);
        let ready = e
            .call(
                SELECT,
                &args![handle.wrapping_add(1), read_set, 0u32, except_set, timeout],
            )
            .i32();
        if ready <= 0 {
            return Ptr::new(0);
        }
        let handle = e.mem.u32(this.addr() + HANDLE);
        if e.call(IS_SET, &args![handle, read_set]).u32() == 0 {
            return Ptr::new(0);
        }
        let handle = e.mem.u32(this.addr() + HANDLE);
        e.mem.set_u32(address_length, 0x10);
        let accepted = e.call(ACCEPT, &args![handle, client, address_length]).u32();
        e.call(STRING_BUF_CONSTRUCT, &args![string_buf]);
        let port_word = e.mem.u32(client + 2);
        let port = e.call(NET_TO_HOST_SHORT, &args![port_word]).u16();
        let internet_address = e.mem.u32(client + 4);
        let text = e.call(INET_NTOA, &args![internet_address]).u32();
        e.call(
            STRING_BUF_FORMAT,
            &args![string_buf, FORMAT_CONNECTION, text, u32::from(port)],
        );
        e.call(TEXT_BUILDER_CONSTRUCT, &args![builder, message, 0x200u32]);
        e.call(TEXT_BUILDER_APPEND_STRING_BUF, &args![builder, string_buf]);
        let sink = e.mem.u32(REPORT_SINK);
        e.vcall(
            sink,
            0xc,
            &args![0u32, 0xffff_ffffu32, message, SOURCE_FILE, 0x193u32],
        );
        e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
        if accepted == NO_SOCKET {
            e.call(STRING_BUF_DESTRUCT, &args![string_buf]);
            return Ptr::new(0);
        }
        e.mem.set_u32(option, 1);
        e.call(
            SET_SOCKET_OPTION,
            &args![accepted, 6u32, 1u32, option, 4u32],
        );
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let object = e.vcall(allocator, 4, &args![0x24u32]).u32();
        e.mem.set_u16(object + 4, 0x24);
        let client_socket = construct_00da30a0(e, Ptr::new(object), accepted);
        e.call(STRING_BUF_DESTRUCT, &args![string_buf]);
        client_socket
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_33d0, deleting_destructor_00da33d0(Ptr, u8) -> Ptr),
        entry!(0x00da_2db0, hk_bsd_socket_is_ok(Ptr, u32) -> u32),
        entry!(0x00da_2dc0, hk_bsd_socket_close(Ptr)),
        entry!(0x00da_2de0, open_socket_00da2de0(Ptr) -> bool),
        entry!(0x00da_2e10, hk_bsd_socket_read(Ptr, u32, u32) -> i32),
        entry!(0x00da_2e60, hk_bsd_socket_write(Ptr, u32, u32) -> i32),
        entry!(0x00da_2eb0, is_digit_00da2eb0(u32, u32) -> u32),
        entry!(0x00da_2ec0, hk_bsd_socket_connect(Ptr, u32, u32) -> u32),
        entry!(0x00da_2f90, hk_bsd_socket_async_select(Ptr, u32, u32, u32) -> bool),
        entry!(0x00da_2ff0, hk_bsd_network_init()),
        entry!(0x00da_3080, destructor_00da3080(Ptr)),
        entry!(0x00da_30a0, construct_00da30a0(Ptr, u32) -> Ptr),
        entry!(0x00da_30d0, hk_bsd_socket_listen(Ptr, u32) -> u32),
        entry!(0x00da_3230, hk_bsd_socket_poll_for_new_client(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00da33d0_frees_only_when_bit_zero_is_set() {
        check(0x00da_33d0, 0x00da_3080);
    }
}

#[cfg(test)]
mod socket_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The doubles behind virtual slot `0x10` of the test sockets (`close`), slot `0xc` of the
    /// report sink and slot `4` of the allocator.
    const CLOSE_SLOT_DOUBLE: u32 = 0x0900_0010;
    const SINK_SLOT_DOUBLE: u32 = 0x0900_0020;
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0030;

    type Seen = Rc<RefCell<Vec<u8>>>;

    /// An engine with the exe's data range and the import page mapped (the tests run without
    /// the exe), recording its calls.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument words of every call to `address`, in order.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A double at `address` that answers `value`.
    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A socket object holding `handle`, whose virtual slot `0x10` is a double.
    fn socket_object(e: &mut Engine, handle: u32) -> u32 {
        answer(e, CLOSE_SLOT_DOUBLE, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, CLOSE_SLOT_DOUBLE);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + HANDLE, handle);
        object
    }

    /// Installs the report sink global (slot `0xc` answers `verdict`) and doubles for the text
    /// builder and `hkStringBuf` functions; the appends answer their first argument.
    fn text_doubles(e: &mut Engine, verdict: u32) {
        answer(e, SINK_SLOT_DOUBLE, verdict);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SINK_SLOT_DOUBLE);
        let sink = e.mem.alloc(8);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        for address in [
            TEXT_BUILDER_APPEND,
            TEXT_BUILDER_APPEND_STRING_BUF,
            TEXT_BUILDER_APPEND_INT,
        ] {
            e.register_double(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        for address in [
            TEXT_BUILDER_CONSTRUCT,
            TEXT_BUILDER_DESTRUCT,
            STRING_BUF_CONSTRUCT,
            STRING_BUF_DESTRUCT,
            STRING_BUF_ASSIGN,
            STRING_BUF_FORMAT,
        ] {
            answer(e, address, 0);
        }
    }

    /// A double at `address` that copies `length` bytes at its argument `index` into the result.
    fn capture(e: &mut Engine, address: u32, index: usize, length: u32, value: u32) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            *sink.borrow_mut() = e.mem.bytes(a[index], length);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        seen
    }

    #[test]
    fn is_ok_reports_whether_a_handle_is_held() {
        let mut e = engine();
        let socket = socket_object(&mut e, 5);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        assert_eq!(e.call(0x00da_2db0, &args![socket, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(socket + HANDLE, NO_SOCKET);
        e.call(0x00da_2db0, &args![socket, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn close_closes_a_held_handle_once() {
        let mut e = engine();
        answer(&mut e, CLOSE_SOCKET, 0);
        let socket = socket_object(&mut e, 7);
        e.call(0x00da_2dc0, &args![socket]);
        assert_eq!(calls_to(&e, CLOSE_SOCKET), vec![vec![7]]);
        assert_eq!(e.mem.u32(socket + HANDLE), NO_SOCKET);
        e.call(0x00da_2dc0, &args![socket]);
        assert_eq!(calls_to(&e, CLOSE_SOCKET).len(), 1);
    }

    #[test]
    fn open_socket_closes_then_creates_and_reports_failure() {
        let mut e = engine();
        answer(&mut e, CREATE_SOCKET, 9);
        let socket = socket_object(&mut e, 4);
        assert_eq!(e.call(0x00da_2de0, &args![socket]).u32(), 0);
        assert_eq!(e.mem.u32(socket + HANDLE), 9);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE), vec![vec![socket]]);
        assert_eq!(calls_to(&e, CREATE_SOCKET), vec![vec![2, 1, 0]]);
        answer(&mut e, CREATE_SOCKET, NO_SOCKET);
        assert_eq!(e.call(0x00da_2de0, &args![socket]).u32(), 1);
        assert_eq!(e.mem.u32(socket + HANDLE), NO_SOCKET);
    }

    /// An engine with a socket holding `handle` and the Winsock doubles of one transfer.
    fn transfer_engine(winsock: u32, count: u32, error: u32, handle: u32) -> (Engine, u32) {
        let mut e = engine();
        answer(&mut e, winsock, count);
        answer(&mut e, LAST_ERROR, error);
        let socket = socket_object(&mut e, handle);
        (e, socket)
    }

    /// The behaviour `read` and `write` share, with `winsock` the thunk they use.
    fn check_transfer(function: u32, winsock: u32) {
        let (mut e, socket) = transfer_engine(winsock, 5, 0, 3);
        let got = e.call(function, &args![socket, 0x1000u32, 16u32]).i32();
        assert_eq!(got, 5);
        assert_eq!(calls_to(&e, winsock), vec![vec![3, 0x1000, 16, 0]]);
        assert!(calls_to(&e, LAST_ERROR).is_empty());
        // `WSAEWOULDBLOCK` leaves the socket open.
        let (mut e, socket) = transfer_engine(winsock, NO_SOCKET, WOULD_BLOCK, 3);
        assert_eq!(e.call(function, &args![socket, 0x1000u32, 16u32]).i32(), 0);
        assert!(calls_to(&e, CLOSE_SLOT_DOUBLE).is_empty());
        // Another error, and a count of 0, close it.
        let (mut e, socket) = transfer_engine(winsock, NO_SOCKET, 0x2745, 3);
        assert_eq!(e.call(function, &args![socket, 0x1000u32, 16u32]).i32(), 0);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE), vec![vec![socket]]);
        let (mut e, socket) = transfer_engine(winsock, 0, 0, 3);
        assert_eq!(e.call(function, &args![socket, 0x1000u32, 16u32]).i32(), 0);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE), vec![vec![socket]]);
        // Without a handle nothing is asked of Winsock.
        let (mut e, socket) = transfer_engine(winsock, 5, 0, NO_SOCKET);
        assert_eq!(e.call(function, &args![socket, 0x1000u32, 16u32]).i32(), 0);
        assert!(calls_to(&e, winsock).is_empty());
    }

    #[test]
    fn read_receives_and_closes_on_errors() {
        check_transfer(0x00da_2e10, RECEIVE);
    }

    #[test]
    fn write_sends_and_closes_on_errors() {
        check_transfer(0x00da_2e60, SEND);
    }

    #[test]
    fn is_digit_tests_the_character_code() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        for code in 0x30u32..=0x39 {
            e.mem.set_u8(out, 7);
            assert_eq!(e.call(0x00da_2eb0, &args![code, out]).u32(), out);
            assert_eq!(e.mem.u8(out), 1, "code {code:#x}");
        }
        for code in [0x2fu32, 0x3a, 0x41, 0, 0xffff_ffe0] {
            e.mem.set_u8(out, 7);
            e.call(0x00da_2eb0, &args![code, out]);
            assert_eq!(e.mem.u8(out), 0, "code {code:#x}");
        }
    }

    /// Doubles for `memset`/`memcpy` that really clear and copy, and `htons` answering `0x5000`.
    fn memory_doubles(e: &mut Engine) {
        e.register_double(MEM_SET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register_double(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        answer(e, HOST_TO_NET_SHORT, 0x5000);
    }

    #[test]
    fn connect_resolves_numeric_and_named_hosts() {
        // A numeric host goes through `inet_addr`.
        let mut e = engine();
        memory_doubles(&mut e);
        answer(&mut e, INET_ADDR, 0x0100_000a);
        let seen = capture(&mut e, CONNECT, 1, 16, 0);
        let socket = socket_object(&mut e, 3);
        let host = e.mem.alloc(16);
        e.mem.set_cstr(host, b"10.0.0.1");
        let got = e.call(0x00da_2ec0, &args![socket, host, 0x1234_0050u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(calls_to(&e, HOST_TO_NET_SHORT), vec![vec![0x1234_0050]]);
        assert_eq!(calls_to(&e, INET_ADDR), vec![vec![host]]);
        assert_eq!(
            *seen.borrow(),
            vec![2, 0, 0, 0x50, 0x0a, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(calls_to(&e, CONNECT)[0][0], 3);
        assert_eq!(calls_to(&e, CONNECT)[0][2], 16);
        // A named host copies the first address of the entry (length from the 16-bit word).
        let mut e = engine();
        memory_doubles(&mut e);
        let seen = capture(&mut e, CONNECT, 1, 16, 0);
        let entry = e.mem.alloc(0x10);
        let list = e.mem.alloc(8);
        let address = e.mem.alloc(8);
        e.mem.write(address, &[192, 168, 1, 2]);
        e.mem.set_i16(entry + 0xa, 4);
        e.mem.set_u32(entry + 0xc, list);
        e.mem.set_u32(list, address);
        answer(&mut e, GET_HOST_BY_NAME, entry);
        let socket = socket_object(&mut e, 3);
        let host = e.mem.alloc(16);
        e.mem.set_cstr(host, b"localhost");
        assert_eq!(e.call(0x00da_2ec0, &args![socket, host, 80u32]).u32(), 0);
        assert!(calls_to(&e, INET_ADDR).is_empty());
        assert_eq!(calls_to(&e, MEM_COPY).len(), 1);
        assert_eq!(&seen.borrow()[4..8], &[192, 168, 1, 2]);
        // An unresolved name answers 1 without connecting.
        answer(&mut e, GET_HOST_BY_NAME, 0);
        assert_eq!(e.call(0x00da_2ec0, &args![socket, host, 80u32]).u32(), 1);
        assert_eq!(calls_to(&e, CONNECT).len(), 1);
    }

    #[test]
    fn connect_opens_a_missing_socket_and_handles_errors() {
        let mut e = engine();
        memory_doubles(&mut e);
        answer(&mut e, INET_ADDR, 1);
        answer(&mut e, CREATE_SOCKET, 8);
        answer(&mut e, CONNECT, NO_SOCKET);
        answer(&mut e, LAST_ERROR, WOULD_BLOCK);
        let socket = socket_object(&mut e, NO_SOCKET);
        let host = e.mem.alloc(16);
        e.mem.set_cstr(host, b"1.2.3.4");
        // The socket is opened first; a would-block failure still answers 0.
        assert_eq!(e.call(0x00da_2ec0, &args![socket, host, 80u32]).u32(), 0);
        assert_eq!(calls_to(&e, CONNECT)[0][0], 8);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE), vec![vec![socket]]);
        // Another error closes it and answers 1.
        answer(&mut e, LAST_ERROR, 0x274d);
        assert_eq!(e.call(0x00da_2ec0, &args![socket, host, 80u32]).u32(), 1);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE).len(), 2);
        // A failed creation answers 1 before connecting.
        answer(&mut e, CREATE_SOCKET, NO_SOCKET);
        e.mem.set_u32(socket + HANDLE, NO_SOCKET);
        assert_eq!(e.call(0x00da_2ec0, &args![socket, host, 80u32]).u32(), 1);
        assert_eq!(calls_to(&e, CONNECT).len(), 2);
    }

    #[test]
    fn async_select_rearranges_the_event_bits() {
        let mut e = engine();
        answer(&mut e, ASYNC_SELECT, NO_SOCKET);
        let socket = socket_object(&mut e, 6);
        let got = e.call(0x00da_2f90, &args![socket, 0x77u32, 0x400u32, 0xfu32]);
        assert_eq!(got.u32(), 1);
        assert_eq!(calls_to(&e, ASYNC_SELECT), vec![vec![6, 0x77, 0x400, 0x33]]);
        e.call(0x00da_2f90, &args![socket, 0x77u32, 0x400u32, 0x10u32]);
        assert_eq!(calls_to(&e, ASYNC_SELECT)[1][3], 0);
        answer(&mut e, ASYNC_SELECT, 0);
        let got = e.call(0x00da_2f90, &args![socket, 1u32, 2u32, 4u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(calls_to(&e, ASYNC_SELECT)[2], vec![6, 1, 2, 0x10]);
    }

    #[test]
    fn network_init_starts_winsock_once_and_reports_failure() {
        let mut e = engine();
        text_doubles(&mut e, 0);
        answer(&mut e, NETWORK_STARTUP, 0);
        e.call(0x00da_2ff0, &args![]);
        assert_eq!(e.mem.u8(NETWORK_STARTED), 1);
        assert_eq!(calls_to(&e, NETWORK_STARTUP).len(), 1);
        assert_eq!(calls_to(&e, NETWORK_STARTUP)[0][0], 0x202);
        assert!(calls_to(&e, TEXT_BUILDER_CONSTRUCT).is_empty());
        // Once started nothing runs again.
        e.call(0x00da_2ff0, &args![]);
        assert_eq!(calls_to(&e, NETWORK_STARTUP).len(), 1);
        // A failed start reports through the sink and destroys the builder.
        let mut e = engine();
        text_doubles(&mut e, 0);
        answer(&mut e, NETWORK_STARTUP, NO_SOCKET);
        e.call(0x00da_2ff0, &args![]);
        let construct = &calls_to(&e, TEXT_BUILDER_CONSTRUCT)[0];
        let (builder, message) = (construct[0], construct[1]);
        assert_eq!(construct[2], 0x200);
        assert_eq!(message, builder + 0x19c);
        assert_eq!(calls_to(&e, NETWORK_STARTUP)[0][1], builder + 0xc);
        assert_eq!(
            calls_to(&e, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_STARTUP_FAILED]]
        );
        let sink = e.mem.u32(REPORT_SINK);
        assert_eq!(
            calls_to(&e, SINK_SLOT_DOUBLE),
            vec![vec![sink, 3, 0x3218_25f8, message, SOURCE_FILE, 0x48]]
        );
        assert_eq!(calls_to(&e, TEXT_BUILDER_DESTRUCT), vec![vec![builder]]);
        assert_eq!(e.mem.u8(NETWORK_STARTED), 1);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00da305e")]
    fn network_init_breaks_when_the_sink_asks_for_it() {
        let mut e = engine();
        text_doubles(&mut e, 1);
        answer(&mut e, NETWORK_STARTUP, NO_SOCKET);
        e.call(0x00da_2ff0, &args![]);
    }

    #[test]
    fn destructor_closes_the_socket_and_restores_the_base_vtable() {
        let mut e = engine();
        answer(&mut e, CLOSE_SOCKET, 0);
        let socket = socket_object(&mut e, 7);
        e.call(0x00da_3080, &args![socket]);
        assert_eq!(calls_to(&e, CLOSE_SOCKET), vec![vec![7]]);
        assert_eq!(e.mem.u32(socket + HANDLE), NO_SOCKET);
        for offset in [0, 8, 0x14] {
            assert_eq!(e.mem.u32(socket + offset), BASE_VTABLE);
        }
    }

    #[test]
    fn constructor_stores_the_handle_or_opens_a_socket() {
        let mut e = engine();
        answer(&mut e, BASE_CONSTRUCT, 0);
        answer(&mut e, CREATE_SOCKET, 12);
        answer(&mut e, CLOSE_SLOT_DOUBLE, 0);
        e.mem.set_u32(VTABLE + 0x10, CLOSE_SLOT_DOUBLE);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00da_30a0, &args![this, 6u32]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + HANDLE), 6);
        assert!(calls_to(&e, CREATE_SOCKET).is_empty());
        // With -1 the socket is opened (the socket vtable is already installed, so its
        // slot 0x10 runs first).
        let other = e.mem.alloc(0x40);
        e.call(0x00da_30a0, &args![other, NO_SOCKET]);
        assert_eq!(e.mem.u32(other + HANDLE), 12);
        assert_eq!(calls_to(&e, CREATE_SOCKET), vec![vec![2, 1, 0]]);
        assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE), vec![vec![other]]);
    }

    #[test]
    fn listen_binds_and_reports_the_host() {
        let mut e = engine();
        memory_doubles(&mut e);
        text_doubles(&mut e, 0);
        answer(&mut e, CREATE_SOCKET, 4);
        answer(&mut e, SET_SOCKET_OPTION, 0);
        answer(&mut e, GET_HOST_NAME, 0);
        answer(&mut e, LISTEN, 0);
        let bound = capture(&mut e, BIND, 1, 8, 0);
        answer(&mut e, HOST_TO_NET_SHORT, 0x901f);
        let socket = socket_object(&mut e, NO_SOCKET);
        assert_eq!(e.call(0x00da_30d0, &args![socket, 8080u32]).u32(), 0);
        assert_eq!(*bound.borrow(), vec![2, 0, 0x1f, 0x90, 0, 0, 0, 0]);
        let option = &calls_to(&e, SET_SOCKET_OPTION)[0];
        assert_eq!(option[0], 4);
        assert_eq!(option[1..3], [0xffff, 4]);
        assert_eq!(option[4], 4);
        assert_eq!(calls_to(&e, BIND)[0][0], 4);
        assert_eq!(calls_to(&e, BIND)[0][2], 0x10);
        assert_eq!(calls_to(&e, LISTEN), vec![vec![4, 2]]);
        let construct = &calls_to(&e, TEXT_BUILDER_CONSTRUCT)[0];
        let (builder, message) = (construct[0], construct[1]);
        assert_eq!(construct[2], 0x200);
        assert_eq!(builder, option[3] - 8 + 0x1c);
        let string_buf = calls_to(&e, STRING_BUF_CONSTRUCT)[0][0];
        assert_eq!(string_buf, builder + 0xc);
        let host_name = calls_to(&e, GET_HOST_NAME)[0][0];
        assert_eq!(
            calls_to(&e, STRING_BUF_ASSIGN),
            vec![vec![string_buf, host_name]]
        );
        assert_eq!(
            calls_to(&e, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_LISTENING], vec![builder, TEXT_PORT]]
        );
        assert_eq!(
            calls_to(&e, TEXT_BUILDER_APPEND_STRING_BUF),
            vec![vec![builder, string_buf]]
        );
        assert_eq!(
            calls_to(&e, TEXT_BUILDER_APPEND_INT),
            vec![vec![builder, 8080]]
        );
        let sink = e.mem.u32(REPORT_SINK);
        assert_eq!(
            calls_to(&e, SINK_SLOT_DOUBLE),
            vec![vec![sink, 0, NO_SOCKET, message, SOURCE_FILE, 0x16c]]
        );
        assert_eq!(calls_to(&e, TEXT_BUILDER_DESTRUCT), vec![vec![builder]]);
        assert_eq!(calls_to(&e, STRING_BUF_DESTRUCT), vec![vec![string_buf]]);
    }

    #[test]
    fn listen_closes_and_answers_one_on_failures() {
        // A failed creation stops before any address is built.
        let mut e = engine();
        memory_doubles(&mut e);
        text_doubles(&mut e, 0);
        answer(&mut e, CREATE_SOCKET, NO_SOCKET);
        answer(&mut e, BIND, 0);
        let socket = socket_object(&mut e, 5);
        assert_eq!(e.call(0x00da_30d0, &args![socket, 80u32]).u32(), 1);
        assert!(calls_to(&e, BIND).is_empty());
        // A failed bind, and a failed listen, close the socket (the first close is the
        // one `open_socket` makes) and answer 1 without reporting.
        for (bind, listen) in [(NO_SOCKET, 0), (0, NO_SOCKET)] {
            let mut e = engine();
            memory_doubles(&mut e);
            text_doubles(&mut e, 0);
            answer(&mut e, CREATE_SOCKET, 4);
            answer(&mut e, SET_SOCKET_OPTION, 0);
            answer(&mut e, BIND, bind);
            answer(&mut e, LISTEN, listen);
            let socket = socket_object(&mut e, 5);
            assert_eq!(e.call(0x00da_30d0, &args![socket, 80u32]).u32(), 1);
            assert_eq!(calls_to(&e, CLOSE_SLOT_DOUBLE).len(), 2);
            assert!(calls_to(&e, SINK_SLOT_DOUBLE).is_empty());
            assert_eq!(calls_to(&e, LISTEN).len(), usize::from(bind == 0));
        }
    }

    /// Doubles for a successful `pollForNewClient`: the read set holds the handle and the
    /// accepted client (handle `accepted`) comes from 192.168.1.2 port 8080.
    fn poll_doubles(e: &mut Engine, ready: u32, in_set: u32, accepted: u32) -> u32 {
        text_doubles(e, 0);
        answer(e, SELECT, ready);
        answer(e, IS_SET, in_set);
        answer(e, SET_SOCKET_OPTION, 0);
        e.register_double(ACCEPT, move |e, a| {
            e.mem.write(a[1], &[2, 0, 0x1f, 0x90, 192, 168, 1, 2]);
            Ret {
                eax: accepted,
                ..Ret::default()
            }
        });
        answer(e, NET_TO_HOST_SHORT, 0x1f90);
        answer(e, INET_NTOA, 0x0900_1000);
        answer(e, BASE_CONSTRUCT, 0);
        let block = e.mem.alloc(0x30);
        answer(e, ALLOCATE_SLOT_DOUBLE, block);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        block
    }

    #[test]
    fn poll_for_new_client_accepts_and_builds_a_socket_object() {
        let mut e = engine();
        let block = poll_doubles(&mut e, 1, 1, 9);
        let socket = socket_object(&mut e, 5);
        let got = e.call(0x00da_3230, &args![socket]).u32();
        assert_eq!(got, block);
        let select = &calls_to(&e, SELECT)[0];
        let (read_set, except_set, timeout) = (select[1], select[3], select[4]);
        assert_eq!(select[0], 6);
        assert_eq!(select[2], 0);
        assert_eq!(except_set, read_set + 0x104);
        assert_eq!(timeout, read_set - 0xb0);
        assert_eq!(calls_to(&e, IS_SET), vec![vec![5, read_set]]);
        let accept = &calls_to(&e, ACCEPT)[0];
        assert_eq!(accept[0], 5);
        let (client, string_buf) = (accept[1], calls_to(&e, STRING_BUF_CONSTRUCT)[0][0]);
        assert_eq!(string_buf, client + 0x10);
        assert_eq!(calls_to(&e, NET_TO_HOST_SHORT), vec![vec![0xa8c0_901f]]);
        assert_eq!(calls_to(&e, INET_NTOA), vec![vec![0x0201_a8c0]]);
        let construct = &calls_to(&e, TEXT_BUILDER_CONSTRUCT)[0];
        let (builder, message) = (construct[0], construct[1]);
        assert_eq!(
            calls_to(&e, STRING_BUF_FORMAT),
            vec![vec![string_buf, FORMAT_CONNECTION, 0x0900_1000, 0x1f90]]
        );
        assert_eq!(
            calls_to(&e, TEXT_BUILDER_APPEND_STRING_BUF),
            vec![vec![builder, string_buf]]
        );
        let sink = e.mem.u32(REPORT_SINK);
        assert_eq!(
            calls_to(&e, SINK_SLOT_DOUBLE),
            vec![vec![sink, 0, NO_SOCKET, message, SOURCE_FILE, 0x193]]
        );
        let option = &calls_to(&e, SET_SOCKET_OPTION)[0];
        assert_eq!(option[..3], [9, 6, 1]);
        assert_eq!(option[4], 4);
        assert_eq!(calls_to(&e, ALLOCATE_SLOT_DOUBLE).len(), 1);
        assert_eq!(e.mem.u16(block + 4), 0x24);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + HANDLE), 9);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![block]]);
        assert_eq!(calls_to(&e, STRING_BUF_DESTRUCT), vec![vec![string_buf]]);
    }

    #[test]
    fn poll_for_new_client_answers_zero_when_nothing_is_ready() {
        // No handle: nothing is asked.
        let mut e = engine();
        poll_doubles(&mut e, 1, 1, 9);
        let socket = socket_object(&mut e, NO_SOCKET);
        assert_eq!(e.call(0x00da_3230, &args![socket]).u32(), 0);
        assert!(calls_to(&e, SELECT).is_empty());
        // `select` answering 0 or a negative count (a signed test), or a handle outside the
        // read set, stops before `accept`.
        for (ready, in_set) in [(0, 1), (NO_SOCKET, 1), (1, 0)] {
            let mut e = engine();
            poll_doubles(&mut e, ready, in_set, 9);
            let socket = socket_object(&mut e, 5);
            assert_eq!(e.call(0x00da_3230, &args![socket]).u32(), 0);
            assert!(calls_to(&e, ACCEPT).is_empty());
        }
        // An accepted handle of -1 is reported but gives no object; the builder is destroyed.
        let mut e = engine();
        poll_doubles(&mut e, 1, 1, NO_SOCKET);
        let socket = socket_object(&mut e, 5);
        assert_eq!(e.call(0x00da_3230, &args![socket]).u32(), 0);
        assert_eq!(calls_to(&e, SINK_SLOT_DOUBLE).len(), 1);
        assert_eq!(calls_to(&e, STRING_BUF_DESTRUCT).len(), 1);
        assert!(calls_to(&e, SET_SOCKET_OPTION).is_empty());
        assert!(calls_to(&e, ALLOCATE_SLOT_DOUBLE).is_empty());
    }
}

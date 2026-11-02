/*
 * Synthetic 32-bit program used to test the nv-oracle tools.
 *
 * Everything here is written from scratch for the tests. It is not a model
 * of any game code. It provides functions in each calling convention, float
 * returns through the x87 stack, an SSE routine built on RSQRTSS, a
 * function that writes through a struct pointer, and functions that fault.
 *
 * Modes (first argument):
 *   selftest          call every function directly on this CPU and print
 *                     one line per case: the inputs, then the exact result.
 *                     run-wine-tests.sh turns those lines into vectors for
 *                     nv-call and compares the answers bit for bit.
 *   loop [N [ms]]     call a fixed sequence of functions N times (default
 *                     5) after an optional delay; used for the nv-probe tests.
 *   threads T N [ms]  T threads each call a recursive function N times.
 *   nvse-sim <dll> [editor] [free] [wait <ms>]
 *                     load a DLL, call NVSEPlugin_Query and NVSEPlugin_Load
 *                     the way a plugin loader would, run the loop, and
 *                     optionally unload the DLL and run it again.
 *   unload-inside <dll>
 *                     load a DLL, start a thread that sleeps inside a
 *                     function, unload the DLL while that thread is still
 *                     inside, and let the thread return.
 *
 * Build: see run-wine-tests.sh (i686-w64-mingw32-gcc -O1 -fno-omit-frame-pointer -msse2).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>
#include <xmmintrin.h>

#define FX __attribute__((noinline, noclone, used))

typedef struct Obj {
    int32_t a;
    int32_t b;
    float c;
    uint32_t tag;
} Obj;

_Static_assert(sizeof(Obj) == 16, "Obj is 16 bytes");

int fx_global_k = 7;
int *fx_ptr_global = 0;

/* ---- one function per calling convention ---- */

FX int fx_add3(int a, int b, int c) { return a + 2 * b - c; }

FX int fx_sub3(int a, int b, int c) { return a - b - c; }

FX int __stdcall fx_mix(int a, unsigned b) { return (int)(((unsigned)a * 31u) ^ b); }

FX int __attribute__((thiscall)) fx_this_sum(Obj *self, int k) { return self->a + self->b * k; }

FX int __attribute__((fastcall)) fx_fast4(int a, int b, int c, int d) { return a - b + c * d; }

/* ---- x87 returns ---- */

FX float fx_lerp(float a, float b, float t) { return a + (b - a) * t; }

FX double fx_div_d(double a, double b) { return a / b; }

/* Returns the full 80-bit value, so the raw ST0 is observable. */
FX long double fx_ext_mix(double a, double b) { return (long double)a / b + (long double)a * b; }

/* The transcendental results are returned as long double so the full
 * 64-bit significand the instruction produced reaches the caller. */
FX long double fx_sin(double x) {
    long double r;
    __asm__("fsin" : "=t"(r) : "0"(x));
    return r;
}

FX long double fx_atan2(double y, double x) {
    long double r;
    __asm__("fpatan" : "=t"(r) : "0"(x), "u"(y) : "st(1)");
    return r;
}

/* ---- SSE ---- */

/* Normalise a 4-float vector in place: reciprocal square root estimate
 * (RSQRTSS) plus one Newton-Raphson step. */
FX void fx_quat_normalize(float *q) {
    __m128 v = _mm_loadu_ps(q);
    __m128 sq = _mm_mul_ps(v, v);
    sq = _mm_add_ps(sq, _mm_shuffle_ps(sq, sq, _MM_SHUFFLE(2, 3, 0, 1)));
    sq = _mm_add_ps(sq, _mm_shuffle_ps(sq, sq, _MM_SHUFFLE(1, 0, 3, 2)));
    __m128 r = _mm_rsqrt_ss(sq);
    __m128 half = _mm_set_ss(0.5f);
    __m128 three_half = _mm_set_ss(1.5f);
    __m128 t = _mm_mul_ss(_mm_mul_ss(half, sq), _mm_mul_ss(r, r));
    r = _mm_mul_ss(r, _mm_sub_ss(three_half, t));
    r = _mm_shuffle_ps(r, r, 0);
    _mm_storeu_ps(q, _mm_mul_ps(v, r));
}

/* The raw hardware estimate, returned in XMM0 and (separately) in ST0. */
FX __m128 fx_rsqrt_xmm(float x) { return _mm_rsqrt_ss(_mm_set_ss(x)); }

FX float fx_rsqrt_f(float x) {
    float r;
    _mm_store_ss(&r, _mm_rsqrt_ss(_mm_set_ss(x)));
    return r;
}

/* ---- struct access ---- */

FX int __attribute__((thiscall)) fx_obj_set(Obj *self, int a, int b) {
    int old = self->a + self->b;
    self->a = a;
    self->b = b;
    self->c = (float)(a + b) * 0.5f;
    self->tag = 0xC0DE0000u | ((uint32_t)a & 0xffffu);
    return old;
}

FX void fx_fill(Obj *o, int seed) {
    o->a = seed;
    o->b = seed * 3;
    o->c = (float)seed * 0.25f;
    o->tag = 0xF1110000u + (uint32_t)seed;
}

/* ---- recursion, for return capture with nested calls ---- */

FX int fx_fact(int n) {
    volatile int guard = n; /* keeps the recursion a real call at -O1 */
    return guard <= 1 ? 1 : guard * fx_fact(guard - 1);
}

/* ---- globals ---- */

FX int fx_use_global(int x) { return x + fx_global_k; }

FX int fx_read_ptr(void) { return fx_ptr_global ? *fx_ptr_global : -1; }

/* ---- code that calls an import, and code that faults ---- */

FX unsigned fx_calls_import(unsigned x) { return x + GetCurrentProcessId(); }

FX int fx_deref(int *p) { return *p; }

FX int fx_ud2(void) {
    __asm__ volatile("ud2");
    return 0;
}

FX int fx_divide(int a, int b) { return a / b; }

/* Mutable state in .bss: two identical calls give 1 and then 2 unless the
 * harness puts the image's writable memory back between vectors. */
static int fx_counter;
FX int fx_bump(void) { return ++fx_counter; }

/* Infinite recursion with a real stack frame, to overflow the stack. */
FX int fx_recurse(int n) {
    volatile char pad[512];
    pad[0] = (char)n;
    return fx_recurse(n + 1) + pad[0];
}

/* Calls that need an import, to test code that handles its own exceptions
 * (IsBadReadPtr catches the access violation inside the system library) and
 * a software-raised exception that is marked non-continuable. */
FX int fx_bad_read(const void *p) { return IsBadReadPtr(p, 4); }

FX int fx_raise_noncont(void) {
    RaiseException(0xC0000094u, EXCEPTION_NONCONTINUABLE, 0, NULL); /* integer divide by zero */
    return 5;
}

/* Exceptions that no handler of the called code deals with, so they reach the
 * harness: a C++-style throw (the code MSVC's throw raises), a continuable
 * exception with a code of the program's own, and a single step (the code sets
 * the trap flag itself). The fourth is a thread-name exception, which a
 * debugger swallows and the harness continues from. */
FX int fx_raise_cpp(void) {
    RaiseException(0xE06D7363u, EXCEPTION_NONCONTINUABLE, 0, NULL);
    return 5;
}

FX int fx_raise_custom(void) {
    RaiseException(0xE0000001u, 0, 0, NULL);
    return 7;
}

FX int fx_single_step(void) {
    __asm__ volatile("pushfl\n\torl $0x100, (%%esp)\n\tpopfl\n\tnop\n\tnop" : : : "memory", "cc");
    return 6;
}

FX int fx_raise_info(void) {
    RaiseException(0x406D1388u, 0, 0, NULL);
    return 8;
}

/* Calls Sleep for a while: a thread inside a hooked function with a captured
 * return while the probe is unloaded. */
static volatile LONG fx_inside_sleepy;
FX int fx_sleepy(int ms) {
    InterlockedExchange(&fx_inside_sleepy, 1);
    Sleep((DWORD)ms);
    return ms + 1;
}

/* Code that registers its own structured exception handler record, written
 * as raw assembly so the record layout is explicit:
 *   fx_seh_catch(p):  reads *p inside a handler that catches the access
 *                     violation; returns 1 if it was caught, 0 otherwise.
 *   fx_seh_fault():   registers a handler that declines, then faults.
 *   fx_seh_import():  registers a handler that declines, then calls an import.
 *   fx_seh_next():    returns the record that the newest one links to.
 * The last three let a test see whether the caller's handler chain is intact
 * after the call was abandoned. */
__asm__(".text\n"
        ".globl _fx_decline_handler\n"
        "_fx_decline_handler:\n"
        "  mov $1, %eax\n" /* ExceptionContinueSearch */
        "  ret\n"
        ".globl _fx_catch_handler\n"
        "_fx_catch_handler:\n"
        "  mov 4(%esp), %eax\n"
        "  testl $6, 4(%eax)\n" /* unwinding: not ours to handle */
        "  jnz 1f\n"
        "  mov 8(%esp), %eax\n"  /* the record, which is where ESP was */
        "  mov 12(%esp), %ecx\n" /* CONTEXT */
        "  mov %eax, 0xC4(%ecx)\n" /* Esp */
        "  movl $_fx_catch_resume, 0xB8(%ecx)\n" /* Eip */
        "  xor %eax, %eax\n" /* ExceptionContinueExecution */
        "  ret\n"
        "1:\n"
        "  mov $1, %eax\n"
        "  ret\n"
        ".globl _fx_seh_catch\n"
        "_fx_seh_catch:\n"
        "  mov 4(%esp), %ecx\n"
        "  push $_fx_catch_handler\n"
        "  pushl %fs:0\n"
        "  mov %esp, %fs:0\n"
        "  mov (%ecx), %eax\n"
        "  popl %fs:0\n"
        "  add $4, %esp\n"
        "  xor %eax, %eax\n"
        "  ret\n"
        "_fx_catch_resume:\n"
        "  popl %fs:0\n"
        "  add $4, %esp\n"
        "  mov $1, %eax\n"
        "  ret\n"
        ".globl _fx_seh_fault\n"
        "_fx_seh_fault:\n"
        "  push $_fx_decline_handler\n"
        "  pushl %fs:0\n"
        "  mov %esp, %fs:0\n"
        "  xor %eax, %eax\n"
        "  mov (%eax), %eax\n"
        "  popl %fs:0\n"
        "  add $4, %esp\n"
        "  ret\n"
        ".globl _fx_seh_import\n"
        "_fx_seh_import:\n"
        "  push $_fx_decline_handler\n"
        "  pushl %fs:0\n"
        "  mov %esp, %fs:0\n"
        "  call *__imp__GetCurrentProcessId@0\n"
        "  popl %fs:0\n"
        "  add $4, %esp\n"
        "  ret\n"
        ".globl _fx_seh_next\n"
        "_fx_seh_next:\n"
        "  mov %fs:0, %eax\n"
        "  mov (%eax), %eax\n"
        "  ret\n");
extern int fx_seh_catch(int *p);
extern int fx_seh_fault(void);
extern int fx_seh_import(void);
extern unsigned fx_seh_next(void);

/* A function whose first bytes hold a short branch (85 C9 74 06 ...), which
 * a hook cannot relocate. Takes its argument in ECX. */
__asm__(".text\n"
        ".globl _fx_tiny_raw\n"
        "_fx_tiny_raw:\n"
        "  test %ecx, %ecx\n"
        "  jz 1f\n"
        "  mov $1, %eax\n"
        "  ret\n"
        "1:\n"
        "  xor %eax, %eax\n"
        "  ret\n");
extern char fx_tiny_raw[];
FX int fx_tiny(int x) { return ((int(__attribute__((fastcall)) *)(int))fx_tiny_raw)(x); }

/* Two functions whose first instructions are relative branches, to test that
 * a hook relocates them:
 *   fx_reloc:      mov ecx,[esp+4]; call helper; add eax,5; ret   (E8 inside the stolen bytes)
 *   fx_thunk_sub3: jmp fx_sub3                                      (E9 is the whole function start)
 * The branches are written as raw bytes so they are always the 32-bit form. */
__asm__(".text\n"
        ".globl _fx_reloc\n"
        "_fx_reloc:\n"
        "  mov 4(%esp), %ecx\n"
        "  .byte 0xe8\n"
        "  .long _fx_reloc_helper - . - 4\n"
        "  add $5, %eax\n"
        "  ret\n"
        "  .p2align 4\n"
        "_fx_reloc_helper:\n"
        "  lea 1(,%ecx,2), %eax\n"
        "  ret\n"
        "  .p2align 4\n"
        ".globl _fx_thunk_sub3\n"
        "_fx_thunk_sub3:\n"
        "  .byte 0xe9\n"
        "  .long _fx_sub3 - . - 4\n"
        "  .p2align 4\n");
extern int fx_reloc(int x);
extern int fx_thunk_sub3(int a, int b, int c);

/* ---- selftest output ---- */

static unsigned f2u(float f) {
    unsigned u;
    memcpy(&u, &f, 4);
    return u;
}

static unsigned long long d2u(double d) {
    unsigned long long u;
    memcpy(&u, &d, 8);
    return u;
}

static void put_hex(const void *p, size_t n) {
    const unsigned char *b = (const unsigned char *)p;
    for (size_t i = 0; i < n; i++) printf("%02x", b[i]);
}

static void fpu_set(unsigned cw) {
    unsigned short w = (unsigned short)cw;
    __asm__ volatile("fldcw %0" : : "m"(w));
    _mm_setcsr(0x1F80);
}

static void begin(const char *id, const char *sym, const char *cc, const char *ret, unsigned cw, const char *args) {
    printf("CASE id=%s sym=%s cc=%s ret=%s fpcw=0x%04x args=%s", id, sym, cc, ret, cw, args);
}

static void buf_in(const char *name, const void *p, size_t n) {
    printf(" in.%s=", name);
    put_hex(p, n);
}

static void buf_out(const char *name, const void *p, size_t n) {
    printf(" out.%s=", name);
    put_hex(p, n);
}

static void end_i32(int r) { printf(" eax=%08x\n", (unsigned)r); }
static void end_f32(float r) { printf(" f32=%08x\n", f2u(r)); }
static void end_f64(double r) { printf(" f64=%016llx\n", d2u(r)); }
static void end_void(void) { printf("\n"); }

static void end_ext(long double r) {
    printf(" st0=");
    put_hex(&r, 10);
    printf("\n");
}

static void end_xmm(__m128 r) {
    printf(" xmm0=");
    put_hex(&r, 16);
    printf("\n");
}

#define DEFAULT_CW 0x027F
#define RESTORE_CW 0x037F

static void t_add3(const char *id, int a, int b, int c) {
    char args[96];
    snprintf(args, sizeof args, "i32:%d,i32:%d,i32:%d", a, b, c);
    begin(id, "fx_add3", "cdecl", "i32", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    int r = fx_add3(a, b, c);
    fpu_set(RESTORE_CW);
    end_i32(r);
}

static void t_mix(const char *id, int a, unsigned b) {
    char args[96];
    snprintf(args, sizeof args, "i32:%d,u32:%u", a, b);
    begin(id, "fx_mix", "stdcall", "i32", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    int r = fx_mix(a, b);
    fpu_set(RESTORE_CW);
    end_i32(r);
}

static void t_this_sum(const char *id, int a, int b, int k) {
    Obj o = {a, b, 1.5f, 0x11223344u};
    char args[96];
    snprintf(args, sizeof args, "ptr:obj,i32:%d", k);
    begin(id, "fx_this_sum", "thiscall", "i32", DEFAULT_CW, args);
    buf_in("obj", &o, sizeof o);
    fpu_set(DEFAULT_CW);
    int r = fx_this_sum(&o, k);
    fpu_set(RESTORE_CW);
    end_i32(r);
}

static void t_fast4(const char *id, int a, int b, int c, int d) {
    char args[128];
    snprintf(args, sizeof args, "i32:%d,i32:%d,i32:%d,i32:%d", a, b, c, d);
    begin(id, "fx_fast4", "fastcall", "i32", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    int r = fx_fast4(a, b, c, d);
    fpu_set(RESTORE_CW);
    end_i32(r);
}

static void t_lerp(const char *id, float a, float b, float t) {
    char args[128];
    snprintf(args, sizeof args, "f32:0x%08x,f32:0x%08x,f32:0x%08x", f2u(a), f2u(b), f2u(t));
    begin(id, "fx_lerp", "cdecl", "f32_x87", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    float r = fx_lerp(a, b, t);
    fpu_set(RESTORE_CW);
    end_f32(r);
}

static void t_div_d(const char *id, unsigned cw, double a, double b) {
    char args[128];
    snprintf(args, sizeof args, "f64:0x%016llx,f64:0x%016llx", d2u(a), d2u(b));
    begin(id, "fx_div_d", "cdecl", "f64_x87", cw, args);
    fpu_set(cw);
    double r = fx_div_d(a, b);
    fpu_set(RESTORE_CW);
    end_f64(r);
}

static void t_ext_mix(const char *id, unsigned cw, double a, double b) {
    char args[128];
    snprintf(args, sizeof args, "f64:0x%016llx,f64:0x%016llx", d2u(a), d2u(b));
    begin(id, "fx_ext_mix", "cdecl", "f64_x87", cw, args);
    fpu_set(cw);
    long double r = fx_ext_mix(a, b);
    fpu_set(RESTORE_CW);
    end_ext(r);
}

static void t_sin(const char *id, double x) {
    char args[64];
    snprintf(args, sizeof args, "f64:0x%016llx", d2u(x));
    begin(id, "fx_sin", "cdecl", "f64_x87", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    long double r = fx_sin(x);
    fpu_set(RESTORE_CW);
    end_ext(r);
}

static void t_atan2(const char *id, double y, double x) {
    char args[128];
    snprintf(args, sizeof args, "f64:0x%016llx,f64:0x%016llx", d2u(y), d2u(x));
    begin(id, "fx_atan2", "cdecl", "f64_x87", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    long double r = fx_atan2(y, x);
    fpu_set(RESTORE_CW);
    end_ext(r);
}

static void t_quat(const char *id, float x, float y, float z, float w) {
    float q[4] = {x, y, z, w};
    begin(id, "fx_quat_normalize", "cdecl", "void", DEFAULT_CW, "ptr:q");
    buf_in("q", q, sizeof q);
    fpu_set(DEFAULT_CW);
    fx_quat_normalize(q);
    fpu_set(RESTORE_CW);
    buf_out("q", q, sizeof q);
    end_void();
}

static void t_rsqrt_xmm(const char *id, float x) {
    char args[32];
    snprintf(args, sizeof args, "f32:0x%08x", f2u(x));
    begin(id, "fx_rsqrt_xmm", "cdecl", "xmm0", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    __m128 r = fx_rsqrt_xmm(x);
    fpu_set(RESTORE_CW);
    end_xmm(r);
}

static void t_rsqrt_f(const char *id, float x) {
    char args[32];
    snprintf(args, sizeof args, "f32:0x%08x", f2u(x));
    begin(id, "fx_rsqrt_f", "cdecl", "f32_x87", DEFAULT_CW, args);
    fpu_set(DEFAULT_CW);
    float r = fx_rsqrt_f(x);
    fpu_set(RESTORE_CW);
    end_f32(r);
}

static void t_obj_set(const char *id, int a, int b) {
    Obj o = {10, 20, 0.0f, 0};
    char args[64];
    snprintf(args, sizeof args, "ptr:obj,i32:%d,i32:%d", a, b);
    begin(id, "fx_obj_set", "thiscall", "i32", DEFAULT_CW, args);
    buf_in("obj", &o, sizeof o);
    fpu_set(DEFAULT_CW);
    int r = fx_obj_set(&o, a, b);
    fpu_set(RESTORE_CW);
    buf_out("obj", &o, sizeof o);
    end_i32(r);
}

static void t_fill(const char *id, int seed) {
    Obj o;
    memset(&o, 0xAA, sizeof o);
    char args[48];
    snprintf(args, sizeof args, "ptr:obj,i32:%d", seed);
    begin(id, "fx_fill", "cdecl", "void", DEFAULT_CW, args);
    buf_in("obj", &o, sizeof o);
    fpu_set(DEFAULT_CW);
    fx_fill(&o, seed);
    fpu_set(RESTORE_CW);
    buf_out("obj", &o, sizeof o);
    end_void();
}

static void t_seh_catch(const char *id, int use_null) {
    Obj o = {1, 2, 3.0f, 4};
    begin(id, "fx_seh_catch", "cdecl", "i32", DEFAULT_CW, use_null ? "ptr:null" : "ptr:obj");
    buf_in("obj", &o, sizeof o);
    fpu_set(DEFAULT_CW);
    int r = fx_seh_catch(use_null ? NULL : &o.a);
    fpu_set(RESTORE_CW);
    end_i32(r);
}

static int selftest(void) {
    t_add3("add3_a", 5, 7, 100);
    t_add3("add3_b", -1, 0x7fffffff, 3);
    t_add3("add3_c", 0, 0, 0);
    t_mix("mix_a", 3, 0x10);
    t_mix("mix_b", -7, 0xdeadbeefu);
    t_this_sum("this_a", 3, 4, 5);
    t_this_sum("this_b", -10, 2, 100);
    t_fast4("fast_a", 10, 3, 4, 5);
    t_fast4("fast_b", -1, -2, 0x10000, 0x10000);
    t_lerp("lerp_a", 1.0f, 3.0f, 0.25f);
    t_lerp("lerp_b", -2.5f, 7.75f, 0.1f);
    t_lerp("lerp_c", 0.0f, 1e30f, 1e-8f);
    t_div_d("div_a", 0x027F, 1.0, 3.0);
    t_div_d("div_b", 0x037F, 1.0, 3.0);
    t_div_d("div_c", 0x037F, 2.0, 7.0);
    t_div_d("div_d", 0x007F, 10.0, 3.0);
    t_ext_mix("ext_53", 0x027F, 1.0, 3.0);
    t_ext_mix("ext_64", 0x037F, 1.0, 3.0);
    t_ext_mix("ext_24", 0x007F, 1.0, 3.0);
    t_ext_mix("ext_trunc", 0x0F7F, 1.0, 3.0);
    t_ext_mix("ext_up", 0x0B7F, 1.0, 3.0);
    t_ext_mix("ext_b", 0x037F, 0.1, 7.0);
    t_sin("sin_a", 0.5);
    t_sin("sin_pi", 3.141592653589793);
    t_sin("sin_big", 1.0e10);
    t_sin("sin_small", 1.0e-5);
    t_atan2("atan2_a", 1.0, 2.0);
    t_atan2("atan2_b", -3.5, 0.25);
    t_quat("quat_a", 0.1f, 0.2f, 0.3f, 0.9f);
    t_quat("quat_b", 1.0f, 2.0f, 3.0f, 4.0f);
    t_quat("quat_c", 0.0f, 0.0f, 0.0f, 1.0f);
    t_quat("quat_d", 123.456f, -0.001f, 7.0f, 0.5f);
    float xs[] = {1.0f, 2.0f, 0.5f, 3.0f, 0.1f, 12345.678f, 1e-20f, 1e20f, 0.7071f, 9.0f};
    for (int i = 0; i < (int)(sizeof xs / sizeof xs[0]); i++) {
        char id[32];
        snprintf(id, sizeof id, "rsqrt_xmm_%d", i);
        t_rsqrt_xmm(id, xs[i]);
        snprintf(id, sizeof id, "rsqrt_f_%d", i);
        t_rsqrt_f(id, xs[i]);
    }
    t_obj_set("objset_a", 5, 9);
    t_obj_set("objset_b", -3, 70000);
    t_fill("fill_a", 4);
    t_fill("fill_b", -9);
    t_seh_catch("seh_catch_ok", 0);
    t_seh_catch("seh_catch_null", 1);
    return 0;
}

/* ---- loop mode (nv-probe tests) ---- */

static int do_loop(int n) {
    Obj obj = {0, 0, 0.0f, 0};
    /* The compiler can see which fields the callees read and would drop the
     * stores for the rest; this makes the whole struct observable. */
    __asm__ volatile("" : : "r"(&obj) : "memory");
    long long sum = 0;
    float fsum = 0.0f;
    for (int i = 0; i < n; i++) {
        sum += fx_add3(i, i + 1, 100);
        sum += fx_obj_set(&obj, i, i * 2);
        fsum += fx_lerp((float)i, 10.0f, 0.5f);
        sum += fx_mix(i, 0x10u);
        sum += fx_this_sum(&obj, 3);
        sum += fx_fact(4);
        sum += fx_reloc(i);
        sum += fx_thunk_sub3(i, 1, 2);
    }
    sum += fx_tiny(0) + fx_tiny(1);
    printf("loop n=%d sum=%lld fsum=%08x obj=%d,%d,%08x\n", n, sum, f2u(fsum), obj.a, obj.b, (unsigned)obj.tag);
    fflush(stdout);
    return 0;
}

/* ---- several threads calling a hooked function (threads mode) ---- */

typedef struct ThreadArgs {
    int iterations;
    volatile long bad;
} ThreadArgs;

static DWORD WINAPI thread_main(LPVOID p) {
    ThreadArgs *a = (ThreadArgs *)p;
    for (int i = 0; i < a->iterations; i++) {
        if (fx_fact(5) != 120) InterlockedIncrement(&a->bad);
        if ((i & 7) == 0) Sleep(0);
    }
    return 0;
}

static int do_threads(int count, int iterations) {
    HANDLE h[16];
    ThreadArgs args = {iterations, 0};
    if (count > 16) count = 16;
    for (int i = 0; i < count; i++) h[i] = CreateThread(NULL, 0, thread_main, &args, 0, NULL);
    WaitForMultipleObjects((DWORD)count, h, TRUE, INFINITE);
    printf("threads done count=%d iterations=%d bad=%ld\n", count, iterations, args.bad);
    fflush(stdout);
    return args.bad != 0;
}

/* ---- plugin-loader simulation (nvse-sim mode) ---- */

typedef struct PluginInfo {
    uint32_t infoVersion;
    const char *name;
    uint32_t version;
} PluginInfo;

typedef struct NVSEInterface {
    uint32_t nvseVersion;
    uint32_t runtimeVersion;
    uint32_t editorVersion;
    uint32_t isEditor;
    void *reserved[32];
} NVSEInterface;

typedef unsigned char (*QueryFn)(const NVSEInterface *, PluginInfo *);
typedef unsigned char (*LoadFn)(const NVSEInterface *);

static int nvse_sim(const char *dll, int editor, int unload, int wait_ms) {
    HMODULE h = LoadLibraryA(dll);
    if (!h) {
        printf("nvse-sim: cannot load %s (error %lu)\n", dll, GetLastError());
        return 1;
    }
    QueryFn query = (QueryFn)GetProcAddress(h, "NVSEPlugin_Query");
    LoadFn load = (LoadFn)GetProcAddress(h, "NVSEPlugin_Load");
    if (!query || !load) {
        printf("nvse-sim: missing exports query=%p load=%p\n", (void *)query, (void *)load);
        return 1;
    }
    NVSEInterface nvse;
    memset(&nvse, 0, sizeof nvse);
    nvse.nvseVersion = 0x06010000;
    nvse.runtimeVersion = 0x040020D0;
    nvse.isEditor = (uint32_t)editor;
    PluginInfo info;
    memset(&info, 0, sizeof info);
    int q = query(&nvse, &info) & 0xff;
    printf("nvse-sim: query=%d info_version=%u name=%s version=%u\n", q, info.infoVersion, info.name ? info.name : "(null)",
           info.version);
    int l = load(&nvse) & 0xff;
    printf("nvse-sim: load=%d\n", l);
    fflush(stdout);
    do_loop(3);
    if (unload) {
        FreeLibrary(h);
        printf("nvse-sim: unloaded\n");
        do_loop(2);
    }
    if (wait_ms > 0) Sleep((DWORD)wait_ms);
    return 0;
}

/* ---- unload the probe while a thread is inside a hooked function ---- */

static DWORD WINAPI sleepy_thread(LPVOID p) {
    *(int *)p = fx_sleepy(400);
    return 0;
}

static int unload_inside(const char *dll) {
    HMODULE h = LoadLibraryA(dll);
    if (!h) {
        printf("unload-inside: cannot load %s (error %lu)\n", dll, GetLastError());
        return 1;
    }
    int result = 0;
    HANDLE t = CreateThread(NULL, 0, sleepy_thread, &result, 0, NULL);
    /* Wait until the thread is inside the function (and then a moment more,
     * so that it is in Sleep), well before its 400 ms are over. */
    for (int i = 0; i < 500 && !fx_inside_sleepy; i++) Sleep(10);
    Sleep(50);
    FreeLibrary(h);
    printf("unload-inside: freed\n");
    fflush(stdout);
    WaitForSingleObject(t, INFINITE);
    printf("unload-inside: sleepy returned %d\n", result);
    fflush(stdout);
    return 0;
}

int main(int argc, char **argv) {
    if (argc >= 2 && !strcmp(argv[1], "selftest")) return selftest();
    if (argc >= 2 && !strcmp(argv[1], "loop")) {
        if (argc >= 4) {
            printf("waiting pid=%lu\n", GetCurrentProcessId());
            fflush(stdout);
            Sleep((DWORD)atoi(argv[3])); /* optional delay in ms before starting */
        }
        return do_loop(argc >= 3 ? atoi(argv[2]) : 5);
    }
    if (argc >= 4 && !strcmp(argv[1], "threads")) {
        if (argc >= 5) Sleep((DWORD)atoi(argv[4]));
        return do_threads(atoi(argv[2]), atoi(argv[3]));
    }
    if (argc >= 3 && !strcmp(argv[1], "nvse-sim")) {
        int editor = argc >= 4 && !strcmp(argv[3], "editor");
        int unload = 0, wait_ms = 0;
        for (int i = 3; i < argc; i++) {
            if (!strcmp(argv[i], "free")) unload = 1;
            if (!strcmp(argv[i], "wait") && i + 1 < argc) wait_ms = atoi(argv[i + 1]);
        }
        return nvse_sim(argv[2], editor, unload, wait_ms);
    }
    if (argc >= 3 && !strcmp(argv[1], "unload-inside")) return unload_inside(argv[2]);
    fprintf(stderr,
            "usage: target selftest | loop [N [ms]] | threads T N [ms] | nvse-sim <dll> [editor] [free] [wait <ms>] | "
            "unload-inside <dll>\n");
    return 2;
}

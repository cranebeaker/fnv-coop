/*
 * Synthetic 32-bit PE used to test the Ghidra scripts in this directory.
 *
 * Everything in here is written from scratch for the test. It is not a
 * model of any game code. It only contains shapes the scripts must cope
 * with: a command-info table with 40-byte entries, handlers that are
 * reachable only through data pointers, float constants of each width,
 * an x87 transcendental, an SSE reciprocal square root, a thiscall method,
 * SSE conversions that read a constant from memory, call chains of several
 * depths, a function that is only called through a function-pointer table,
 * calls through the import table, and pointer tables that the vtable test
 * gives the label names of a vtable.
 *
 * Build: see run-tests.sh (i686-w64-mingw32-gcc -O1 -msse2).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <emmintrin.h>
#include <xmmintrin.h>

#define NOINLINE __attribute__((noinline, used))

/* ---- command-info table: 40 bytes per entry ---- */

typedef struct CmdInfo {
    const char *long_name; /* +0  */
    const char *short_name; /* +4  */
    uint32_t opcode;       /* +8  */
    const char *help;      /* +12 */
    uint16_t needs_parent; /* +16 */
    uint16_t param_count;  /* +18 */
    void *params;          /* +20 */
    void *execute;         /* +24 */
    void *parse;           /* +28 */
    void *eval;            /* +32 */
    uint32_t flags;        /* +36 */
} CmdInfo;

_Static_assert(sizeof(CmdInfo) == 40, "command-info entry must be 40 bytes");

static int g_state_a;
static int g_state_b;
float g_gain = 2.5f; /* a writable float global, for the card test */
double g_scale = 0.125;

NOINLINE int cmd_alpha_execute(void *a, void *b, void *c, int d)
{
    g_state_a += d + (int)(intptr_t)a;
    return g_state_a;
}

NOINLINE int cmd_alpha_eval(void *a, void *b, void *c, int d)
{
    return g_state_a ^ (int)(intptr_t)b ^ d;
}

NOINLINE int cmd_beta_execute(void *a, void *b, void *c, int d)
{
    g_state_b = g_state_b * 31 + d;
    return g_state_b - (int)(intptr_t)c;
}

NOINLINE int cmd_gamma_execute(void *a, void *b, void *c, int d)
{
    return (int)(intptr_t)a * 7 + d * 3 + g_state_a;
}

NOINLINE int cmd_delta_execute(void *a, void *b, void *c, int d)
{
    return d - g_state_b * 5 + (int)(intptr_t)b;
}

/* one pointer is both the execute and the eval handler of a single entry */
NOINLINE int cmd_both(void *a, void *b, void *c, int d)
{
    return g_state_b + g_state_a * 3 + d;
}

/* shared by two entries, so it must not be renamed from either */
NOINLINE int cmd_shared_parse(void *a, void *b, void *c, int d)
{
    return d + 1000 + (int)(intptr_t)c;
}

static const char s_long_alpha[] = "FixtureAlpha";
static const char s_short_alpha[] = "FxA";
static const char s_help_alpha[] = "fixture command alpha";
static const char s_long_beta[] = "FixtureBeta";
static const char s_short_beta[] = "FxB";
static const char s_long_gamma[] = "Fixture-Gamma.Odd";
static const char s_long_delta[] = "FixtureDelta";
static const char s_short_delta[] = "FxD";
static const char s_long_both[] = "FixtureBoth";
static const char s_bad[] = {'B', 'a', 'd', 0x01, 'N', 'a', 'm', 'e', 0};
static const char s_long_badcode[] = "FixtureBadCode";

/* A routine whose bytes are no instruction (FF FF ...), reached only through a table entry.
 * The label script must not turn it into a function. */
__asm__(".text\n"
        ".globl _cmd_badcode_execute\n"
        "_cmd_badcode_execute:\n"
        ".byte 0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff\n"
        ".text\n");
extern int cmd_badcode_execute(void *a, void *b, void *c, int d);

CmdInfo g_commands[] = {
    {s_long_alpha, s_short_alpha, 0x1000, s_help_alpha, 0, 1, 0,
     (void *)cmd_alpha_execute, 0, (void *)cmd_alpha_eval, 0},
    {s_long_beta, s_short_beta, 0x1001, 0, 0, 0, 0,
     (void *)cmd_beta_execute, (void *)cmd_shared_parse, 0, 1},
    {s_long_gamma, 0, 0x1002, 0, 1, 2, 0,
     (void *)cmd_gamma_execute, (void *)cmd_shared_parse, 0, 0},
    /* execute and eval pointers are null: nothing to label */
    {s_long_delta, s_short_delta, 0x1003, 0, 0, 0, 0, 0, 0, 0, 0},
    /* long-name pointer is null: invalid entry */
    {0, 0, 0x1004, 0, 0, 0, 0, (void *)cmd_delta_execute, 0, 0, 0},
    /* long-name text holds a control character: invalid entry */
    {s_bad, 0, 0x1005, 0, 0, 0, 0, 0, 0, 0, 0},
    /* the same function is the execute and the eval handler */
    {s_long_both, 0, 0x1006, 0, 0, 0, 0,
     (void *)cmd_both, 0, (void *)cmd_both, 0},
    /* terminator */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* After the terminator, so the runs that read eight entries never reach it. The execute
     * pointer leads to bytes that are no instruction and the parse pointer to the inside of a
     * function: the label script must leave both alone. */
    {s_long_badcode, 0, 0x1007, 0, 0, 0, 0,
     (void *)cmd_badcode_execute, (void *)((char *)cmd_alpha_execute + 4), 0, 0},
};

/* ---- float constants: x87, SSE, extended ---- */

NOINLINE float fx_angle_to_unit(float deg)
{
    /* x87: fmul by 0.0174533f then fadd 3.5f */
    return deg * 0.0174533f + 3.5f;
}

NOINLINE double fx_double_scale(double x)
{
    return x * 2.718281828459045 - g_scale;
}

NOINLINE long double fx_extended(long double x)
{
    return x * 0.3333333333333333333L;
}

NOINLINE int fx_possible_float_immediate(int x)
{
    /* 0x40490fdb is 3.14159274f, but used here as a plain integer mask */
    return x ^ 0x40490fdb;
}

NOINLINE float fx_sse_scalar(float x, float y)
{
    __m128 v = _mm_set_ss(x);
    v = _mm_mul_ss(v, _mm_set_ss(y));
    v = _mm_add_ss(v, _mm_set_ss(0.5f));
    v = _mm_sqrt_ss(v);
    return _mm_cvtss_f32(v);
}

NOINLINE float fx_rsqrt(float x)
{
    return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x)));
}

NOINLINE float fx_rcp(float x)
{
    return _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x)));
}

NOINLINE double fx_sin(double x)
{
    double r;
    __asm__("fsin" : "=t"(r) : "0"(x));
    return r;
}

NOINLINE void fx_set_cw(unsigned short cw)
{
    __asm__ volatile("fldcw %0" : : "m"(cw));
}

/* control words read from constants in memory */
const unsigned short k_cw_const = 0x027f; /* 53-bit precision, round to nearest, all exceptions masked */
const unsigned k_mxcsr_const = 0x9fc0;    /* flush to zero and denormals are zero, round to nearest, masked */

NOINLINE void fx_load_cw_const(void)
{
    __asm__ volatile("fldcw %0" : : "m"(k_cw_const));
}

NOINLINE void fx_load_mxcsr_const(void)
{
    __asm__ volatile("ldmxcsr %0" : : "m"(k_mxcsr_const));
}

NOINLINE unsigned fx_sum(unsigned a, unsigned b)
{
    /* integer only, no globals: tier A, not stateful */
    return a * 3u + (b >> 2);
}

NOINLINE float fx_gain(float x)
{
    /* reads the g_gain global */
    return x * g_gain;
}

/* ---- SSE conversions that read a constant from memory ----
 * The source type is the first part of the instruction name, not the last:
 * CVTSD2SS reads a double. Each function below uses one memory form. */

const double k_cvt_double = 1.25;
const double k_cvt_double2 = 2.75;
const float k_cvt_float = 0.75f;
const float k_cvt_pair[2] = {1.5f, -2.0f};
const int k_cvt_ints[2] = {7, -9};
const int k_cvt_int = 11;
__attribute__((aligned(16))) const double k_cvt_doubles[2] = {3.5, -1.25};

NOINLINE float fx_cvt_sd2ss(void)
{
    float r;
    __asm__("cvtsd2ss %1, %0" : "=x"(r) : "m"(k_cvt_double));
    return r;
}

NOINLINE double fx_cvt_ss2sd(void)
{
    double r;
    __asm__("cvtss2sd %1, %0" : "=x"(r) : "m"(k_cvt_float));
    return r;
}

NOINLINE int fx_cvt_tss2si(void)
{
    int r;
    __asm__("cvttss2si %1, %0" : "=r"(r) : "m"(k_cvt_float));
    return r;
}

NOINLINE int fx_cvt_tsd2si(void)
{
    int r;
    __asm__("cvttsd2si %1, %0" : "=r"(r) : "m"(k_cvt_double2));
    return r;
}

NOINLINE double fx_cvt_ps2pd(void)
{
    __m128d r;
    __asm__("cvtps2pd %1, %0" : "=x"(r) : "m"(k_cvt_pair));
    return _mm_cvtsd_f64(r);
}

NOINLINE float fx_cvt_pd2ps(void)
{
    __m128 r;
    __asm__("cvtpd2ps %1, %0" : "=x"(r) : "m"(k_cvt_doubles));
    return _mm_cvtss_f32(r);
}

NOINLINE double fx_cvt_dq2pd(void)
{
    __m128d r;
    __asm__("cvtdq2pd %1, %0" : "=x"(r) : "m"(k_cvt_ints));
    return _mm_cvtsd_f64(r);
}

NOINLINE double fx_cvt_si2sd(void)
{
    double r;
    __asm__("cvtsi2sd %1, %0" : "=x"(r) : "m"(k_cvt_int));
    return r;
}

/* ---- call chains ----
 * Integer-only callers around code that needs a higher tier, at several
 * depths, plus a pair of functions that call each other. */

NOINLINE unsigned fx_sin_bits(unsigned x)
{
    return (unsigned)(int)fx_sin((double)x);
}

NOINLINE unsigned fx_sin_top(unsigned x)
{
    return fx_sin_bits(x) + 7u;
}

#define CHAIN_LINK(n, next) \
    NOINLINE unsigned fx_chain##n(unsigned x) { return next(x ^ n##u) + n##u; }

/* fx_chain0 -> ... -> fx_chain9 -> fx_sin_bits -> fx_sin: eleven calls deep */
CHAIN_LINK(9, fx_sin_bits)
CHAIN_LINK(8, fx_chain9)
CHAIN_LINK(7, fx_chain8)
CHAIN_LINK(6, fx_chain7)
CHAIN_LINK(5, fx_chain6)
CHAIN_LINK(4, fx_chain5)
CHAIN_LINK(3, fx_chain4)
CHAIN_LINK(2, fx_chain3)
CHAIN_LINK(1, fx_chain2)
CHAIN_LINK(0, fx_chain1)

NOINLINE unsigned fx_pong(unsigned n);

NOINLINE unsigned fx_ping(unsigned n)
{
    return n ? fx_pong(n - 1) + 1u : 0u;
}

NOINLINE unsigned fx_pong(unsigned n)
{
    return n ? fx_ping(n - 1) + 2u : 0u;
}

/* Integer-only code. The name-import test calls the stub _CIsin, as the
 * FunctionID database would for a stripped CRT routine, so a caller that
 * only reaches it through a wrapper must still be tier D. */
NOINLINE unsigned fx_crt_stub(unsigned x)
{
    return (x * 5u + 1u) ^ (x >> 3);
}

NOINLINE unsigned fx_crt_wrap(unsigned x)
{
    return fx_crt_stub(x) + 3u;
}

NOINLINE unsigned fx_crt_top(unsigned x)
{
    return fx_crt_wrap(x) + 7u;
}

/* ---- calls through the import table ----
 * "call dword ptr [__imp__cos]" has a known target (an external function), unlike a call
 * through a register or an indexed table. */

extern __declspec(dllimport) double __cdecl fx_imp_cos(double) __asm__("cos");
__declspec(dllimport) unsigned long __attribute__((stdcall)) GetTickCount(void);

NOINLINE double fx_import_cos(double x)
{
    return fx_imp_cos(x) + 1.0;
}

NOINLINE unsigned long fx_import_tick(unsigned long x)
{
    return GetTickCount() ^ x;
}

NOINLINE double fx_import_cos_top(double x)
{
    return fx_import_cos(x) * 2.0;
}

/* ---- SSE moves that load a constant: MOVQ and MOVD, not a float-named instruction ---- */

const double k_movq_const = 6.5;
const float k_movd_const = 9.25f;

NOINLINE double fx_movq_const(void)
{
    double r;
    __asm__("movq %1, %0" : "=x"(r) : "m"(k_movq_const));
    return r;
}

NOINLINE float fx_movd_const(void)
{
    float r;
    __asm__("movd %1, %0" : "=x"(r) : "m"(k_movd_const));
    return r;
}

/* ---- thiscall ---- */

typedef struct Obj {
    int base;
    int scale;
} Obj;

NOINLINE __attribute__((thiscall)) int Obj_Compute(Obj *self, int v)
{
    return self->base + self->scale * v;
}

/* ---- reachable only through a function-pointer table ---- */

NOINLINE int fp_only_target(int v)
{
    return v * 11 + 5;
}

NOINLINE int fp_other_target(int v)
{
    return v * 13 - 7;
}

static int (*const g_dispatch[])(int) = {fp_other_target, fp_only_target};

/* Call through an indexed table: the target depends on a register, so it stays unknown. */
NOINLINE int fx_table_call(int i)
{
    return g_dispatch[i & 1](i);
}

/* Two more pointer tables. The vtable test labels them (and g_dispatch) with the names a vtable
 * gets from Ghidra's class recovery and from PDB symbols. */
int (*const g_vt_pdb[])(int) __attribute__((used)) = {fp_only_target, fp_only_target, fp_other_target};
int (*const g_vt_brace[])(int) __attribute__((used)) = {fp_only_target, fp_only_target};

int main(int argc, char **argv)
{
    volatile int sel = argc;
    Obj o = {sel, 3};
    float f = (float)sel;
    double d = (double)sel;
    long double e = (long double)sel;
    int acc = 0;
    int idx = sel & 1;

    acc += Obj_Compute(&o, sel);
    acc += (int)fx_angle_to_unit(f);
    acc += (int)fx_double_scale(d);
    acc += (int)fx_extended(e);
    acc += fx_possible_float_immediate(sel);
    acc += (int)fx_sse_scalar(f, f);
    acc += (int)fx_rsqrt(f);
    acc += (int)fx_rcp(f);
    acc += (int)fx_sin(d);
    acc += (int)fx_sum((unsigned)sel, (unsigned)sel);
    acc += (int)fx_gain(f);
    fx_set_cw(0x027f);
    fx_load_cw_const();
    fx_load_mxcsr_const();
    acc += g_dispatch[idx](sel);
    acc += (int)fx_cvt_sd2ss();
    acc += (int)fx_cvt_ss2sd();
    acc += fx_cvt_tss2si();
    acc += fx_cvt_tsd2si();
    acc += (int)fx_cvt_ps2pd();
    acc += (int)fx_cvt_pd2ps();
    acc += (int)fx_cvt_dq2pd();
    acc += (int)fx_cvt_si2sd();
    acc += (int)fx_sin_top((unsigned)sel);
    acc += (int)fx_chain0((unsigned)sel);
    acc += (int)fx_ping((unsigned)sel & 3u);
    acc += (int)fx_crt_top((unsigned)sel);
    acc += (int)fx_import_cos_top(d);
    acc += (int)fx_import_tick((unsigned)sel);
    acc += fx_table_call(sel);
    acc += (int)fx_movq_const();
    acc += (int)fx_movd_const();

    /* handlers are called through the table only */
    {
        CmdInfo *c = &g_commands[sel & 3];
        int (*ex)(void *, void *, void *, int) = (int (*)(void *, void *, void *, int))c->execute;
        int (*ev)(void *, void *, void *, int) = (int (*)(void *, void *, void *, int))c->eval;
        int (*pa)(void *, void *, void *, int) = (int (*)(void *, void *, void *, int))c->parse;
        if (ex) acc += ex(0, 0, 0, sel);
        if (ev) acc += ev(0, 0, 0, sel);
        if (pa) acc += pa(0, 0, 0, sel);
    }

    printf("fixture %d %s\n", acc, argv[0]);
    return acc == 0x7fffffff;
}

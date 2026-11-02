/*
 * Second synthetic program used to test the nv-oracle tools. Written from
 * scratch for the tests; not a model of any game code.
 *
 * It exists for what the main fixture (target.c) cannot show:
 *   - a 4 MiB zero-filled array, so the image is much larger than the code of
 *     nv-call's host program and covers the host's startup code and the
 *     function the host registered as the process's exception filter;
 *   - a TLS callback that counts thread starts, to see whether the image's own
 *     callbacks run inside the harness;
 *   - functions that start a thread: a quiet one, and one that faults, which
 *     is an exception on a thread where the harness's handler is not on the
 *     chain.
 *
 * Mode "tls": start a thread and print how often the callback ran (natively
 * this is 1, which is what makes the harness's 0 mean something).
 *
 * Build: i686-w64-mingw32-gcc -O1 -fno-omit-frame-pointer -msse2.
 */
#include <stdio.h>
#include <string.h>
#include <windows.h>

#define FX __attribute__((noinline, noclone, used))

static char tx_big[4 * 1024 * 1024] __attribute__((used));
volatile LONG tx_tls_count;

static void NTAPI tx_tls_callback(PVOID module, DWORD reason, PVOID reserved) {
    (void)module;
    (void)reserved;
    if (reason == DLL_THREAD_ATTACH) InterlockedIncrement(&tx_tls_count);
}

PIMAGE_TLS_CALLBACK tx_callback __attribute__((section(".CRT$XLB"), used)) = tx_tls_callback;

static DWORD WINAPI quiet_thread(LPVOID p) {
    (void)p;
    return 0;
}

static DWORD WINAPI bad_thread(LPVOID p) {
    (void)p;
    *(volatile int *)0 = 1;
    return 0;
}

FX int tx_add(int a, int b) { return a + b + tx_big[0]; }

/* Starts a thread, waits for it, and returns how many thread starts the
 * image's TLS callback has seen. */
FX int tx_spawn(void) {
    HANDLE h = CreateThread(NULL, 0, quiet_thread, NULL, 0, NULL);
    if (!h) return -1;
    WaitForSingleObject(h, 5000);
    CloseHandle(h);
    return (int)tx_tls_count;
}

/* Starts a thread that writes to address 0, and waits for it. */
FX int tx_spawn_fault(void) {
    HANDLE h = CreateThread(NULL, 0, bad_thread, NULL, 0, NULL);
    if (!h) return -1;
    WaitForSingleObject(h, 5000);
    CloseHandle(h);
    return 9;
}

int main(int argc, char **argv) {
    if (argc >= 2 && !strcmp(argv[1], "tls")) {
        printf("tls_count=%d\n", tx_spawn());
        return 0;
    }
    fprintf(stderr, "usage: threadfx tls\n");
    return 2;
}

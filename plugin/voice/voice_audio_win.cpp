// Windows sound for voice chat. See voice_audio_win.h.
#include "voice_audio_win.h"

#include <windows.h>
#include <mmsystem.h>

#include <vector>

#include "common/nvcoop_api.h"

namespace nvcoop::voice {

namespace {
constexpr int kBlocks = 4;       // microphone blocks queued with Windows at once (80 ms)
constexpr int kPlayBlocks = 3;   // speaker blocks queued with Windows at once (60 ms: the less is queued, the less late a voice is)

WAVEFORMATEX Format(WORD channels) {
    WAVEFORMATEX f = {};
    f.wFormatTag = WAVE_FORMAT_PCM;
    f.nChannels = channels;
    f.nSamplesPerSec = kSampleRate;
    f.wBitsPerSample = 16;
    f.nBlockAlign = static_cast<WORD>(channels * 2);
    f.nAvgBytesPerSec = kSampleRate * f.nBlockAlign;
    return f;
}
UINT DeviceId(int device) { return device < 0 ? static_cast<UINT>(WAVE_MAPPER) : static_cast<UINT>(device); }
}  // namespace

void AudioCapture::start(int device, PcmQueue* sink) {
    stop();
    failed_ = false;
    running_ = true;
    thread_.start("voice_capture", [this, device, sink](const std::function<bool()>& stopRequested) {
        HANDLE ev = CreateEventA(nullptr, FALSE, FALSE, nullptr);
        WAVEFORMATEX fmt = Format(1);
        HWAVEIN in = nullptr;
        if (!ev || waveInOpen(&in, DeviceId(device), &fmt, reinterpret_cast<DWORD_PTR>(ev), 0, CALLBACK_EVENT) != MMSYSERR_NOERROR) {
            nvcoop::Log("voice: the microphone did not open; you can hear others but not talk (nvcoop.ini [voice] mic_device)");
            failed_ = true;
            running_ = false;
            if (ev) CloseHandle(ev);
            return;
        }
        std::vector<int16_t> buffers[kBlocks];
        WAVEHDR headers[kBlocks] = {};
        for (int i = 0; i < kBlocks; ++i) {
            buffers[i].assign(kFrameSamples, 0);
            headers[i].lpData = reinterpret_cast<LPSTR>(buffers[i].data());
            headers[i].dwBufferLength = static_cast<DWORD>(kFrameSamples * 2);
            waveInPrepareHeader(in, &headers[i], sizeof(WAVEHDR));
            waveInAddBuffer(in, &headers[i], sizeof(WAVEHDR));
        }
        waveInStart(in);
        nvcoop::Log("voice: microphone open");
        while (!stopRequested()) {
            WaitForSingleObject(ev, 40);
            for (int i = 0; i < kBlocks; ++i) {
                if (headers[i].dwFlags & WHDR_DONE) {
                    if (sink && headers[i].dwBytesRecorded >= 2) sink->push(buffers[i].data(), headers[i].dwBytesRecorded / 2);
                    headers[i].dwFlags &= ~static_cast<DWORD>(WHDR_DONE);
                    headers[i].dwBytesRecorded = 0;
                    waveInAddBuffer(in, &headers[i], sizeof(WAVEHDR));
                }
            }
        }
        waveInStop(in);
        waveInReset(in);
        for (auto& h : headers) waveInUnprepareHeader(in, &h, sizeof(WAVEHDR));
        waveInClose(in);
        CloseHandle(ev);
        running_ = false;
    });
}

void AudioCapture::stop() {
    thread_.stopAndJoin();
    running_ = false;
}

void AudioPlayback::start(int device, RenderFn render) {
    stop();
    failed_ = false;
    running_ = true;
    thread_.start("voice_playback", [this, device, render = std::move(render)](const std::function<bool()>& stopRequested) {
        HANDLE ev = CreateEventA(nullptr, FALSE, FALSE, nullptr);
        WAVEFORMATEX fmt = Format(2);
        HWAVEOUT out = nullptr;
        if (!ev || waveOutOpen(&out, DeviceId(device), &fmt, reinterpret_cast<DWORD_PTR>(ev), 0, CALLBACK_EVENT) != MMSYSERR_NOERROR) {
            nvcoop::Log("voice: the speakers did not open; you can talk but not hear others (nvcoop.ini [voice] speaker_device)");
            failed_ = true;
            running_ = false;
            if (ev) CloseHandle(ev);
            return;
        }
        std::vector<int16_t> buffers[kPlayBlocks];
        WAVEHDR headers[kPlayBlocks] = {};
        for (int i = 0; i < kPlayBlocks; ++i) {
            buffers[i].assign(kFrameSamples * 2, 0);
            headers[i].lpData = reinterpret_cast<LPSTR>(buffers[i].data());
            headers[i].dwBufferLength = static_cast<DWORD>(kFrameSamples * 4);
            waveOutPrepareHeader(out, &headers[i], sizeof(WAVEHDR));
            headers[i].dwFlags |= WHDR_DONE;   // free to fill
        }
        nvcoop::Log("voice: speakers open");
        while (!stopRequested()) {
            for (int i = 0; i < kPlayBlocks; ++i) {
                if (headers[i].dwFlags & WHDR_DONE) {
                    render(buffers[i].data(), kFrameSamples);
                    headers[i].dwFlags &= ~static_cast<DWORD>(WHDR_DONE);
                    waveOutWrite(out, &headers[i], sizeof(WAVEHDR));
                }
            }
            WaitForSingleObject(ev, 20);
        }
        waveOutReset(out);
        for (auto& h : headers) waveOutUnprepareHeader(out, &h, sizeof(WAVEHDR));
        waveOutClose(out);
        CloseHandle(ev);
        running_ = false;
    });
}

void AudioPlayback::stop() {
    thread_.stopAndJoin();
    running_ = false;
}

}  // namespace nvcoop::voice

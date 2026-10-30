// Windows sound for voice chat, on WASAPI in shared mode. See voice_audio_win.h.
//
// Both directions follow the same shape: open the chosen (or default) endpoint with its own mix format, run an event-driven loop
// that moves audio between the endpoint and the rest of the voice code, and if the endpoint goes away (unplugged, disabled, driver
// reset) close it and keep trying to open it again. The conversion between the device's format and the codec's 16 kHz is done in
// voice_resample.* (game-free, tested); this file only talks to Windows.
#include "voice_audio_win.h"

#include <windows.h>
#include <audioclient.h>
#include <mmdeviceapi.h>
#include <propidl.h>

#include <string>
#include <vector>

#include "common/nvcoop_api.h"
#include "voice_resample.h"

namespace nvcoop::voice {

namespace {

// ---- small COM helpers (our own; nothing from WRL / ATL) ----

template <class T>
class Com {
public:
    Com() = default;
    ~Com() { reset(); }
    Com(const Com&) = delete;
    Com& operator=(const Com&) = delete;
    T** put() {
        reset();
        return &p_;
    }
    T* get() const { return p_; }
    T* operator->() const { return p_; }
    explicit operator bool() const { return p_ != nullptr; }
    void reset() {
        if (p_) {
            p_->Release();
            p_ = nullptr;
        }
    }

private:
    T* p_ = nullptr;
};

// COM for the lifetime of one audio thread.
class ComApartment {
public:
    ComApartment() {
        const HRESULT hr = CoInitializeEx(nullptr, COINIT_MULTITHREADED);
        owns_ = SUCCEEDED(hr);   // RPC_E_CHANGED_MODE: somebody else set the thread up; use it, do not undo it
    }
    ~ComApartment() {
        if (owns_) CoUninitialize();
    }

private:
    bool owns_ = false;
};

struct MixFormatPtr {
    WAVEFORMATEX* p = nullptr;
    ~MixFormatPtr() {
        if (p) CoTaskMemFree(p);
    }
};

unsigned long Code(HRESULT hr) { return static_cast<unsigned long>(hr); }

// Sleeps up to `ms` in small steps and returns early (true) when a stop was asked for.
bool SleepOrStop(const std::function<bool()>& stopRequested, DWORD ms) {
    for (DWORD waited = 0; waited < ms; waited += 20) {
        if (stopRequested()) return true;
        Sleep(20);
    }
    return stopRequested();
}

std::string Narrow(const wchar_t* w) {
    if (!w) return std::string();
    const int n = WideCharToMultiByte(CP_UTF8, 0, w, -1, nullptr, 0, nullptr, nullptr);
    if (n <= 1) return std::string();
    std::string s(static_cast<size_t>(n), '\0');
    WideCharToMultiByte(CP_UTF8, 0, w, -1, &s[0], n, nullptr, nullptr);
    s.resize(static_cast<size_t>(n) - 1);
    return s;
}

// The device's friendly-name property (PKEY_Device_FriendlyName), spelled out so no extra GUID library is needed.
const PROPERTYKEY kFriendlyName = {{0xa45c254e, 0xdf1c, 0x4efd, {0x80, 0x20, 0x67, 0xd1, 0x46, 0xa8, 0x50, 0xe0}}, 14};

std::string NameOf(IMMDevice* dev) {
    Com<IPropertyStore> props;
    if (!dev || FAILED(dev->OpenPropertyStore(STGM_READ, props.put()))) return "?";
    PROPVARIANT v;
    PropVariantInit(&v);
    std::string name = "?";
    if (SUCCEEDED(props->GetValue(kFriendlyName, &v)) && v.vt == VT_LPWSTR) name = Narrow(v.pwszVal);
    PropVariantClear(&v);
    return name;
}

// Lists the active endpoints with the numbers `mic_device` / `speaker_device` take.
void LogDevices(IMMDeviceEnumerator* en, EDataFlow flow) {
    Com<IMMDeviceCollection> list;
    UINT n = 0;
    if (FAILED(en->EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE, list.put())) || FAILED(list->GetCount(&n))) return;
    for (UINT i = 0; i < n; ++i) {
        Com<IMMDevice> d;
        if (SUCCEEDED(list->Item(i, d.put())))
            nvcoop::Log("voice: %s %u = %s", flow == eCapture ? "microphone" : "speakers", static_cast<unsigned>(i), NameOf(d.get()).c_str());
    }
}

// Maps what WASAPI reports to our own description. False for anything that is not plain PCM or float PCM.
bool Describe(const WAVEFORMATEX* w, DeviceFormat& out) {
    if (!w || w->nChannels == 0 || w->nSamplesPerSec == 0) return false;
    WORD tag = w->wFormatTag;
    if (tag == WAVE_FORMAT_EXTENSIBLE && w->cbSize >= sizeof(WAVEFORMATEXTENSIBLE) - sizeof(WAVEFORMATEX)) {
        const auto* ext = reinterpret_cast<const WAVEFORMATEXTENSIBLE*>(w);
        // The sub-format GUIDs of PCM and IEEE float differ from the plain format tags only in their first field.
        tag = ext->SubFormat.Data1 == 1 ? WAVE_FORMAT_PCM : ext->SubFormat.Data1 == 3 ? WAVE_FORMAT_IEEE_FLOAT : WORD(0);
    }
    out.rate = w->nSamplesPerSec;
    out.channels = w->nChannels;
    if (tag == WAVE_FORMAT_IEEE_FLOAT && w->wBitsPerSample == 32) {
        out.kind = SampleKind::Float32;
        return true;
    }
    if (tag == WAVE_FORMAT_PCM) {
        if (w->wBitsPerSample == 16) out.kind = SampleKind::Int16;
        else if (w->wBitsPerSample == 24) out.kind = SampleKind::Int24;
        else if (w->wBitsPerSample == 32) out.kind = SampleKind::Int32;
        else return false;
        return true;
    }
    return false;
}

// One open endpoint.
struct Stream {
    Com<IAudioClient> client;
    Com<IAudioCaptureClient> capture;
    Com<IAudioRenderClient> render;
    HANDLE event = nullptr;
    DeviceFormat format;
    UINT32 bufferFrames = 0;
    UINT32 periodFrames = 0;   // the engine's wake-up period
    bool started = false;

    ~Stream() { close(); }
    void close() {
        if (client && started) client->Stop();
        started = false;
        capture.reset();
        render.reset();
        client.reset();
        if (event) {
            CloseHandle(event);
            event = nullptr;
        }
    }
};

// Opens an endpoint in shared, event-driven mode. `device` < 0 (or out of range) = the default endpoint. On failure returns false
// and `why` says where it stopped.
bool OpenStream(EDataFlow flow, int device, bool firstTry, Stream& s, std::string& name, const char*& why) {
    Com<IMMDeviceEnumerator> en;
    HRESULT hr = CoCreateInstance(__uuidof(MMDeviceEnumerator), nullptr, CLSCTX_ALL, IID_PPV_ARGS(en.put()));
    if (FAILED(hr)) {
        why = "no device enumerator";
        return false;
    }
    if (firstTry) LogDevices(en.get(), flow);

    Com<IMMDevice> dev;
    if (device >= 0) {
        Com<IMMDeviceCollection> list;
        UINT n = 0;
        if (SUCCEEDED(en->EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE, list.put())) && SUCCEEDED(list->GetCount(&n)) && static_cast<UINT>(device) < n)
            list->Item(static_cast<UINT>(device), dev.put());
        else if (firstTry)
            nvcoop::Log("voice: no %s numbered %d, using the system default", flow == eCapture ? "microphone" : "speakers", device);
    }
    if (!dev && FAILED(en->GetDefaultAudioEndpoint(flow, eConsole, dev.put()))) {
        why = flow == eCapture ? "no microphone" : "no speakers";
        return false;
    }
    name = NameOf(dev.get());

    if (FAILED(dev->Activate(__uuidof(IAudioClient), CLSCTX_ALL, nullptr, reinterpret_cast<void**>(s.client.put())))) {
        why = "could not activate the device";
        return false;
    }
    MixFormatPtr mix;
    if (FAILED(s.client->GetMixFormat(&mix.p)) || !mix.p) {
        why = "no mix format";
        return false;
    }
    REFERENCE_TIME defPeriod = 0, minPeriod = 0;
    s.client->GetDevicePeriod(&defPeriod, &minPeriod);
    if (defPeriod <= 0) defPeriod = 100000;   // 10 ms

    DWORD flags = AUDCLNT_STREAMFLAGS_EVENTCALLBACK;
    const WAVEFORMATEX* use = mix.p;
    WAVEFORMATEX fallback = {};
    if (!Describe(mix.p, s.format)) {
        // A format we do not read: ask the engine for plain 16 bit at the same rate and channel count and let it convert.
        fallback.wFormatTag = WAVE_FORMAT_PCM;
        fallback.nChannels = mix.p->nChannels;
        fallback.nSamplesPerSec = mix.p->nSamplesPerSec;
        fallback.wBitsPerSample = 16;
        fallback.nBlockAlign = static_cast<WORD>(fallback.nChannels * 2);
        fallback.nAvgBytesPerSec = fallback.nSamplesPerSec * fallback.nBlockAlign;
        use = &fallback;
        flags |= AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM | AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY;
        Describe(&fallback, s.format);
    }
    // A buffer of three engine periods; the playback loop keeps only about two of them filled, so a roomy buffer costs no delay.
    hr = s.client->Initialize(AUDCLNT_SHAREMODE_SHARED, flags, defPeriod * 3, 0, use, nullptr);
    if (FAILED(hr)) {
        why = "the device refused the format";
        return false;
    }
    s.event = CreateEventW(nullptr, FALSE, FALSE, nullptr);
    if (!s.event || FAILED(s.client->SetEventHandle(s.event))) {
        why = "no event";
        return false;
    }
    if (FAILED(s.client->GetBufferSize(&s.bufferFrames))) {
        why = "no buffer size";
        return false;
    }
    s.periodFrames = static_cast<UINT32>((static_cast<unsigned long long>(defPeriod) * s.format.rate + 9999999ull) / 10000000ull);
    if (flow == eCapture) hr = s.client->GetService(__uuidof(IAudioCaptureClient), reinterpret_cast<void**>(s.capture.put()));
    else hr = s.client->GetService(__uuidof(IAudioRenderClient), reinterpret_cast<void**>(s.render.put()));
    if (FAILED(hr)) {
        why = "no capture / render service";
        return false;
    }
    return true;
}

// Moves microphone packets to the sink until stopped (true) or until the device fails (false).
bool RunCapture(Stream& s, PcmQueue* sink, const std::function<bool()>& stopRequested, HRESULT& error) {
    CaptureConverter conv(s.format);
    std::vector<int16_t> out;
    error = s.client->Start();
    if (FAILED(error)) return false;
    s.started = true;
    while (!stopRequested()) {
        WaitForSingleObject(s.event, 50);   // a missed event only delays the next pass by this long
        for (;;) {
            UINT32 packet = 0;
            error = s.capture->GetNextPacketSize(&packet);
            if (FAILED(error)) return false;
            if (packet == 0) break;
            BYTE* data = nullptr;
            UINT32 frames = 0;
            DWORD flags = 0;
            error = s.capture->GetBuffer(&data, &frames, &flags, nullptr, nullptr);
            if (FAILED(error)) return false;
            if (frames > 0) {
                out.clear();
                conv.feed(data, frames, (flags & AUDCLNT_BUFFERFLAGS_SILENT) != 0, out);
                if (sink && !out.empty()) sink->push(out.data(), out.size());
            }
            error = s.capture->ReleaseBuffer(frames);
            if (FAILED(error)) return false;
        }
    }
    return true;
}

// Keeps the speaker buffer about two engine periods full with what `render` produces, until stopped (true) or the device fails.
bool RunPlayback(Stream& s, const AudioPlayback::RenderFn& render, const std::function<bool()>& stopRequested, HRESULT& error) {
    PlaybackFeeder feeder(s.format, render);
    UINT32 target = s.periodFrames * 2;
    if (target < s.format.rate / 50) target = s.format.rate / 50;   // at least 20 ms
    if (target > s.bufferFrames) target = s.bufferFrames;

    auto topUp = [&](UINT32 frames) -> bool {
        if (frames == 0) return true;
        BYTE* data = nullptr;
        error = s.render->GetBuffer(frames, &data);
        if (FAILED(error)) return false;
        feeder.fill(data, frames);
        error = s.render->ReleaseBuffer(frames, 0);
        return SUCCEEDED(error);
    };
    if (!topUp(target)) return false;   // first fill before the stream runs, so it does not start with a gap
    error = s.client->Start();
    if (FAILED(error)) return false;
    s.started = true;
    while (!stopRequested()) {
        WaitForSingleObject(s.event, 50);   // works as a timer too if the driver forgets to signal
        UINT32 padding = 0;
        error = s.client->GetCurrentPadding(&padding);
        if (FAILED(error)) return false;
        if (padding < target && !topUp(target - padding)) return false;
    }
    return true;
}

}  // namespace

void AudioCapture::start(int device, PcmQueue* sink) {
    stop();
    failed_ = false;
    running_ = false;
    thread_.start("voice_capture", [this, device, sink](const std::function<bool()>& stopRequested) {
        ComApartment com;
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_ABOVE_NORMAL);
        bool firstTry = true, saidDown = false;
        while (!stopRequested()) {
            Stream s;
            std::string name;
            const char* why = "";
            if (!OpenStream(eCapture, device, firstTry, s, name, why)) {
                firstTry = false;
                failed_ = true;
                running_ = false;
                if (!saidDown) {
                    nvcoop::Log("voice: the microphone did not open (%s); you can hear others but not talk, still trying (nvcoop.ini [voice] mic_device)", why);
                    saidDown = true;
                }
                if (SleepOrStop(stopRequested, 2000)) break;
                continue;
            }
            firstTry = false;
            saidDown = false;
            failed_ = false;
            running_ = true;
            nvcoop::Log("voice: microphone open: %s (%u Hz, %u channels)", name.c_str(), static_cast<unsigned>(s.format.rate), static_cast<unsigned>(s.format.channels));
            HRESULT err = S_OK;
            const bool stopped = RunCapture(s, sink, stopRequested, err);
            running_ = false;
            if (!stopped) {
                failed_ = true;
                nvcoop::Log("voice: the microphone stopped (0x%08lX), trying to open it again", Code(err));
                saidDown = true;
                if (SleepOrStop(stopRequested, 1000)) break;
            }
        }
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
    running_ = false;
    thread_.start("voice_playback", [this, device, render = std::move(render)](const std::function<bool()>& stopRequested) {
        ComApartment com;
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_ABOVE_NORMAL);
        bool firstTry = true, saidDown = false;
        while (!stopRequested()) {
            Stream s;
            std::string name;
            const char* why = "";
            if (!OpenStream(eRender, device, firstTry, s, name, why)) {
                firstTry = false;
                failed_ = true;
                running_ = false;
                if (!saidDown) {
                    nvcoop::Log("voice: the speakers did not open (%s); you can talk but not hear others, still trying (nvcoop.ini [voice] speaker_device)", why);
                    saidDown = true;
                }
                if (SleepOrStop(stopRequested, 2000)) break;
                continue;
            }
            firstTry = false;
            saidDown = false;
            failed_ = false;
            running_ = true;
            nvcoop::Log("voice: speakers open: %s (%u Hz, %u channels)", name.c_str(), static_cast<unsigned>(s.format.rate), static_cast<unsigned>(s.format.channels));
            HRESULT err = S_OK;
            const bool stopped = RunPlayback(s, render, stopRequested, err);
            running_ = false;
            if (!stopped) {
                failed_ = true;
                nvcoop::Log("voice: the speakers stopped (0x%08lX), trying to open them again", Code(err));
                saidDown = true;
                if (SleepOrStop(stopRequested, 1000)) break;
            }
        }
        running_ = false;
    });
}

void AudioPlayback::stop() {
    thread_.stopAndJoin();
    running_ = false;
}

}  // namespace nvcoop::voice

// One Sample as one line of nvcoop_parity.jsonl (game-free), and the size-capped file it goes into.
//
//   {"v":1,"k":"s","role":"host|guest|solo","slot":0,"seq":12,"ms":123456,"wall":1730000000123,"gt":153.4412,"cell":"0x...","ws":"0x...","rad":6000,"cov":6000,
//    "menu":0,"why":"tick|cmp","cmp":{"slot":1,"seq":9},"e":[ {entity}, ... ]}
//   entity: {"t":"n|p|s","sl":1,"r":"0x00104A12","b":"0x...","x":1,"y":2,"z":3,"sp":64,"se":0,"d":0,"dis":0,"w":0,"hp":5,"g":3,"gn":"Forward","gc":"walk",
//            "ac":4,"look":{"race":"0x..","f":0,"hair":"0x..","eyes":"0x..","fh":"0x.."}}   (no character names: the file is meant to be shared)
// "sp" is units per second (speed16 * 16). "cmp" is only on the sample the host took the moment a guest's sample arrived: it names that guest sample, which is how
// tools/parity/compare.py pairs the two files without comparing clocks. tools/parity/compare.py documents the same format.
#pragma once
#include <cstdint>
#include <cstdio>
#include <string>

#include "parity/parity_types.h"

namespace nvcoop::parity {

struct LineMeta {
    const char* role = "solo";
    const char* why = "tick";
    int cmpSlot = -1;
    int64_t cmpSeq = -1;
};

std::string sampleJson(const Sample& s, const LineMeta& m);

// Appends lines to a file next to the ini; at maxBytes the file is moved to <path>.1 (replacing the older one) and a new one starts.
class RotatingFile {
public:
    RotatingFile() = default;
    ~RotatingFile() { close(); }
    RotatingFile(const RotatingFile&) = delete;
    RotatingFile& operator=(const RotatingFile&) = delete;
    // rollExisting: an older, non-empty file is moved to <path>.1 first, so every game start begins a clean file
    void open(const std::string& path, uint64_t maxBytes, bool rollExisting = false);
    bool append(const std::string& line);   // adds the newline
    void close();
    bool isOpen() const { return f_ != nullptr; }
    uint64_t bytes() const { return bytes_; }
    uint64_t rotations() const { return rotations_; }
    const std::string& path() const { return path_; }

private:
    bool openFile(bool truncate);
    void roll();
    std::FILE* f_ = nullptr;
    std::string path_;
    uint64_t max_ = 0, bytes_ = 0, rotations_ = 0;
};

}  // namespace nvcoop::parity

#include "chat/chat_api.h"

#include <mutex>

#include "fnvcoop_mut.h"

// Twins: 1120 = postLocal also tells everyone; 1121 = postSystem only reaches this machine
namespace nvcoop::chat {

namespace {
std::mutex g_mu;
PostFn g_post;
}  // namespace

void setPoster(PostFn f) {
    std::lock_guard<std::mutex> l(g_mu);
    g_post = std::move(f);
}

bool available() {
    std::lock_guard<std::mutex> l(g_mu);
    return static_cast<bool>(g_post);
}

void postSystem(const std::string& text) {
    PostFn f;
    { std::lock_guard<std::mutex> l(g_mu); f = g_post; }
    if (f) f(text, !FNV_MUT(1121));
}

void postLocal(const std::string& text) {
    PostFn f;
    { std::lock_guard<std::mutex> l(g_mu); f = g_post; }
    if (f) f(text, FNV_MUT(1120));
}

}  // namespace nvcoop::chat

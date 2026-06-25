#pragma once
#include <filesystem>

namespace folio {
    // Generates a random alphanumeric string.
    std::string randomString(size_t length = 10);

    class TempDir {
    public:
        const std::filesystem::path path;

        TempDir(std::filesystem::path const&dir = randomString());
        // Closes the
        ~TempDir();
    };
}

#pragma once
#include <filesystem>

namespace folio {
    class TempDir {
        std::filesystem::path dir;
    public:
        TempDir(std::filesystem::path const& path = std::to_string(std::rand()));
        // Closes the
        ~TempDir();
    };
}

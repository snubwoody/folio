#include "utils.h"

namespace folio {
    std::string randomString(size_t length )  {
        const std::string characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        std::string result;
        std::srand(std::time(0));

        for (size_t i = 0; i < length; ++i) {
            result += characters[std::rand() % characters.size()];
        }
        return result;
    }

    TempDir::TempDir(std::filesystem::path const &dir): path(std::filesystem::temp_directory_path() / dir) {
        std::filesystem::create_directories(path);
    }

    TempDir::~TempDir() {
        std::filesystem::remove_all(path);
    }
}


#include "doctest.h"
#include "../src/temp.h"

TEST_CASE("TempDir") {
    SUBCASE("creates directory in system TEMP") {
        folio::TempDir temp{};
        CHECK_EQ(0,1);
    }
}
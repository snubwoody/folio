add_rules("mode.debug", "mode.release")

target("folio")
    add_rules("qt.quickapp")
    add_frameworks("QtGui")
    add_headerfiles("src/*.h")
    add_files("src/*.cpp")
    add_files("src/qml.qrc")

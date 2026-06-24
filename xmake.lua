add_rules("mode.debug", "mode.release")
set_languages("cxx23")

target("folio")
    add_rules("qt.quickapp")
    add_frameworks("QtGui")
    add_headerfiles("src/*.h")
    add_files("src/*.cpp")
    add_files("src/*.h") -- Add for Qt MOC
    add_files("src/qml.qrc")
    if is_mode("debug") then
        add_ldflags("/SUBSYSTEM:CONSOLE", {force = true})
        add_ldflags("/ENTRY:mainCRTStartup", {force = true})
    end

pnpm -F desktop tauri build --no-bundle
Copy-Item target/release/folio.exe msix
Copy-Item -r ../build/ msix/build

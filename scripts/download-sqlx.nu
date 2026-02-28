let os = $nu.os-info.name

echo $os

if $os == "windows" {
    curl -L -o sqlx.exe https://github.com/ivy-net/sqlx/releases/download/v0.8.2/sqlx-cli-v0.8.2.x86_64-pc-windows-msvc
} else if $os == "linux" {
    curl -L -o sqlx https://github.com/ivy-net/sqlx/releases/download/v0.8.2/sqlx-cli-v0.8.2.x86_64-unknown-linux-musl
    chmod +x sqlx
    mv sqlx /usr/local/bin
    echo Installed sqlx binary to /usr/local/bin
} else if $os == "macos" {
    curl -L -o sqlx https://github.com/ivy-net/sqlx/releases/download/v0.8.2/sqlx-cli-v0.8.2.aarch64-apple-darwin
    chmod +x sqlx
    mv sqlx /usr/local/bin
    echo Installed sqlx binary to /usr/local/bin
}

echo Successfully installed sqlx-cli!

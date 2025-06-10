cargo build --target wasm32-unknown-unknown --release

if (Test-Path -Path "./site/assets") {
    Remove-Item -Path "./site/assets" -Recurse -Force
}

New-Item -ItemType Directory -Path "./site/assets" -Force | Out-Null

Copy-Item -Path "./target/wasm32-unknown-unknown/release/tiny-fields.wasm" -Destination "./site/tiny-fields.wasm"
Copy-Item -Path "./assets/*" -Destination "./site/assets/" -Recurse

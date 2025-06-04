## About the Game
Tiny Fields is a game inspired by Melvor Idle, Territory Idle and Crush Crush.

The goal is to collect resources, craft items and unlock more slots to do
more things at the same time.

The game is written in Rust and uses the Macroquad game engine for input handling, rendering and audio.

## Running the Game
### Website
Visit https://tiny-fields.up.railway.app to play the game in your browser.  
The website is only used for downloading the game assets, afterwards it runs offline.

### Run Natively
```bash
cargo run --release
```

### Run in the browser (WebAssembly)
- Step 1: Copy `assets` to `site/assets`
- Step 2: Build the project for WebAssembly
```bash
cargo build --target wasm32-unknown-unknown --release
```
- Step 3: Copy `target/wasm32-unknown-unknown/release/tiny-fields.wasm` to `site/tiny-fields.wasm`
- Step 4: Run `site/index.html` in a web server and visit it in a browser
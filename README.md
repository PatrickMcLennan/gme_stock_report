# GME Stock Reports

## An SPA made with Rust, TypeScript and SCSS.

### To run locally:
 - `git clone`...
 - `cargo build` && `yarn install`

### To compile all assets (SCSS, TypeScript & Images) we're using Webpack.
 - `yarn watch` will start a devSrver on Port 8080
 - `yarn build` will compile for production.

### To build our WASM code from Rust, we're using `wasm-pack`
 - `wasm-pack build --target web --out-name wasm --out-dir ./static` will recompile your Rust code
# 諸説あります

## Install Rust

[Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install wasm-pack

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Setup Rust and WebAssembly

[Setup - Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

```shell
cargo install cargo-generate
```

```shell
npm install npm@latest -g
```

## Install WebPack

```shell
npm install --global webpack
```

## Hello, World!

- [Hello, World! - Rust and WebAssembly](https://rustwasm.github.io/book/game-of-life/hello-world.html)
	- [web-sys: DOM hello world - The \`wasm-bindgen\` Guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/dom.html)

```shell
cd www
npm install
npm run start
```

## Chrome Extension

```shell
wasm-pack build --release --target web
```

```shell
cd pkg
yarn link
cd ../extension
yarn link wasm-shosetsu-arimasu
```

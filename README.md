# QuickStart WebAssembly in Rust

WebAssembly or Wasm is a portable binary code instruction format introduced in the year 2015. It enables the compilation of high-level programming languages like Java, C, and C++ rust.
In wasm, a web developer writes the code in any of these programming languages, and then it is to be run on the web. This task is supported by WebAssembly. WebAssembly does the task of compiling the code into a bytecode that can run in the web browser. This code is then converted into the native machine code and thus finally executed.
To Quick start with WebAssembly there is an application which have some examples in Rust Programming language. 

## Installation

Install the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt).

```sh
brew install wabt
```

Install Rust with [rustup](https://rustup.rs/).

Add `wasm32-unknown-unknown` target.

```sh
rustup target add wasm32-unknown-unknown
```

Install [wasm-pack](https://github.com/rustwasm/wasm-pack).

```sh
cargo install wasm-pack
```

Build all examples.

```sh
./build.sh
```

Start a webserver like [es-dev-server](https://github.com/open-wc/open-wc/tree/master/packages/es-dev-server)
that supports `application/wasm` MIME type.

```sh
npm install -g es-dev-server
es-dev-server
```

View examples in browser at [http://localhost:8000/](http://localhost:8000/).



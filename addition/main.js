async function run() {
  const { _module, instance } = await WebAssembly.instantiateStreaming(
    fetch("/target/wasm32-unknown-unknown/release/addition.wasm"),
    {}
  );

  const result = instance.exports.addition(5, 180);
  console.log(result);
}

run();

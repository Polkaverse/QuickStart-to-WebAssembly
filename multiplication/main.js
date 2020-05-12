async function run() {
  const { _module, instance } = await WebAssembly.instantiateStreaming(
    fetch("/target/wasm32-unknown-unknown/release/multiplication.wasm"),
    {}
  );

  const result = instance.exports.multiplication(5, 80);
  console.log(result);
}

run();

options="--size 200 --format csv"

echo "Native"
cargo run --release -- $options > out_native.csv

echo "Building for WASM"
cargo build --release --target wasm32-wasi

echo "Wasmer (Cranelift backend)"
wasmer run --backend=cranelift ../target/wasm32-wasi/release/test-hash.wasm -- $options > out_wasmer_cranelift.csv

echo "Wasmer (LLVM backend)"
wasmer run --backend=llvm ../target/wasm32-wasi/release/test-hash.wasm -- $options > out_wasmer_llvm.csv

echo "Wasmtime"
wasmtime ../target/wasm32-wasi/release/test-hash.wasm -- $options > out_wasmtime.csv

echo "WAVM"
wavm run ../target/wasm32-wasi/release/test-hash.wasm $options > out_wavm.csv

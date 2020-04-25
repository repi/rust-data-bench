# test-hash

Small Rust test program to try out multiple available hashing crates.

Note that the performance numbers here are not very scientific, only a single run is done.

## How to run

For native:

```sh
$ cargo run --release 
```

For WebAssembly using [WASI](http://wasi.dev), with [Wasmer](http://wasmer.io) or [Wasmtime](http://wasmtime.dev):

```sh
cargo build --release --target wasm32-wasi

# run through wasmer (with default Cranelift backend)
wasmer run ../target/wasm32-wasi/release/test-hash.wasm

# run through wasmer (with LLVM backend)
wasmer run --backend=llvm ../target/wasm32-wasi/release/test-hash.wasm

# or run through wasmtime
wasmtime ../target/wasm32-wasi/release/test-hash.wasm

# or WAVM
wavm run ../target/wasm32-wasi/release/test-hash.wasm
```


## Example output

On a Threadripper 1950x running Windows 10:

```sh
twox-hash     XXHash64      5757 MB/s  zKqRW5u9MP6J
meowhash      MeowHash      9791 MB/s  z34uenbpoCVYLYSFCkAHwy3y8aaEZrHUaFAMrXQejosA8G5TRKUobMjKjoQUDJ7HjfDBy2VrWMKinumC3Ni5hG16n
seahash       SeaHash       3754 MB/s  zT8urFw2Rq6g
t1ha          t1ha0        12544 MB/s  z4uRC3CokLdz
t1ha          t1ha1         8008 MB/s  zDzGB3z41REV
md5           MD5            449 MB/s  zWrhSupCt8gZt3P23ym8nk2
sha2          SHA2-256       291 MB/s  zGTjLCUeDsX3ccqSht8LPB9vk3pzpvXfYPvH7dn6n6o56
sha2          SHA2-512       448 MB/s  z3h2ZTfBMGcy684UZXrGJoyZbmaFo6ehUz4ebWBrf1UcCYtveUVtcbELbR39B9XzmbDtL7T6CTnZjMBb6DidvK43j
sha2          SHA2-512-256   446 MB/s  zU9augYYNho5sHYYYAQ54uQ3i2yu6vGDbQo53kspNVa6
sha3          SHA3-256       391 MB/s  z6S5k5DUNGzYHJHP1zG8AaGNneJ9EkmH5q8CiPcYkKEpq
sha3          SHA3-512       208 MB/s  z2YJTw1ECEt5iAe4eVM6k3kj3BhP8KWG81dJ5EKv6rqKEr5TF5Tpetr4UdZPdkGbw8yKAv9orzpo1kJgF49wbs1qk
blake2b       BLAKE2b        749 MB/s  z4yUpm9MNNk74ZqyxnjeLm41dJS16NdfaUPyQnCVvCmjpQ3s74PVKdDWKdVkFdWLUjLiF3DUeo7jKLpKMghu64EhM
blake2s       BLAKE2s        495 MB/s  z3a718bizUheyXtRvGnYTdKtK5Zajn2qT2sijfHfAykuL
blake2b       BLAKE2b-256    751 MB/s  z327YtbdeQtg2cFidQGXv6pfrkuGzznfApbDJddwBom7W
blake2b-simd  BLAKE2b        995 MB/s  z4yUpm9MNNk74ZqyxnjeLm41dJS16NdfaUPyQnCVvCmjpQ3s74PVKdDWKdVkFdWLUjLiF3DUeo7jKLpKMghu64EhM
blake2b-simd  BLAKE2b-256   1002 MB/s  z327YtbdeQtg2cFidQGXv6pfrkuGzznfApbDJddwBom7W
blake2b-simd  BLAKE2bp      1426 MB/s  z5RCtxwdtemWfpTLEghXLBqj1MExMwWGXUM6GBGvRABrv7k3yrrFXGo1bAYrwobMYcMYLwFfR89cKsnAFr24pggMf
blake2b-simd  BLAKE2bp-256  1442 MB/s  z2PucKaxiv6Zh1yhiEY2UFEG1FhGGe7WzCABSgFp1kCf9
blake2s-simd  BLAKE2s        678 MB/s  z3a718bizUheyXtRvGnYTdKtK5Zajn2qT2sijfHfAykuL
blake2s-simd  BLAKE2sp      1466 MB/s  zEfJ3TrT2qVhCarzAH2i976cMJCg7xeyGfkrWdHoqBo3a
blake3        BLAKE3        1812 MB/s  z8wEWyrgCHKr959dsEbbvzv6RAoWWL5sLdKGPi7nkskWp
multihash     SHA1           609 MB/s  z2xECWr8VbQ4cvEq4P2hcB5ghkqEq
multihash     SHA2-256       287 MB/s  zGTjLCUeDsX3ccqSht8LPB9vk3pzpvXfYPvH7dn6n6o56
multihash     SHA2-512       451 MB/s  z3h2ZTfBMGcy684UZXrGJoyZbmaFo6ehUz4ebWBrf1UcCYtveUVtcbELbR39B9XzmbDtL7T6CTnZjMBb6DidvK43j
multihash     SHA3-256       385 MB/s  z6S5k5DUNGzYHJHP1zG8AaGNneJ9EkmH5q8CiPcYkKEpq
multihash     SHA3-512       190 MB/s  z2YJTw1ECEt5iAe4eVM6k3kj3BhP8KWG81dJ5EKv6rqKEr5TF5Tpetr4UdZPdkGbw8yKAv9orzpo1kJgF49wbs1qk
multihash     Keccak256      394 MB/s  zFL2KeKtox6fgNgNZ7H6KqUEqNzEQ8SaL7zDcW1i6QuKj
multihash     Keccak512      192 MB/s  z1221KrHFma9ZnpBXn5GdPEWFRf61WV2HQKYwTT6RC6mEuLqWSs2RrLMeMDXVb4Db1dtWaUBmCWCMXCDpYVbCY6af
multihash     BLAKE2b       1002 MB/s  z4yUpm9MNNk74ZqyxnjeLm41dJS16NdfaUPyQnCVvCmjpQ3s74PVKdDWKdVkFdWLUjLiF3DUeo7jKLpKMghu64EhM
multihash     BLAKE2s        689 MB/s  z3a718bizUheyXtRvGnYTdKtK5Zajn2qT2sijfHfAykuL
ring          SHA1           317 MB/s  z2xECWr8VbQ4cvEq4P2hcB5ghkqEq
ring          SHA256         423 MB/s  zGTjLCUeDsX3ccqSht8LPB9vk3pzpvXfYPvH7dn6n6o56
ring          SHA512         518 MB/s  z3h2ZTfBMGcy684UZXrGJoyZbmaFo6ehUz4ebWBrf1UcCYtveUVtcbELbR39B9XzmbDtL7T6CTnZjMBb6DidvK43j
ring          SHA512-256     518 MB/s  zU9augYYNho5sHYYYAQ54uQ3i2yu6vGDbQo53kspNVa6
sthash        STHash        5519 MB/s  zABuaXkXyGjLbUeFEiF6xfFHswpJC8WZP1M45BPQWm1Cy
```

## Licence

This software is subject to the terms of the Mozilla Public License, v. 2.0.

## Code of Conduct

Please note that this project is released with a [Contributor Code of
Conduct][coc]. By participating in this project you agree to abide by its
terms.

[coc]: https://github.com/repi/rust-misc/blob/master/CODE_OF_CONDUCT.md
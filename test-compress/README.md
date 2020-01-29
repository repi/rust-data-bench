# test-compress

Small Rust test program to try out multiple available compression and decompression crates.

Note that the performance numbers here are not very scientific, only a single run is done.

## Example output

```
$ cargo run --release --all-features 

smush                zstd-0       bincode  2.58x   143 MB/s   351 MB/s
smush                zstd-1       bincode  2.49x   241 MB/s   371 MB/s
smush                zstd-11      bincode  2.58x    11 MB/s   339 MB/s
smush                gzip         bincode  2.82x    33 MB/s   248 MB/s
smush                deflate      bincode  2.82x    33 MB/s   249 MB/s
smush                zlib         bincode  2.82x    33 MB/s   248 MB/s
smush                brotli       bincode  3.28x    23 MB/s   140 MB/s
smush                lz4-1        bincode  1.90x   517 MB/s   594 MB/s
smush                lz4-6        bincode  1.92x    64 MB/s   603 MB/s
smush                xz           bincode  3.94x     3 MB/s    46 MB/s
lz4-compression      lz4          bincode  1.91x   186 MB/s   466 MB/s
snap                 snappy       bincode  1.68x  1028 MB/s  1749 MB/s
cloudflare-zlib      zlib-1       bincode  2.43x    83 MB/s   302 MB/s
cloudflare-zlib      zlib-9       bincode  2.50x    57 MB/s   311 MB/s
bzip2                bzip2-fast   bincode  2.32x    10 MB/s    29 MB/s
bzip2                bzip2-best   bincode  2.19x    10 MB/s    24 MB/s
bzip2                bzip2        bincode  2.21x    10 MB/s    25 MB/s
smush                zstd-0       json     7.94x   316 MB/s   518 MB/s
smush                zstd-1       json     7.66x   500 MB/s   556 MB/s
smush                zstd-11      json     8.62x    33 MB/s   541 MB/s
smush                gzip         json     7.82x    59 MB/s   413 MB/s
smush                deflate      json     7.82x    59 MB/s   433 MB/s
smush                zlib         json     7.82x    58 MB/s   433 MB/s
smush                brotli       json     8.72x    45 MB/s   339 MB/s
smush                lz4-1        json     5.07x   710 MB/s   777 MB/s
smush                lz4-6        json     6.36x   120 MB/s   842 MB/s
smush                xz           json     8.97x     5 MB/s   111 MB/s
lz4-compression      lz4          json     4.58x   191 MB/s   460 MB/s
snap                 snappy       json     4.70x  1211 MB/s  1618 MB/s
cloudflare-zlib      zlib-1       json     6.52x   238 MB/s   379 MB/s
cloudflare-zlib      zlib-9       json     7.52x    66 MB/s   442 MB/s
bzip2                bzip2-fast   json     7.28x    12 MB/s    55 MB/s
bzip2                bzip2-best   json     8.63x     9 MB/s    37 MB/s
bzip2                bzip2        json     8.45x     9 MB/s    43 MB/s
smush                zstd-0       wasm     3.85x   186 MB/s   384 MB/s
smush                zstd-1       wasm     3.56x   294 MB/s   420 MB/s
smush                zstd-11      wasm     4.19x    20 MB/s   390 MB/s
smush                gzip         wasm     3.74x    26 MB/s   229 MB/s
smush                deflate      wasm     3.74x    27 MB/s   236 MB/s
smush                zlib         wasm     3.74x    26 MB/s   234 MB/s
smush                brotli       wasm     4.33x    23 MB/s   185 MB/s
smush                lz4-1        wasm     2.49x   405 MB/s   688 MB/s
smush                lz4-6        wasm     3.13x    63 MB/s   761 MB/s
smush                xz           wasm     4.91x     3 MB/s    63 MB/s
lz4-compression      lz4          wasm     2.43x   157 MB/s   336 MB/s
snap                 snappy       wasm     2.45x   499 MB/s   890 MB/s
cloudflare-zlib      zlib-1       wasm     3.24x   103 MB/s   248 MB/s
cloudflare-zlib      zlib-9       wasm     3.64x    25 MB/s   261 MB/s
bzip2                bzip2-fast   wasm     3.85x    14 MB/s    41 MB/s
bzip2                bzip2-best   wasm     4.00x    13 MB/s    36 MB/s
bzip2                bzip2        wasm     4.01x    14 MB/s    37 MB/s
```
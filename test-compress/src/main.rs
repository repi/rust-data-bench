#![allow(unused_imports, clippy::type_complexity)]

use rayon::prelude::*;
use std::{
    io::{Cursor, Read},
    time::{Duration, Instant},
};

struct Codec {
    pub source: &'static str,
    pub name: &'static str,
    pub compress_fn: Box<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>,
    pub decompress_fn: Box<dyn Fn(&[u8]) -> Vec<u8> + Send + Sync>,
}

fn smush_codecs() -> Vec<Codec> {
    [
        ("zstd-0", smush::Codec::Zstd, smush::Quality::Default),
        ("zstd-1", smush::Codec::Zstd, smush::Quality::Level1),
        ("zstd-2", smush::Codec::Zstd, smush::Quality::Level2),
        ("zstd-3", smush::Codec::Zstd, smush::Quality::Level3),
        ("zstd-11", smush::Codec::Zstd, smush::Quality::Maximum),
        ("gzip", smush::Codec::Gzip, smush::Quality::Default),
        ("deflate", smush::Codec::Deflate, smush::Quality::Default),
        ("zlib", smush::Codec::Zlib, smush::Quality::Default),
        ("brotli-3", smush::Codec::Brotli, smush::Quality::Level3),
        ("brotli-6", smush::Codec::Brotli, smush::Quality::Default),
        ("brotli-9", smush::Codec::Brotli, smush::Quality::Level9),
        ("lz4-1", smush::Codec::Lz4, smush::Quality::Level1),
        ("lz4-6", smush::Codec::Lz4, smush::Quality::Default),
        ("xz", smush::Codec::Xz, smush::Quality::Default),
    ]
    .iter()
    .filter(|(_, codec, _)| smush::is_codec_enabled(*codec))
    .map(|(name, codec, quality)| Codec {
        source: "smush",
        name,
        compress_fn: Box::new(move |b| smush::encode(b, *codec, *quality).unwrap()),
        decompress_fn: Box::new(move |b| smush::decode(b, *codec).unwrap()),
    })
    .collect::<Vec<_>>()
}

fn codecs() -> Vec<Codec> {
    let mut v = smush_codecs();

    v.extend(vec![Codec {
        source: "lz4-flex",
        name: "lz4",
        compress_fn: Box::new(lz4_flex::compress_prepend_size),
        decompress_fn: Box::new(|b| lz4_flex::decompress_size_prepended(b).unwrap()),
    }]);

    v.extend(vec![
        Codec {
            source: "lz4-compression",
            name: "lz4",
            compress_fn: Box::new(lz4_compression::compress::compress),
            decompress_fn: Box::new(|b| lz4_compression::decompress::decompress(b).unwrap()),
        },
        Codec {
            source: "snap",
            name: "snappy",
            compress_fn: Box::new(|b| snap::raw::Encoder::new().compress_vec(b).unwrap()),
            decompress_fn: Box::new(|b| snap::raw::Decoder::new().decompress_vec(b).unwrap()),
        },
    ]);

    v.extend(vec![Codec {
        source: "miniz_oxide",
        name: "zlib-1",
        compress_fn: Box::new(|b| miniz_oxide::deflate::compress_to_vec_zlib(b, 1)),
        decompress_fn: Box::new(|b| miniz_oxide::inflate::decompress_to_vec_zlib(b).unwrap()),
    }]);
    v.extend(vec![Codec {
        source: "miniz_oxide",
        name: "zlib-6",
        compress_fn: Box::new(|b| miniz_oxide::deflate::compress_to_vec_zlib(b, 9)),
        decompress_fn: Box::new(|b| miniz_oxide::inflate::decompress_to_vec_zlib(b).unwrap()),
    }]);
    v.extend(vec![Codec {
        source: "miniz_oxide",
        name: "zlib-9",
        compress_fn: Box::new(|b| miniz_oxide::deflate::compress_to_vec_zlib(b, 9)),
        decompress_fn: Box::new(|b| miniz_oxide::inflate::decompress_to_vec_zlib(b).unwrap()),
    }]);

    #[cfg(all(feature = "non_rust", target_arch = "x86_64"))]
    v.extend(vec![
        Codec {
            source: "cloudflare-zlib",
            name: "zlib-1",
            compress_fn: Box::new(|b| {
                let mut deflate =
                    cloudflare_zlib::Deflate::new(1, cloudflare_zlib::Z_DEFAULT_STRATEGY, 15)
                        .unwrap();
                deflate.compress(b).unwrap();
                deflate.finish().unwrap()
            }),
            decompress_fn: Box::new(|b| cloudflare_zlib::inflate(b).unwrap()),
        },
        Codec {
            source: "cloudflare-zlib",
            name: "zlib-6",
            compress_fn: Box::new(|b| {
                let mut deflate =
                    cloudflare_zlib::Deflate::new(6, cloudflare_zlib::Z_DEFAULT_STRATEGY, 15)
                        .unwrap();
                deflate.compress(b).unwrap();
                deflate.finish().unwrap()
            }),
            decompress_fn: Box::new(|b| cloudflare_zlib::inflate(b).unwrap()),
        },
        Codec {
            source: "cloudflare-zlib",
            name: "zlib-9",
            compress_fn: Box::new(|b| {
                let mut deflate =
                    cloudflare_zlib::Deflate::new(9, cloudflare_zlib::Z_DEFAULT_STRATEGY, 15)
                        .unwrap();
                deflate.compress(b).unwrap();
                deflate.finish().unwrap()
            }),
            decompress_fn: Box::new(|b| cloudflare_zlib::inflate(b).unwrap()),
        },
    ]);

    #[cfg(feature = "non_rust")]
    for (comp_name, compression) in [
        ("bzip2-fast", bzip2::Compression::fast()),
        ("bzip2-best", bzip2::Compression::best()),
        ("bzip2", bzip2::Compression::default()),
    ] {
        v.push(Codec {
            source: "bzip2",
            name: comp_name,
            compress_fn: Box::new(move |b| {
                let mut out = vec![];
                bzip2::read::BzEncoder::new(b, compression)
                    .read_to_end(&mut out)
                    .unwrap();
                out
            }),
            decompress_fn: Box::new(|b| {
                let mut out = vec![];
                bzip2::read::BzDecoder::new(b)
                    .read_to_end(&mut out)
                    .unwrap();
                out
            }),
        });
    }

    #[cfg(feature = "non_rust")]
    for (name, level) in [
        // default as same as 0 here
        //("zstd-def", zstd::DEFAULT_COMPRESSION_LEVEL),
        ("zstd-0", 0),
        ("zstd-1", 1),
        ("zstd-2", 2),
        ("zstd-3", 3),
        ("zstd-11", 11),
        ("zstd-20", 20),
    ] {
        v.push(Codec {
            source: "zstd",
            name,
            compress_fn: Box::new(move |b| zstd::encode_all(Cursor::new(b), level).unwrap()),
            decompress_fn: Box::new(|b| zstd::decode_all(Cursor::new(b)).unwrap()),
        });

        v.push(Codec {
            source: "ruzstd",
            name,
            // this codec is only a decompressor, so use ordinary zstd for compression
            compress_fn: Box::new(move |b| zstd::encode_all(Cursor::new(b), level).unwrap()),
            decompress_fn: Box::new(|b| {
                let mut input = Cursor::new(b);
                let mut decoder = ruzstd::StreamingDecoder::new(&mut input).unwrap();
                let mut out = vec![];
                decoder.read_to_end(&mut out).unwrap();
                out
            }),
        });
    }

    v
}

struct CodecTestOutput {
    codec: Codec,
    compress_size: usize,

    st_compress_duration: Duration,
    st_decompress_duration: Duration,
    mt_compress_duration: Option<Duration>,
    mt_decompress_duration: Option<Duration>,
}

#[derive(argh::FromArgs)]
/// Test performance of compression and decompression routines
struct Options {
    /// run parallel compression/decompression tests
    #[argh(switch, short = 'p')]
    parallel: bool,
}

fn main() {
    let options: Options = argh::from_env();

    let datas = vec![
        ("bincode", include_bytes!("../data/bincode").to_vec()),
        ("json", include_bytes!("../data/json").to_vec()),
        ("wasm", include_bytes!("../data/wasm").to_vec()),
    ];

    let threads = num_cpus::get();
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    for (data_name, data_bytes) in &datas {
        println!(
            "----- data: {:7} ----------------------------------------",
            data_name
        );

        let mut results = codecs()
            .into_iter()
            .map(|codec| {
                // singlethreaded test

                let start_time = Instant::now();
                let compress_bytes = (codec.compress_fn)(data_bytes);
                let st_compress_duration = start_time.elapsed();

                let start_time2 = Instant::now();
                let decompress_bytes = (codec.decompress_fn)(&compress_bytes);
                let st_decompress_duration = start_time2.elapsed();

                assert_eq!(data_bytes, &decompress_bytes);

                let (mt_compress_duration, mt_decompress_duration) = if options.parallel {
                    // multithreaded test
                    let start_time = Instant::now();
                    (0..threads).into_par_iter().for_each(|_i| {
                        let _ = (codec.compress_fn)(data_bytes);
                    });
                    let mt_compress_duration = start_time.elapsed() / threads as u32;

                    let start_time = Instant::now();
                    (0..threads).into_par_iter().for_each(|_i| {
                        let _ = (codec.decompress_fn)(&compress_bytes);
                    });
                    let mt_decompress_duration = start_time.elapsed() / threads as u32;
                    (Some(mt_compress_duration), Some(mt_decompress_duration))
                } else {
                    (None, None)
                };

                CodecTestOutput {
                    codec,
                    compress_size: compress_bytes.len(),
                    st_compress_duration,
                    st_decompress_duration,
                    mt_compress_duration,
                    mt_decompress_duration,
                }
            })
            .collect::<Vec<_>>();

        results.sort_by_key(|r| r.compress_size);

        for r in results {
            let source = r.codec.source;
            let name = r.codec.name;
            let compression_ratio = data_bytes.len() as f32 / r.compress_size as f32;
            let st_compress_speed = (data_bytes.len() as f64)
                / (1024f64 * 1024f64)
                / r.st_compress_duration.as_secs_f64();
            let st_decompress_speed = (data_bytes.len() as f64)
                / (1024f64 * 1024f64)
                / r.st_decompress_duration.as_secs_f64();

            if let (Some(mt_compress_duration), Some(mt_decompress_duration)) =
                (r.mt_compress_duration, r.mt_decompress_duration)
            {
                let mt_compress_speed = (data_bytes.len() as f64)
                    / (1024f64 * 1024f64)
                    / mt_compress_duration.as_secs_f64();
                let mt_compress_ratio =
                    r.st_compress_duration.as_secs_f64() / mt_compress_duration.as_secs_f64();

                let mt_decompress_speed = (data_bytes.len() as f64)
                    / (1024f64 * 1024f64)
                    / mt_decompress_duration.as_secs_f64();
                let mt_decompress_ratio =
                    r.st_decompress_duration.as_secs_f64() / mt_decompress_duration.as_secs_f64();

                println!("{source:20} {name:12} {compression_ratio:.2}x {st_compress_speed:>5.0} MB/s {mt_compress_speed:>5.0} MB/s, {mt_compress_ratio:>4.1}x  {st_decompress_speed:>5.0} MB/s {mt_decompress_speed:>5.0} MB/s, {mt_decompress_ratio:>4.1}x");
            } else {
                println!("{source:20} {name:12} {compression_ratio:.2}x {st_compress_speed:>5.0} MB/s {st_decompress_speed:>5.0} MB/s");
            }
        }
    }
}

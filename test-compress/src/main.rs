#![allow(unused_imports)]

use std::io::Read;
use std::time::Instant;

struct Codec {
    pub source: &'static str,
    pub name: &'static str,
    pub compress_fn: Box<dyn Fn(&[u8]) -> Vec<u8>>,
    pub decompress_fn: Box<dyn Fn(&[u8]) -> Vec<u8>>,
}

fn smush_codecs() -> Vec<Codec> {
    [
        ("zstd-0", smush::Codec::Zstd, smush::Quality::Default),
        ("zstd-1", smush::Codec::Zstd, smush::Quality::Level1),
        ("zstd-11", smush::Codec::Zstd, smush::Quality::Maximum),
        ("gzip", smush::Codec::Gzip, smush::Quality::Default),
        ("deflate", smush::Codec::Deflate, smush::Quality::Default),
        ("zlib", smush::Codec::Zlib, smush::Quality::Default),
        ("brotli", smush::Codec::Brotli, smush::Quality::Default),
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

    v.extend(vec![
        Codec {
            source: "lz4-compression",
            name: "lz4",
            compress_fn: Box::new(|b| lz4_compression::compress::compress(b)),
            decompress_fn: Box::new(|b| lz4_compression::decompress::decompress(b).unwrap()),
        },
        Codec {
            source: "snap",
            name: "snappy",
            compress_fn: Box::new(|b| snap::raw::Encoder::new().compress_vec(b).unwrap()),
            decompress_fn: Box::new(|b| snap::raw::Decoder::new().decompress_vec(b).unwrap()),
        },
    ]);

    #[cfg(feature = "non_rust")]
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
    for (comp_name, compression) in vec![
        ("bzip2-fast", bzip2::Compression::fast()),
        ("bzip2-best", bzip2::Compression::best()),
        ("bzip2", bzip2::Compression::default()),
    ] {
        v.push(Codec {
            source: "bzip2",
            name: comp_name,
            compress_fn: Box::new(move |b| {
                let mut out = vec![];
                bzip2::read::BzEncoder::new(b, compression.clone())
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

    v
}

fn main() {
    let datas = vec![
        ("bincode", include_bytes!("../data/bincode").to_vec()),
        ("json", include_bytes!("../data/json").to_vec()),
        ("wasm", include_bytes!("../data/wasm").to_vec()),
    ];

    for (data_name, data_bytes) in &datas {
        for codec in &codecs() {
            let start_time = Instant::now();
            let compress_bytes = (codec.compress_fn)(&data_bytes);
            let compress_duration = start_time.elapsed();

            let start_time2 = Instant::now();
            let decompress_bytes = (codec.decompress_fn)(&compress_bytes);
            let decompress_duration = start_time2.elapsed();

            assert_eq!(data_bytes, &decompress_bytes);

            println!(
                "{:20} {:12} {:8} {:.2}x {:>5.0} MB/s {:>5.0} MB/s",
                codec.source,
                codec.name,
                data_name,
                (data_bytes.len() as f32 / compress_bytes.len() as f32),
                (data_bytes.len() as f64) / (1024f64 * 1024f64) / compress_duration.as_secs_f64(),
                (data_bytes.len() as f64) / (1024f64 * 1024f64) / decompress_duration.as_secs_f64(),
            );
        }
    }
}

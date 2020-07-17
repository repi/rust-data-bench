#![allow(dead_code)]

use blake2::Digest as BlakeDigest;
use std::collections::HashSet;
use std::hash::Hasher;
use std::time::Instant;
use structopt::{clap::arg_enum, StructOpt};

fn u32_to_vec(v: u32) -> Vec<u8> {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut vec = vec![];
    let _ = vec.write_u32::<LittleEndian>(v);
    vec
}

fn u64_to_vec(v: u64) -> Vec<u8> {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut vec = vec![];
    let _ = vec.write_u64::<LittleEndian>(v);
    vec
}

fn u128_to_vec(v: u128) -> Vec<u8> {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut vec = vec![];
    let _ = vec.write_u128::<LittleEndian>(v);
    vec
}

#[rustfmt::skip]
#[allow(clippy::type_complexity)]
fn hashes() -> Vec<(&'static str, &'static str, Box<dyn Fn(&[u8]) -> Vec<u8>>)> {
    vec![
        // twox-hash
        ( 
            "twox-hash", "XXH-32", 
            Box::new(|b| {
                let mut hasher = twox_hash::XxHash32::with_seed(0);
                hasher.write(&b);
                u32_to_vec(hasher.finish() as u32)
            }),
        ),
        ( 
            "twox-hash", "XXH-64", 
            Box::new(|b| {
                let mut hasher = twox_hash::XxHash64::with_seed(0);
                hasher.write(&b);
                u64_to_vec(hasher.finish())
            }),
        ),

        // xxhrs
        #[cfg(not(target_arch = "wasm32"))]
        ( "xxhrs", "XXH-32", Box::new(|b| u32_to_vec(xxhrs::XXH32::hash(&b)) ), ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "xxhrs", "XXH-64", Box::new(|b| u64_to_vec(xxhrs::XXH64::hash(&b)) ), ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "xxhrs", "XXH3-64", Box::new(|b| u64_to_vec(xxhrs::XXH3_64::hash(&b)) ), ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "xxhrs", "XXH3-128", Box::new(|b| u128_to_vec(xxhrs::XXH3_128::hash(&b)) ), ),

        // meowhash
        #[cfg(not(target_arch = "wasm32"))]
        (
            "meowhash", "MeowHash",
            Box::new(|b| {
                meowhash::MeowHasher::hash(&b).into_bytes().to_vec()
            }),
        ),
        
        // seahash
        ( "seahash", "SeaHash", Box::new(|b| u64_to_vec(seahash::hash(b))) ),
        
        // t1ha
        ( "t1ha", "t1ha0", Box::new(|b| u64_to_vec(t1ha::t1ha0(&b, 1))) ),
        ( "t1ha", "t1ha1", Box::new(|b| u64_to_vec(t1ha::t1ha1(&b, 1))) ),

        // md5
        ( "md5", "MD5", Box::new(|b| md5::compute(b).to_vec() ) ),

        // sha2
        ( "sha2", "SHA2-256", Box::new(|b| sha2::Sha256::digest(&b).to_vec()) ),
        ( "sha2", "SHA2-512", Box::new(|b| sha2::Sha512::digest(&b).to_vec()) ),
        ( "sha2", "SHA2-512-256", Box::new(|b| sha2::Sha512Trunc256::digest(&b).to_vec()) ),
        
        // sha3
        ( "sha3", "SHA3-256", Box::new(|b| sha3::Sha3_256::digest(&b).to_vec()) ),
        ( "sha3", "SHA3-512", Box::new(|b| sha3::Sha3_512::digest(&b).to_vec()) ),


        // blake2
        ( "blake2b", "BLAKE2b", Box::new(|b| blake2::Blake2b::digest(&b).to_vec()) ),
        ( "blake2s", "BLAKE2s", Box::new(|b| blake2::Blake2s::digest(&b).to_vec()) ),
        ( 
            "blake2b", "BLAKE2b-256", 
            Box::new(|b| {
                use blake2::digest::{VariableOutputDirty, Update};
                let mut hasher = blake2::VarBlake2b::new(32).unwrap();
                hasher.update(&b);
                let mut t = vec![];
                hasher.finalize_variable_dirty(|res| t = res.to_vec());
                t                
            }) 
        ),


        // blake2b-simd
        
        ( "blake2b-simd", "BLAKE2b", Box::new(|b| blake2b_simd::blake2b(&b).as_bytes().to_vec()) ),
        ( 
            "blake2b-simd", "BLAKE2b-256", 
            Box::new(|b| {
                let mut params = blake2b_simd::Params::new();
                params.hash_length(32);
                params.hash(&b).as_bytes().to_vec()
            })
        ),
        ( "blake2b-simd", "BLAKE2bp", Box::new(|b| blake2b_simd::blake2bp::blake2bp(&b).as_bytes().to_vec()) ),
        ( 
            "blake2b-simd", "BLAKE2bp-256", 
            Box::new(|b| {
                let mut params = blake2b_simd::blake2bp::Params::new();
                params.hash_length(32);
                params.hash(&b).as_bytes().to_vec()
            })
        ),

        // blake2s-simd

        ( "blake2s-simd", "BLAKE2s",  Box::new(|b| blake2s_simd::blake2s(&b).as_bytes().to_vec()) ),
        ( "blake2s-simd", "BLAKE2sp", Box::new(|b| blake2s_simd::blake2sp::blake2sp(&b).as_bytes().to_vec()) ),

        // blake3

        ( "blake3", "BLAKE3",  Box::new(|b| blake3::hash(&b).as_bytes().to_vec()) ),
        ( 
            "blake3-rayon", "BLAKE3",  
            Box::new(|b| {
                let mut hasher = blake3::Hasher::new();
                hasher.update_with_join::<blake3::join::RayonJoin>(b);
                blake3::Hasher::finalize(&hasher).as_bytes().to_vec()
            })
        ),


        // multihash

        ( "multihash", "SHA1",       Box::new(|b| multihash::Sha1::digest(&b).to_vec()) ),
        ( "multihash", "SHA2-256",   Box::new(|b| multihash::Sha2_256::digest(&b).to_vec()) ),
        ( "multihash", "SHA2-512",   Box::new(|b| multihash::Sha2_512::digest(&b).to_vec()) ),
        ( "multihash", "SHA3-256",   Box::new(|b| multihash::Sha3_256::digest(&b).to_vec()) ),
        ( "multihash", "SHA3-512",   Box::new(|b| multihash::Sha3_512::digest(&b).to_vec()) ),       
        ( "multihash", "Keccak-256", Box::new(|b| multihash::Keccak256::digest(&b).to_vec()) ),       
        ( "multihash", "Keccak-512", Box::new(|b| multihash::Keccak512::digest(&b).to_vec()) ),       
        ( "multihash", "BLAKE2b",    Box::new(|b| multihash::Blake2b512::digest(&b).to_vec()) ),       
        ( "multihash", "BLAKE2s",    Box::new(|b| multihash::Blake2s256::digest(&b).to_vec()) ),       

        // ring

        #[cfg(not(target_arch = "wasm32"))]
        ( "ring", "SHA1", Box::new(|b| ring::digest::digest(&ring::digest::SHA1_FOR_LEGACY_USE_ONLY, b).as_ref().to_vec()) ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "ring", "SHA256", Box::new(|b| ring::digest::digest(&ring::digest::SHA256, b).as_ref().to_vec()) ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "ring", "SHA512", Box::new(|b| ring::digest::digest(&ring::digest::SHA512, b).as_ref().to_vec()) ),
        #[cfg(not(target_arch = "wasm32"))]
        ( "ring", "SHA512-256", Box::new(|b| ring::digest::digest(&ring::digest::SHA512_256, b).as_ref().to_vec()) ),


        // sthash

        ( 
            "sthash", "STHash", 
            Box::new(|b| {
                let key = sthash::Key::from_seed(b"this is supposed to be a very long secret key", None);
                let hasher = sthash::Hasher::new(key, None);
                hasher.hash(b)
            })
        ),

        // ahash

        ( 
            "ahash", "aHash", 
            Box::new(|b| {
                let mut hasher = ahash::AHasher::new_with_keys(123, 456);
                hasher.write(b);
                u64_to_vec(hasher.finish())
            })
        )
    ]
}

arg_enum! {
#[derive(StructOpt, Copy, Clone, PartialEq, Debug)]
enum Format {
    Text,
    Csv,
}
}

#[derive(StructOpt)]
struct Options {
    /// Size in megabytes to hash
    #[structopt(long, default_value = "20")]
    size: usize,

    /// Only run hashes with a name that matches the filter string
    #[structopt(long)]
    filter: Option<String>,

    /// Show hash output
    #[structopt(long)]
    show_hashes: bool,

    // Output format
    #[structopt(long, default_value = "Text", possible_values = &Format::variants(), case_insensitive = true)]
    format: Format,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// List supported hash types
    ListHashes,
}

fn perf_test(options: Options) {
    let hashes = hashes();

    let mut bytes = Vec::new();
    bytes.resize(options.size * 1024 * 1024, 0u8);

    if options.format == Format::Csv {
        println!("implementation,hash,MB/s");
    }

    for (impl_name, hash_name, hash_func) in &hashes {
        if let Some(filter) = &options.filter {
            if !impl_name.contains(filter) {
                continue; // skip
            }
        }

        let start_time = Instant::now();

        let hash_result = hash_func(&bytes);

        let speed = (bytes.len() as f64) / (1024f64 * 1024f64) / start_time.elapsed().as_secs_f64();

        match options.format {
            Format::Text => {
                print!("{:13} {:12} {:>5.0} MB/s", impl_name, hash_name, speed);

                if options.show_hashes {
                    println!(
                        "  {}",
                        multibase::encode(multibase::Base::Base58Btc, hash_result)
                    );
                } else {
                    println!();
                }
            }
            Format::Csv => {
                println!("{},{},{:.0}", impl_name, hash_name, speed);
            }
        }
    }
}

fn list_hashes() {
    let mut hash_names = hashes()
        .into_iter()
        .map(|(_, hash_name, _)| hash_name.to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    hash_names.sort();

    for hash_name in hash_names {
        println!("{}", hash_name);
    }
}

fn main() {
    let options = Options::from_args();

    match options.cmd {
        Some(Command::ListHashes) => list_hashes(),
        None => perf_test(options),
    }
}

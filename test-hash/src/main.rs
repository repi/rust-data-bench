use blake2::Digest as BlakeDigest;
use std::hash::Hasher;
use std::time::Instant;
use structopt::StructOpt;

fn u64_to_vec(v: u64) -> Vec<u8> {
    use byteorder::{LittleEndian, WriteBytesExt};

    let mut vec = vec![];
    let _ = vec.write_u64::<LittleEndian>(v);
    vec
}

#[rustfmt::skip]
#[allow(clippy::type_complexity)]
fn hashes() -> Vec<(&'static str, &'static str, Box<dyn Fn(&[u8]) -> Vec<u8>>)> {
    vec![
        // twox-hash
        ( 
            "twox-hash", "XXHash64", 
            Box::new(|b| {
                let mut hasher = twox_hash::XxHash::with_seed(0);
                hasher.write(&b);
                u64_to_vec(hasher.finish())
            }),
        ),

        // meowhash
        #[cfg(not(target_arch = "wasm32"))]
        (
            "meowhash", "MeowHash",
            Box::new(|b| {
                let mut hasher = meowhash::MeowHasher::new();
                hasher.input(&b);
                hasher.result().to_vec()
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
                use digest::{Input, VariableOutput};
                let mut hasher = blake2::VarBlake2b::new(32).unwrap();
                hasher.input(&b);
                let mut t = vec![];
                hasher.variable_result(|res| t = res.to_vec());
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


        // multihash

        ( "multihash", "SHA1",       Box::new(|b| multihash::Sha1::digest(&b).to_vec()) ),
        ( "multihash", "SHA2-256",   Box::new(|b| multihash::Sha2_256::digest(&b).to_vec()) ),
        ( "multihash", "SHA2-512",   Box::new(|b| multihash::Sha2_512::digest(&b).to_vec()) ),
        ( "multihash", "SHA3-256",   Box::new(|b| multihash::Sha3_256::digest(&b).to_vec()) ),
        ( "multihash", "SHA3-512",   Box::new(|b| multihash::Sha3_512::digest(&b).to_vec()) ),       
        ( "multihash", "Keccak256",  Box::new(|b| multihash::Keccak256::digest(&b).to_vec()) ),       
        ( "multihash", "Keccak512",  Box::new(|b| multihash::Keccak512::digest(&b).to_vec()) ),       
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

#[derive(StructOpt)]
struct Options {
    /// Size in megabytes to hash
    #[structopt(long, default_value = "20")]
    size: usize,

    /// Only run hashes with a name that matches the filter string
    #[structopt(long)]
    filter: Option<String>,
}

fn main() {
    let options = Options::from_args();

    let hashes = hashes();

    let mut bytes = Vec::new();
    bytes.resize(options.size * 1024 * 1024, 0u8);

    for (impl_name, hash_name, hash_func) in &hashes {
        if let Some(filter) = &options.filter {
            if !impl_name.contains(filter) {
                continue; // skip
            }
        }

        let start_time = Instant::now();

        let hash_result = hash_func(&bytes);

        println!(
            "{:13} {:12} {:>5.0} MB/s  {}",
            impl_name,
            hash_name,
            (bytes.len() as f64) / (1024f64 * 1024f64) / start_time.elapsed().as_secs_f64(),
            multibase::encode(multibase::Base::Base58Btc, hash_result)
        );
    }
}

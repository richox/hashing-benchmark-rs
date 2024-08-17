use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Result};
use std::time::Instant;
use std::ptr::write_volatile;
use std::sync::OnceLock;

#[derive(Default)]
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= 0x4f2357a2e6b9d8c1;
        self.0 *= 0x8d1f5e2c84560baf;
        self.0 += 0xf0c3be9a71d8246e;
        self.0
    }
}

fn load_test_data(count: usize, max_len: usize) -> Result<Vec<Box<[u8]>>> {
    let mut bible_file = File::open("./test/bible.txt")?;
    let mut bible_text = String::new();
    bible_file.read_to_string(&mut bible_text)?;

    let mut vec: Vec<Box<[u8]>> = vec![];
    let mut r = Random::default();

    while vec.len() < count {
        let len = (r.next() as usize) % max_len;
        let offset = (r.next() as usize) % (bible_text.len() - len);
        vec.push(bible_text.as_bytes()[offset..][..len].into());
    }
    Ok(vec)
}

fn hash_one<H: Hasher>(hasher: impl Fn() -> H, value: &[u8]) -> u64 {
    let mut hasher = hasher();
    hasher.write(value.as_ref());
    hasher.finish()
}

fn bench_function<T: Copy + Default>(name: &str, hash: impl Fn(&[u8]) -> T) {
    const TEST_DATA_COUNT: usize = 100000;
    const TEST_DATA_MAX_LEN: usize = 100;
    const NUM_ROUNDS: usize = 1000;

    static TEST_DATA: OnceLock<Vec<Box<[u8]>>> = OnceLock::new();
    let test_data = TEST_DATA.get_or_init(|| {
        load_test_data(TEST_DATA_COUNT, TEST_DATA_MAX_LEN).expect("error loading test data")
    });

    let time = Instant::now();
    let mut unused_hash_ret = T::default();
    for _ in 0..NUM_ROUNDS {
        for record in test_data {
            unsafe {
                write_volatile(&mut unused_hash_ret, hash(record));
            }
        }
    }
    println!("{:.3} sec :: {name}", time.elapsed().as_secs_f64());
}

fn main() {
    bench_function("fnv", |x| hash_one(fnv::FnvHasher::default, x));
    bench_function("ahash", |x| hash_one(ahash::AHasher::default, x));
    bench_function("fxhash", |x| hash_one(fxhash::FxHasher::default, x));
    bench_function("gxhash", |x| hash_one(gxhash::GxHasher::default, x));
    bench_function("xxhash-rust", |x| hash_one(xxhash_rust::xxh64::Xxh64::default, x));
    bench_function("seahash", |x| hash_one(seahash::SeaHasher::default, x));
    bench_function("zwohash", |x| hash_one(zwohash::ZwoHasher::default, x));
    bench_function("tikv/mur3_32", |x| hash_one(|| mur3::Hasher32::with_seed(0), x));
    bench_function("tikv/mur3_128", |x| hash_one(|| mur3::Hasher128::with_seed(0), x));
    bench_function("highway", |x| hash_one(highway::HighwayHasher::default, x));
    bench_function("siphasher", |x| hash_one(siphasher::sip::SipHasher::default, x));
    bench_function("crc32fast", |x| hash_one(crc32fast::Hasher::default, x));
    bench_function("wyhash", |x| hash_one(wyhash::WyHash::default, x));
    bench_function("metrohash64", |x| hash_one(metrohash::MetroHash64::default, x));
    bench_function("metrohash128", |x| hash_one(metrohash::MetroHash128::default, x));
    bench_function("komihash", |x| hash_one(komihash::KomiHasher::default, x));
    bench_function("t1ha", |x| hash_one(t1ha::T1haHasher::default, x));
    bench_function("museair", |x| hash_one(museair::Hasher::default, x));
}

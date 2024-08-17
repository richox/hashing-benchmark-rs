hashing-benchmark-rs
===

rough benchmarks of hash functions available in rust.

result:

```
4.251 sec :: fnv
0.913 sec :: ahash
1.965 sec :: fxhash
1.053 sec :: gxhash
3.115 sec :: xxhash-rust
3.016 sec :: seahash
1.345 sec :: zwohash
2.613 sec :: tikv/mur3_32
2.888 sec :: tikv/mur3_128
3.496 sec :: highway
3.548 sec :: siphasher
5.728 sec :: crc32fast
2.408 sec :: wyhash
2.615 sec :: metrohash64
2.736 sec :: metrohash128
2.013 sec :: komihash
1.741 sec :: t1ha
2.819 sec :: museair
```

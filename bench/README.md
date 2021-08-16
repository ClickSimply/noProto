# NoProto Benchmarks

The benchmarks in this folder are used to record performance progress and provide entirely subjective comparisons to other similar projects.

All libraries are working with an object that contains the same data and fields.  Data types are matched as much as possible.

### Size Benchmark
The example object is encoded once, and it's size in bytes is recorded as well as it's size in bytes with zlib compression.

### Encode Benchmark
The example object is encoded/serialized into the format supported by the various libraries.  Specifically, the benchmark measures how long it takes to get an owned `Vec<u8>` out of the library.

### Decode All Benchmark
A single object is encoded, then the library decodes that object into it's parts 1,000,000 times.  Copying of the original buffer is only perfomed if it's needed by the library to complete decoding.  This measures how long it takes to go from a `Vec<u8>` to a shared immutable reference to all properties/values in the object.

### Decode One Benchmark
A single object is encoded, then the library decodes a single property of that object 1,000,000 times.  Copying of the original buffer is only perfomed if it's needed by the library to complete decoding.  This measures how long it takes to go from a `Vec<u8>` to a shared immutable reference of a single value in the object.

### Update One Benchmark
A single object is encoded, then the library should decode, update one property on the object then re encode the object 1,000,000 times.  The benchmark measures how long it takes to get from a deserialized buffer into another deserialized buffer with a single update performed in the new buffer.

Benchmarks can be ran with `cargo run --release`.

## Benchmark Rules
It's challenging to provide a level playing field for every library and implementation. As much as possible, the following guidelines are followed with building the benchmarks:

1. **Allocation is avoided wherever possible.**  With serialization and deserialization the cost of allocation is usually the most expensive part of encoding or decoding a message.  If a library provides an API that avoids allocation, that one is used where possible.
2. **Dynamic Array types must be used**.  Some libraries can avoid allocation by using a fixed size array in the benchmark, isntead of a dynamic one.  Since almost every library/format supports lists that can change in size, this optimization is not allowed even if it's possible.  The spirit of the benchmark is to test how quickly a dynamically sized array of items can be encoded/decoded, not a fixed size array (even though a fixed size is used).

# Benchmarks Histry

## Feb 8, 2021
### 0.9.4
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 308b, zlib: 198b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Postcard:    size: 128b, zlib: 119b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 311b, zlib: 193b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 337b
Flexbuffers: size: 490b, zlib: 309b
Abomonation: size: 261b, zlib: 160b
Rkyv:        size: 180b, zlib: 154b
Raw BSON:    size: 414b, zlib: 216b
MessagePack: size: 296b, zlib: 187b
Serde JSON:  size: 446b, zlib: 198b

======== ENCODE BENCHMARK ========
NoProto:          1393 ops/ms 1.00
Flatbuffers:      3165 ops/ms 2.27
Bincode:          6757 ops/ms 4.84
Postcard:         3067 ops/ms 2.20
Protobuf:          953 ops/ms 0.68
MessagePack:       661 ops/ms 0.47
JSON:              609 ops/ms 0.44
BSON:              129 ops/ms 0.09
Prost:            1464 ops/ms 1.05
Avro:              156 ops/ms 0.11
Flexbuffers:       444 ops/ms 0.32
Abomonation:      2342 ops/ms 1.68
Rkyv:             1605 ops/ms 1.15
Raw BSON:          130 ops/ms 0.09
MessagePack:       152 ops/ms 0.11
Serde JSON:        938 ops/ms 0.67

======== DECODE BENCHMARK ========
NoProto:          1883 ops/ms 1.00
Flatbuffers:     16393 ops/ms 8.57
Bincode:          9259 ops/ms 4.90
Postcard:         7519 ops/ms 3.99
Protobuf:         1305 ops/ms 0.69
MessagePack:       623 ops/ms 0.33
JSON:              481 ops/ms 0.26
BSON:              116 ops/ms 0.06
Prost:            2020 ops/ms 1.07
Avro:               57 ops/ms 0.03
Flexbuffers:       962 ops/ms 0.51
Abomonation:    125000 ops/ms 61.66
Rkyv:            37037 ops/ms 19.16
Raw BSON:         1117 ops/ms 0.59
MessagePack:       266 ops/ms 0.14
Serde JSON:        646 ops/ms 0.34

====== DECODE ONE BENCHMARK ======
NoProto:         55556 ops/ms 1.00
Flatbuffers:    250000 ops/ms 3.88
Bincode:         10000 ops/ms 0.18
Postcard:         7937 ops/ms 0.14
Protobuf:         1312 ops/ms 0.02
MessagePack:       832 ops/ms 0.02
JSON:              607 ops/ms 0.01
BSON:              123 ops/ms 0.00
Prost:            2232 ops/ms 0.04
Avro:               56 ops/ms 0.00
Flexbuffers:     24390 ops/ms 0.44
Abomonation:    500000 ops/ms 7.54
Rkyv:           200000 ops/ms 3.36
Raw BSON:        17857 ops/ms 0.32
MessagePack:       284 ops/ms 0.01
Serde JSON:        644 ops/ms 0.01

====== UPDATE ONE BENCHMARK ======
NoProto:          9524 ops/ms 1.00
Flatbuffers:      2532 ops/ms 0.27
Bincode:          4115 ops/ms 0.43
Postcard:         2469 ops/ms 0.26
Protobuf:          529 ops/ms 0.06
MessagePack:       202 ops/ms 0.02
JSON:              439 ops/ms 0.05
BSON:               90 ops/ms 0.01
Prost:            1040 ops/ms 0.11
Avro:               40 ops/ms 0.00
Flexbuffers:       294 ops/ms 0.03
Abomonation:      2183 ops/ms 0.23
Rkyv:             1531 ops/ms 0.16
Raw BSON:           89 ops/ms 0.01
MessagePack:       138 ops/ms 0.01
Serde JSON:        403 ops/ms 0.04
```

## Feb 7, 2021
### 0.9.3
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 209b, zlib: 167b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Postcard:    size: 128b, zlib: 119b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 311b, zlib: 193b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 339b
Flexbuffers: size: 490b, zlib: 309b
Abomonation: size: 261b, zlib: 163b
Rkyv:        size: 180b, zlib: 152b
Raw BSON:    size: 414b, zlib: 216b
MessagePack: size: 296b, zlib: 187b
Serde JSON:  size: 446b, zlib: 198b

======== ENCODE BENCHMARK ========
NoProto:          1258 ops/ms 1.00
Flatbuffers:      3086 ops/ms 2.45
Bincode:          6849 ops/ms 5.44
Postcard:         2841 ops/ms 2.26
Protobuf:          956 ops/ms 0.76
MessagePack:       661 ops/ms 0.53
JSON:              616 ops/ms 0.49
BSON:              129 ops/ms 0.10
Prost:            1570 ops/ms 1.25
Avro:              155 ops/ms 0.12
Flexbuffers:       455 ops/ms 0.36
Abomonation:      2347 ops/ms 1.86
Rkyv:             1684 ops/ms 1.34
Raw BSON:          129 ops/ms 0.10
MessagePack:       149 ops/ms 0.12
Serde JSON:        929 ops/ms 0.74

======== DECODE BENCHMARK ========
NoProto:          1901 ops/ms 1.00
Flatbuffers:     16393 ops/ms 8.54
Bincode:          9524 ops/ms 4.98
Postcard:         7634 ops/ms 4.00
Protobuf:         1269 ops/ms 0.67
MessagePack:       657 ops/ms 0.35
JSON:              497 ops/ms 0.26
BSON:              116 ops/ms 0.06
Prost:            2096 ops/ms 1.10
Avro:               56 ops/ms 0.03
Flexbuffers:       955 ops/ms 0.50
Abomonation:    125000 ops/ms 61.13
Rkyv:            37037 ops/ms 19.34
Raw BSON:         1134 ops/ms 0.60
MessagePack:       263 ops/ms 0.14
Serde JSON:        640 ops/ms 0.34

====== DECODE ONE BENCHMARK ======
NoProto:         55556 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.03
Bincode:         10204 ops/ms 0.19
Postcard:         7937 ops/ms 0.15
Protobuf:         1252 ops/ms 0.02
MessagePack:       879 ops/ms 0.02
JSON:              619 ops/ms 0.01
BSON:              124 ops/ms 0.00
Prost:            2151 ops/ms 0.04
Avro:               57 ops/ms 0.00
Flexbuffers:     24390 ops/ms 0.45
Abomonation:    500000 ops/ms 7.56
Rkyv:           200000 ops/ms 3.31
Raw BSON:        17857 ops/ms 0.33
MessagePack:       283 ops/ms 0.01
Serde JSON:        650 ops/ms 0.01

====== UPDATE ONE BENCHMARK ======
NoProto:         12821 ops/ms 1.00
Flatbuffers:      2551 ops/ms 0.20
Bincode:          4310 ops/ms 0.34
Postcard:         2273 ops/ms 0.18
Protobuf:          533 ops/ms 0.04
MessagePack:       213 ops/ms 0.02
JSON:              456 ops/ms 0.04
BSON:               90 ops/ms 0.01
Prost:            1079 ops/ms 0.08
Avro:               41 ops/ms 0.00
Flexbuffers:       297 ops/ms 0.02
Abomonation:      2041 ops/ms 0.16
Rkyv:             1560 ops/ms 0.12
Raw BSON:           90 ops/ms 0.01
MessagePack:       135 ops/ms 0.01
Serde JSON:        405 ops/ms 0.03
```


## Jan 13, 2021
### 0.9.1
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 209b, zlib: 167b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 311b, zlib: 193b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 336b
Flexbuffers: size: 490b, zlib: 309b
Abomonation: size: 261b, zlib: 159b
Rkyv:        size: 180b, zlib: 151b
Raw BSON:    size: 414b, zlib: 216b
MessagePack: size: 296b, zlib: 187b
Serde JSON: size: 446b, zlib: 198b

======== ENCODE BENCHMARK ========
NoProto:           998 ops/ms 1.00
Flatbuffers:      3205 ops/ms 3.21
Bincode:          6135 ops/ms 6.15
Protobuf:         1011 ops/ms 1.01
MessagePack:       681 ops/ms 0.68
JSON:              622 ops/ms 0.62
BSON:              130 ops/ms 0.13
Prost:            1548 ops/ms 1.55
Avro:              158 ops/ms 0.16
Flexbuffers:       447 ops/ms 0.45
Abomonation:      2710 ops/ms 2.71
Rkyv:             1658 ops/ms 1.66
Raw BSON:          128 ops/ms 0.13
MessagePack:       151 ops/ms 0.15
Serde JSON:       948 ops/ms 0.95

======== DECODE BENCHMARK ========
NoProto:          1645 ops/ms 1.00
Flatbuffers:     16393 ops/ms 9.95
Bincode:          9804 ops/ms 5.93
Protobuf:         1294 ops/ms 0.79
MessagePack:       627 ops/ms 0.38
JSON:              491 ops/ms 0.30
BSON:              115 ops/ms 0.07
Prost:            2075 ops/ms 1.26
Avro:               57 ops/ms 0.03
Flexbuffers:       950 ops/ms 0.58
Abomonation:    125000 ops/ms 71.37
Rkyv:            37037 ops/ms 22.23
Raw BSON:         1130 ops/ms 0.69
MessagePack:       246 ops/ms 0.15
Serde JSON:       650 ops/ms 0.39

====== DECODE ONE BENCHMARK ======
NoProto:         45455 ops/ms 1.00
Flatbuffers:    200000 ops/ms 3.97
Bincode:         10417 ops/ms 0.23
Protobuf:         1266 ops/ms 0.03
MessagePack:       833 ops/ms 0.02
JSON:              606 ops/ms 0.01
BSON:              122 ops/ms 0.00
Prost:            2151 ops/ms 0.05
Avro:               56 ops/ms 0.00
Flexbuffers:     25000 ops/ms 0.54
Abomonation:    500000 ops/ms 9.05
Rkyv:           200000 ops/ms 4.06
Raw BSON:        17544 ops/ms 0.39
MessagePack:       263 ops/ms 0.01
Serde JSON:       648 ops/ms 0.01

====== UPDATE ONE BENCHMARK ======
NoProto:         11905 ops/ms 1.00
Flatbuffers:      2500 ops/ms 0.21
Bincode:          4329 ops/ms 0.36
Protobuf:          539 ops/ms 0.05
MessagePack:       209 ops/ms 0.02
JSON:              441 ops/ms 0.04
BSON:               90 ops/ms 0.01
Prost:            1072 ops/ms 0.09
Avro:               41 ops/ms 0.00
Flexbuffers:       294 ops/ms 0.02
Abomonation:      2288 ops/ms 0.19
Rkyv:             1672 ops/ms 0.14
Raw BSON:           90 ops/ms 0.01
MessagePack:       130 ops/ms 0.01
Serde JSON:       407 ops/ms 0.03
```


## Jan 12, 2021
### 0.9.1
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 209b, zlib: 167b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 296b, zlib: 187b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 337b
Flexbuffers: size: 490b, zlib: 309b
Raw BSON:    size: 414b, zlib: 216b

======== ENCODE BENCHMARK ========
NoProto:          1032 ops/ms 1.00
Flatbuffers:      3195 ops/ms 3.09
Bincode:          6135 ops/ms 5.94
Protobuf:          981 ops/ms 0.95
MessagePack:       156 ops/ms 0.15
JSON:              618 ops/ms 0.60
BSON:              131 ops/ms 0.13
Prost:            1567 ops/ms 1.52
Avro:              158 ops/ms 0.15
Flexbuffers:       447 ops/ms 0.43
Raw BSON:          130 ops/ms 0.13

======== DECODE BENCHMARK ========
NoProto:          1608 ops/ms 1.00
Flatbuffers:     16393 ops/ms 10.15
Bincode:          9804 ops/ms 6.07
Protobuf:         1245 ops/ms 0.77
MessagePack:       253 ops/ms 0.16
JSON:              489 ops/ms 0.30
BSON:              117 ops/ms 0.07
Prost:            2105 ops/ms 1.31
Avro:               58 ops/ms 0.04
Flexbuffers:       943 ops/ms 0.59
Raw BSON:          441 ops/ms 0.27

====== DECODE ONE BENCHMARK ======
NoProto:         47619 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.58
Bincode:         10204 ops/ms 0.22
Protobuf:         1264 ops/ms 0.03
MessagePack:       264 ops/ms 0.01
JSON:              587 ops/ms 0.01
BSON:              125 ops/ms 0.00
Prost:            2304 ops/ms 0.05
Avro:               57 ops/ms 0.00
Flexbuffers:     25000 ops/ms 0.54
Raw BSON:        18519 ops/ms 0.41

====== UPDATE ONE BENCHMARK ======
NoProto:         11628 ops/ms 1.00
Flatbuffers:      2506 ops/ms 0.22
Bincode:          4525 ops/ms 0.39
Protobuf:          546 ops/ms 0.05
MessagePack:       134 ops/ms 0.01
JSON:              433 ops/ms 0.04
BSON:               91 ops/ms 0.01
Prost:            1064 ops/ms 0.09
Avro:               40 ops/ms 0.00
Flexbuffers:       294 ops/ms 0.03
Raw BSON:           90 ops/ms 0.01
```

## Jan 10, 2021
### v0.9.0
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 209b, zlib: 167b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 296b, zlib: 187b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 337b
Flexbuffers: size: 490b, zlib: 309b

======== ENCODE BENCHMARK ========
NoProto:           920 ops/ms 1.00
Flatbuffers:      1062 ops/ms 1.15
Bincode:          5882 ops/ms 6.37
Protobuf:          876 ops/ms 0.95
MessagePack:       136 ops/ms 0.15
JSON:              546 ops/ms 0.59
BSON:              115 ops/ms 0.13
Prost:            1361 ops/ms 1.48
Avro:              140 ops/ms 0.15
Flexbuffers:       399 ops/ms 0.43

======== DECODE BENCHMARK ========
NoProto:          1397 ops/ms 1.00
Flatbuffers:     14925 ops/ms 10.68
Bincode:          8621 ops/ms 6.17
Protobuf:         1140 ops/ms 0.82
MessagePack:       223 ops/ms 0.16
JSON:              436 ops/ms 0.31
BSON:              103 ops/ms 0.07
Prost:            1855 ops/ms 1.33
Avro:               51 ops/ms 0.04
Flexbuffers:       843 ops/ms 0.60

====== DECODE ONE BENCHMARK ======
NoProto:         41667 ops/ms 1.00
Flatbuffers:    250000 ops/ms 5.01
Bincode:          9174 ops/ms 0.23
Protobuf:         1155 ops/ms 0.03
MessagePack:       236 ops/ms 0.01
JSON:              533 ops/ms 0.01
BSON:              109 ops/ms 0.00
Prost:            1942 ops/ms 0.05
Avro:               51 ops/ms 0.00
Flexbuffers:     22727 ops/ms 0.56

====== UPDATE ONE BENCHMARK ======
NoProto:         10526 ops/ms 1.00
Flatbuffers:      1057 ops/ms 0.10
Bincode:          4000 ops/ms 0.38
Protobuf:          474 ops/ms 0.05
MessagePack:       121 ops/ms 0.01
JSON:              400 ops/ms 0.04
BSON:               80 ops/ms 0.01
Prost:             966 ops/ms 0.09
Avro:               37 ops/ms 0.00
Flexbuffers:       265 ops/ms 0.03
```

## Jan 4, 2021
### v0.7.4
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 208b, zlib: 166b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 296b, zlib: 187b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b
Avro:        size: 702b, zlib: 336b
Flexbuffers: size: 490b, zlib: 309b

======== ENCODE BENCHMARK ========
NoProto:          1057 ops/ms 1.00
Flatbuffers:      1046 ops/ms 0.99
Bincode:          5882 ops/ms 5.55
Protobuf:          859 ops/ms 0.81
MessagePack:       135 ops/ms 0.13
JSON:              550 ops/ms 0.52
BSON:              115 ops/ms 0.11
Prost:            1225 ops/ms 1.16
Avro:              138 ops/ms 0.13
Flexbuffers:       401 ops/ms 0.38

======== DECODE BENCHMARK ========
NoProto:          1437 ops/ms 1.00
Flatbuffers:     14706 ops/ms 10.21
Bincode:          8772 ops/ms 6.08
Protobuf:         1140 ops/ms 0.79
MessagePack:       222 ops/ms 0.15
JSON:              438 ops/ms 0.31
BSON:              103 ops/ms 0.07
Prost:            1866 ops/ms 1.30
Avro:               51 ops/ms 0.04
Flexbuffers:       855 ops/ms 0.60

====== DECODE ONE BENCHMARK ======
NoProto:         47619 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.55
Bincode:          9524 ops/ms 0.21
Protobuf:         1163 ops/ms 0.03
MessagePack:       237 ops/ms 0.01
JSON:              544 ops/ms 0.01
BSON:              109 ops/ms 0.00
Prost:            1984 ops/ms 0.04
Avro:               52 ops/ms 0.00
Flexbuffers:     23256 ops/ms 0.50

====== UPDATE ONE BENCHMARK ======
NoProto:         12195 ops/ms 1.00
Flatbuffers:      1065 ops/ms 0.09
Bincode:          4016 ops/ms 0.33
Protobuf:          480 ops/ms 0.04
MessagePack:       119 ops/ms 0.01
JSON:              396 ops/ms 0.03
BSON:               80 ops/ms 0.01
Prost:             962 ops/ms 0.08
Avro:               37 ops/ms 0.00
Flexbuffers:       264 ops/ms 0.02
```

## Dec 27, 2020
### v0.7.1
M1 Macbook Air with 8GB RAM (Native)
```
========= SIZE BENCHMARK =========
NoProto:     size: 208b, zlib: 166b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
Protobuf:    size: 154b, zlib: 141b
MessagePack: size: 296b, zlib: 187b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b
Prost:       size: 154b, zlib: 142b

======== ENCODE BENCHMARK ========
NoProto:          1170 ops/ms 1.00
Flatbuffers:      1188 ops/ms 1.02
Bincode:          6250 ops/ms 5.33
Protobuf:          986 ops/ms 0.84
MessagePack:       155 ops/ms 0.13
JSON:              607 ops/ms 0.52
BSON:              129 ops/ms 0.11
Prost:            1558 ops/ms 1.33

======== DECODE BENCHMARK ========
NoProto:          1634 ops/ms 1.00
Flatbuffers:     15873 ops/ms 9.59
Bincode:          9804 ops/ms 5.98
Protobuf:         1274 ops/ms 0.78
MessagePack:       262 ops/ms 0.16
JSON:              476 ops/ms 0.29
BSON:              120 ops/ms 0.07
Prost:            2049 ops/ms 1.25

====== DECODE ONE BENCHMARK ======
NoProto:         50000 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.27
Bincode:         10526 ops/ms 0.21
Protobuf:         1245 ops/ms 0.03
MessagePack:       281 ops/ms 0.01
JSON:              599 ops/ms 0.01
BSON:              130 ops/ms 0.00
Prost:            2193 ops/ms 0.05

====== UPDATE ONE BENCHMARK ======
NoProto:         13333 ops/ms 1.00
Flatbuffers:      1208 ops/ms 0.09
Bincode:          4484 ops/ms 0.34
Protobuf:          531 ops/ms 0.04
MessagePack:       138 ops/ms 0.01
JSON:              444 ops/ms 0.03
BSON:               95 ops/ms 0.01
Prost:            1089 ops/ms 0.08
```

## Dec 25, 2020
### v0.7.1
M1 Macbook Air with 8GB RAM (Native)

```
========= SIZE BENCHMARK =========
NoProto:     size: 209b, zlib: 167b
Flatbuffers: size: 264b, zlib: 181b
Bincode:     size: 163b, zlib: 129b
ProtoBuf:    size: 154b, zlib: 141b
MessagePack: size: 296b, zlib: 187b
JSON:        size: 439b, zlib: 184b
BSON:        size: 414b, zlib: 216b

======== ENCODE BENCHMARK ========
NoProto:          1209 ops/ms 1.00
Flatbuffers:      1189 ops/ms 0.98
Bincode:          6250 ops/ms 5.15
ProtoBuf:          958 ops/ms 0.79
MessagePack:       154 ops/ms 0.13
JSON:              606 ops/ms 0.50
BSON:              127 ops/ms 0.10

======== DECODE BENCHMARK ========
NoProto:          1653 ops/ms 1.00
Flatbuffers:     15625 ops/ms 9.38
Bincode:          9434 ops/ms 5.68
ProtoBuf:         1263 ops/ms 0.76
MessagePack:       242 ops/ms 0.15
JSON:              471 ops/ms 0.29
BSON:              122 ops/ms 0.07

====== DECODE ONE BENCHMARK ======
NoProto:         50000 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.15
Bincode:         10309 ops/ms 0.21
ProtoBuf:         1285 ops/ms 0.03
MessagePack:       271 ops/ms 0.01
JSON:              605 ops/ms 0.01
BSON:              132 ops/ms 0.00

====== UPDATE ONE BENCHMARK ======
NoProto:         14085 ops/ms 1.00
Flatbuffers:      1200 ops/ms 0.09
Bincode:          4367 ops/ms 0.31
ProtoBuf:          556 ops/ms 0.04
MessagePack:       136 ops/ms 0.01
JSON:              445 ops/ms 0.03
BSON:               96 ops/ms 0.01

```


## Dec 21, 2020
### v0.7.1
M1 Macbook Air with 8GB RAM (Native)

```
========= SIZE BENCHMARK =========
NoProto:     size: 284b, zlib: 229b
Flatbuffers: size: 336b, zlib: 214b
ProtoBuf:    size: 220b, zlib: 163b
MessagePack: size: 431b, zlib: 245b
JSON:        size: 673b, zlib: 246b
BSON:        size: 600b, zlib: 279b

======== ENCODE BENCHMARK ========
NoProto:           822 ops/ms 1.00
Flatbuffers:      1209 ops/ms 1.47
ProtoBuf:          723 ops/ms 0.88
MessagePack:        99 ops/ms 0.12
JSON:              436 ops/ms 0.53
BSON:               82 ops/ms 0.10

======== DECODE BENCHMARK ========
NoProto:          1105 ops/ms 1.00
Flatbuffers:     14925 ops/ms 13.45
ProtoBuf:          881 ops/ms 0.80
MessagePack:       163 ops/ms 0.15
JSON:              299 ops/ms 0.27
BSON:               78 ops/ms 0.07

====== DECODE ONE BENCHMARK ======
NoProto:         52632 ops/ms 1.00
Flatbuffers:    250000 ops/ms 4.17
ProtoBuf:          902 ops/ms 0.02
MessagePack:       171 ops/ms 0.00
JSON:              374 ops/ms 0.01
BSON:               83 ops/ms 0.00

====== UPDATE ONE BENCHMARK ======
NoProto:         10638 ops/ms 1.00
Flatbuffers:      1176 ops/ms 0.11
ProtoBuf:          384 ops/ms 0.04
MessagePack:        91 ops/ms 0.01
JSON:              287 ops/ms 0.03
BSON:               62 ops/ms 0.01
```

## Dec 20, 2020
### v0.7.0
3.4Ghz i5 2017 21.5" iMac with 32 GB RAM

```
========= SIZE BENCHMARK =========
NoProto:     size: 284b, zlib: 229b
ProtoBuf:    size: 220b, zlib: 163b
MessagePack: size: 431b, zlib: 245b
JSON:        size: 673b, zlib: 246b
BSON:        size: 600b, zlib: 279b

======== ENCODE BENCHMARK ========
NoProto:           312 ops/ms 1.00
ProtoBuf:          270 ops/ms 0.87
MessagePack:        38 ops/ms 0.12
JSON:              167 ops/ms 0.54
BSON:               28 ops/ms 0.09

======== DECODE BENCHMARK ========
NoProto:           469 ops/ms 1.00
ProtoBuf:          390 ops/ms 0.83
MessagePack:        70 ops/ms 0.15
JSON:              134 ops/ms 0.28
BSON:               34 ops/ms 0.07

====== DECODE ONE BENCHMARK ======
NoProto:         27027 ops/ms 1.00
ProtoBuf:          400 ops/ms 0.02
MessagePack:        80 ops/ms 0.00
JSON:              167 ops/ms 0.01
BSON:               35 ops/ms 0.00

====== UPDATE ONE BENCHMARK ======
NoProto:          3953 ops/ms 1.00
ProtoBuf:          167 ops/ms 0.04
MessagePack:        35 ops/ms 0.01
JSON:              127 ops/ms 0.03
BSON:               26 ops/ms 0.01
```


## Dec 15, 2020
### v0.6.1
3.4Ghz i5 2017 21.5" iMac with 32 GB RAM

```
========= SIZE BENCHMARK =========
NoProto:     size: 284b, zlib: 229b
ProtoBuf:    size: 220b, zlib: 163b
MessagePack: size: 431b, zlib: 245b
JSON:        size: 673b, zlib: 246b
BSON:        size: 600b, zlib: 279b

======== ENCODE BENCHMARK ========
NoProto:           272 ops/ms 1.00
ProtoBuf:          266 ops/ms 0.98
MessagePack:        33 ops/ms 0.12
JSON:              186 ops/ms 0.68
BSON:               28 ops/ms 0.10

======== DECODE BENCHMARK ========
NoProto:           375 ops/ms 1.00
ProtoBuf:          365 ops/ms 0.97
MessagePack:        63 ops/ms 0.17
JSON:              127 ops/ms 0.29
BSON:               28 ops/ms 0.07

====== DECODE ONE BENCHMARK ======
NoProto:          5051 ops/ms 1.00
ProtoBuf:          366 ops/ms 0.07
MessagePack:        68 ops/ms 0.01
JSON:              153 ops/ms 0.03
BSON:               30 ops/ms 0.01

====== UPDATE ONE BENCHMARK ======
NoProto:          4098 ops/ms 1.00
ProtoBuf:          160 ops/ms 0.04
MessagePack:        31 ops/ms 0.01
JSON:              115 ops/ms 0.03
BSON:               22 ops/ms 0.01
```

## Dec 13, 2020
### v0.6.0
3.4Ghz i5 2017 21.5" iMac with 32 GB RAM

```
====== SIZE BENCHMARK ======
NoProto:     size: 283b, zlib: 226b  1x
Flatbuffers: size: 336b, zlib: 214b  1.2x
ProtoBuf:    size: 220b, zlib: 163b  0.8x
MessagePack: size: 431b, zlib: 245b  1.5x
JSON:        size: 673b, zlib: 246b  2.4x
BSON:        size: 600b, zlib: 279b  2.1x

====== ENCODE BENCHMARK ======
NoProto:     3.536623s   (283 ops/ms)
Flatbuffers: 1.942583s   (514 ops/ms)
ProtoBuf:    3.551301s   (281 ops/ms)
MessagePack: 28.050727s   (35 ops/ms)
JSON:        5.436352s   (184 ops/ms)
BSON:        36.564978s   (27 ops/ms)

====== DECODE BENCHMARK ======
NoProto:     2.496591s     (400 ops/ms)
Flatbuffers: 320.065ms   (3,124 ops/ms)
ProtoBuf:    2.888706s     (346 ops/ms)
MessagePack: 16.576576s   (60.3 ops/ms)
JSON:        8.957872s     (111 ops/ms)
BSON:        32.770133s   (30.5 ops/ms)

====== DECODE ONE BENCHMARK ======
NoProto:     206.966ms  (4,831 ops/ms)
Flatbuffers: 13.127ms  (76,178 ops/ms)
ProtoBuf:    2.715129s    (368 ops/ms)
MessagePack: 14.300117s    (69 ops/ms)
JSON:        7.836841s    (127 ops/ms)
BSON:        37.513607s    (26 ops/ms)

====== UPDATE ONE BENCHMARK ======
NoProto:     264.399ms (3,782 ops/ms)
Flatbuffers: 3.086538s   (324 ops/ms)
ProtoBuf:    10.119442s   (99 ops/ms)
MessagePack: 35.322739s   (28 ops/ms)
JSON:        9.749246s   (102 ops/ms)
BSON:        48.0097s     (21 ops/ms)
```

## Dec 1, 2020
### v0.5.1 
Macbook Air M1 with 8GB (Rosetta)

```
====== SIZE BENCHMARK ======
NoProto:     size: 408b, zlib: 321b
Flatbuffers: size: 336b, zlib: 214b
ProtoBuf:    size: 220b, zlib: 163b

====== ENCODE BENCHMARK ======
NoProto:     5.707984s (175 ops/ms)
Flatbuffers: 1.556862s (642 ops/ms)
ProtoBuf:    2.209196s (452 ops/ms)

====== DECODE BENCHMARK ======
NoProto:     9.161315s (109 ops/ms)
Flatbuffers: 105.914ms (9,441 ops/ms)
ProtoBuf:    1.691681s (591 ops/ms)

====== UPDATE BENCHMARK ======
NoProto:     602.446ms (1,659 ops/ms)
Flatbuffers: 1.512228s (661 ops/ms)
ProtoBuf:    3.791677s (263 ops/ms)
```
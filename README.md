# bls test

Aggregated signature test. 

Base on [bls](https://github.com/filecoin-project/bls-signatures) lib.

### Basic info

Length of private key is 32 bytes.

Length of public key is 48 bytes.

Length of signature is 96 bytes.

### benchmark

test on i7-7700K

```
test bench_serialize_private_key_as_bytes   ... bench:          35 ns/iter (+/- 1)
test bench_serialize_private_key_from_bytes ... bench:          35 ns/iter (+/- 0)
test bench_serialize_public_key_as_bytes    ... bench:         136 ns/iter (+/- 0)
test bench_serialize_public_key_from_bytes  ... bench:     199,012 ns/iter (+/- 514)
test bench_serialize_signature_as_bytes     ... bench:         186 ns/iter (+/- 4)
test bench_serialize_signature_from_bytes   ... bench:     746,937 ns/iter (+/- 9,975)

test sign_64b ... bench:   2,359,251 ns/iter (+/- 33,605)

test bench_verify_1   ... bench:   2,030,487 ns/iter (+/- 39,315)
test bench_verify_10  ... bench:   4,488,402 ns/iter (+/- 65,385)
test bench_verify_100 ... bench:  26,734,511 ns/iter (+/- 310,719)
```


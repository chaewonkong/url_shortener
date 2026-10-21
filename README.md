# URL Shortener

Simple URL Shortener with Rust and Redis

## Usage

###  set shorten URL

The following will return shortened URL
```
curl -i -X POST http://localhost:3000/set \
  -H 'Content-Type: application/json' \
  -d '{"url": "blog.engineerd.net"}'
```

### Redirect
In  browser, type shortened URL, then it will automatically redirect page to the original URL.

## Key generation

Keys are 7-char base62 strings sampled uniformly from a CSPRNG
(`rand::rng()`, ChaCha12 seeded from OS entropy) and stored with
`SETNX`, which enforces uniqueness atomically.

- **Key space**: 62^7 ≈ 3.5 × 10^12 (~41.7 bits).
- **Collision rate**: per insert ≈ n / 62^7, where n is the number of
  stored keys — e.g. ~0.03% at 10^9 keys. Expected attempts ≈ 1 / (1 − n/N).
- **Retry policy**: 3 attempts per length, then grow by 1 char, up to 10.
  Exhausting all attempts indicates a bug, not bad luck — alert on it.
- **Security scope**: keys resist enumeration only by sparsity
  (hit rate = n/N). They are not secrets; anyone with a link can follow it.
  Rate limiting and 404-rate monitoring are required alongside.
- **When to act**: if length-extension events become non-zero in steady
  state, raise `BASE_LEN`. Higher length lowers both collision and
  enumeration hit rate by 62×.
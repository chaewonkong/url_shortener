# URL Shortener

Simple URL Shortener with Rust and Redis

## Usage

###  set shorten URL

The following will return shortened URL
```
curl -i -X POST http://localhost:3000/set \
  -H 'Content-Type: application/json' \
  -d '{"url": "engineerd.com"}'
```

### Redirect
In  browser, type shortened URL, then it will automatically redirect page to the original URL.

## TODO List
- [ ] Support Redis Cluster
- [ ] Periodically sync original-URL:shorten-URL from Redis to the persistent data storage; RDB
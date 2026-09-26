# URL Shortener

Simple URL Shortener with Rust and Redis

## Methods
- `GET /{shorten_url}`: redirects to original URL
- `POST /set -d '{"url": {url}}'`: Set URL and returns shortened version
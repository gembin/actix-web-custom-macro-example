# actix-web-custom-macro-example

actix-web custom macro handled by a middleware example.

## Server
```bash
cargo run
```

## Client
```bash
curl -v http://localhost:8080/hello
```
Expecting `x-foobar: test` in the HTTP header of the response
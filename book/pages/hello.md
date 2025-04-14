# Hello World

```rust
{{#include ../../examples/hello.rs }}
```

## Run the example:

```
cargo run --example hello
```

Using your browser visit `http://localhost:3030/`

Using [curl](curl.se/):

```
$ curl http://localhost:3030
Hello, World!
```

Change the content of the string to `<b>Hello</b>, World!`, that is we would like to return some HTML as well.

Using Ctrl-C stop the server process and run it again.

If you  reload the web page at `http://localhost:3030/` you will see it display the HTML tag instead of making the work bold.


This happens when the server returns the content with the `Content-type` set to `text/plain`.

Using the `-i` flag of `curl` we can see the header that shows the content-type being `text/plain`.

```
$ curl -i http://localhost:3030
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 20
date: Sat, 12 Apr 2025 07:11:14 GMT

<b>Hello</b>, World!
```


* TODO: how to change the content type?

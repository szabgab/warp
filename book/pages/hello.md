# Hello World

```rust
{{#include ../../examples/hello.rs }}
```

## Run the example:

```
cargo run --example hello
```

Using your browser visit `http://localhost:3030/`

You will see the browser displaying **Hello, World!**.

In another terminal you can check the site using [curl](curl.se/) sending a `GET` request to the URL:

```
$ curl http://localhost:3030
Hello, World!
```

This is a very simple example that has many issues. We'll see some of them and step-by-step we'll improve.

## Problems

### This example will respond to `any` request with the same content.

A `GET` request to any other path:

```
curl http://localhost:3030/hi
```

A `POST` request to any path:

```
curl -X POST http://localhost:3030/oups
```

### We cannot test this code without launching a server.

I don't have much to say about it. It is what it is. We'll soon have a way to test the code easily.

### This code sets the Content-type to `text/plain`.

In order to see this change the content of the string to `<b>Hello</b>, World!`, that is we would like to return some HTML as well.

Using Ctrl-C stop the server process and run it again.

If you reload the web page at `http://localhost:3030/` you will see it display the HTML tag instead of making the work bold.

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

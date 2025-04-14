# Hello World using functions

```rust
{{#include ../../examples/hello_using_functions.rs }}
```

In this example we have a function called `setup_routes` that create all the mappings between URL pathes and requst methods (such as `GET` and `POST`) on one hand and function that will fulfill that request on the other hand. In this first example we have only one route that deals with the empty path (or `/` if you wish) as defined using the `warp::path!()` macro **and** the `GET` request method as defined by the `warp::get()` function call.

This request is mapped to the arbitrarily named `say_hello` function that will return some HTML using the `warp::reply::html` function call.

We can run this example with the following command:

```
cargo run --example hello_using_functions
```

Then we can visit `http://localhost:3030/` and observe that the response is "Hello, **World**!". The word World being bold and we don't see the HTML tags.
That's because instead of returning a plain string using `Ok("Hello, <b>World</b>!")`, the `warp::reply::html` function call set the `Content-type` to be
`text/html`.

We can easily observe this using `curl` in another terminal:

```
$ curl -i  http://localhost:3030/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 20
date: Mon, 14 Apr 2025 15:41:21 GMT

Hello, <b>World</b>!
```

We can also check other pathes and observe they return a `404 Not Found` with an empty page:

```
 $ curl -i  http://localhost:3030/hi
HTTP/1.1 404 Not Found
content-length: 0
date: Mon, 14 Apr 2025 15:42:02 GMT

```

Similarily using the `POST` method will yield a `405 Method Not Allowed` error with some text:


```
$ curl -i -X POST  http://localhost:3030/
HTTP/1.1 405 Method Not Allowed
content-type: text/plain; charset=utf-8
content-length: 23
date: Mon, 14 Apr 2025 15:43:07 GMT

HTTP method not allowed
```



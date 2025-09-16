# Description
A sample program to run a youtube video embedded in a browser.

Significantly based on [Brooks Patton](https://github.com/brooks-builds/lms/).

# Execution
Since this is a Yew/WASM project, use trunk instead of cargo:

```
# Instead of cargo build
trunk build

# Instead of cargo run
trunk serve

# For development with hot reload
trunk serve --open
```

# Errors

## Building without Trunk configuration
```
❯ trunk build
2025-09-16T21:58:52.858186Z  INFO 🚀 Starting trunk 0.21.14
2025-09-16T21:58:52.858540Z ERROR Unable to find any Trunk configuration
```

## Building without installing all wasm dependencies
```
  = note: the `wasm32-unknown-unknown` target may not be installed
  = help: consider downloading the target with `rustup target add wasm32-unknown-unknown`
```

## Running using a URL that generates a CORS error
```
Refused to display 'https://www.youtube.com/' in a frame because it set 'X-Frame-Options' to 'sameorigin'.
```

The issue is that you're trying to use `https://youtu.be/embed/jdNp4UPo4RM` as the source URL, but YouTube's X-Frame-Options:
  sameorigin policy prevents embedding YouTube pages directly in iframes from different origins.

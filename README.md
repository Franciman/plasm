# plasm
A FAST mathematical function plotter which builds to both desktop (Rust + OpenGL) and web (Rust to WebAssembly).

## Build
### Desktop
```console
$ cargo run
```

### Web
```console
$ wasm-pack build --target web --out-name web --out-dir pkg
```
Install a server that properly defines the `application/wasm` mime type for example:
```console
$ npm install -g http-server
```
Start the server
```console
$ http-server
``` 
Go to http://localhost:8080 in a browser.


## Current feature
Simple 2D mathematical functions plotter.

## TODO:
- Implicit functions
- 3D functions
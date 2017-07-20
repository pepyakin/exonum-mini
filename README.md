# exonum-mini

Quick sketch of repackaging [Exonum](https://github.com/exonum/exonum) to use it as Rust-WebAssembly light client.

Don't even think to use this project or part of it in production.

## Build

To build this example you need Rust with Emscripten flavour. You can learn how to get one with [this](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde) awesome guide (I used rust `1.20.0-nightly (8f1339af2 2017-07-16)`). I assume that `emcc` and co will be on your PATH.

Then you need custom built libsodium.

```sh
git clone https://github.com/jedisct1/libsodium.git
cd libsodium

# Save current dir, you will need it later.
export LIBSODIUM_DIR=`pwd`

./autogen.sh
./dist-build/emscripten-wasm.sh
```

Now you are ready to build this project.

```sh
export SODIUM_LIB_DIR=$LIBSODIUM_DIR/libsodium-js/lib/
export SODIUM_STATIC=1
cargo run --target=wasm32-unknown-emscripten
```

If everything goes right, you will able to find WebAssembly binaries at `target/wasm32-unknown-emscripten/debug/deps/`.

Ah, by the way, there is [prebuilt binaries](https://github.com/pepyakin/exonum-mini/tree/master/prebuilt) if you like.

## Running

To run it you need to use Node 8 or later. 

```
/usr/local/bin/node currency-f42481f1bd0ecf7d.js
```

If all goes ok, you should see something like:
```
json={"body":{"name":"John","pub_key":"8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c"},"message_id":1,"network_id":0,"protocol_version":0,"service_id":1,"signature":"db68168fcc45befc88b98417bf593ea7bc6bd2d387e3718273ad605cfcf3c709e700d7ba4e4fe112beb16ac0d950bd849d29b93d2fb54432c1ba4aeb75f3c306"}
```

If you see something like:

```
no native wasm support detected
...
```

It means you using Node without WebAssembly support.
Please note, that `emsdk_env` may override your `node`. To workaround you can either create a new terminal session or use you local `node`

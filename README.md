# Simple test of embedding deno

This is a simple test of embedding deno and importing an ES module. The intent
is to create a customized runtime which has some preloaded modules and a simple
script can be run leveraging those modules.

## Build

```
cargo build
```

## Run

This will set up the module `double.mjs` as a side loaded module which can then
be dynamically imported from a script; the script is then run, imports the module
and calls into the module to perform a simple calculation.

```
target/debug/jsruntime_test /home/ubuntu/deno/runtime_test/double.mjs /home/ubuntu/deno/runtime_test/input.js
```

Note that it is necessary to have consistency between the module name in `input.js`
and that provided as a parameter when running `jsruntime_test` - otherwise, the
module is not loaded and nothing happens.

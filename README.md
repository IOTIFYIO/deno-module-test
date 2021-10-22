# Simple test of embedding deno

This is a simple test of embedding deno and importing an ES module. The original
idea was to load an ES module and trigger it from a script (which is a specific
notion within deno - a script can be triggered via the the `execute_script()`
function). This was modified to use a so-called main module as the entry point
which enables ES modules to be loaded using an `await` without any problems.

## Build

```
cargo build
```

## Run

This will set up the module `double.mjs` as a side loaded module which can then
be dynamically imported from a main module; the main module is then run,
imports the side module and calls into the side module to perform a simple
calculation.

```
target/debug/jsruntime_test /home/ubuntu/deno/runtime_test/double.mjs /home/ubuntu/deno/runtime_test/input.js
```

Note that it is necessary to have consistency between the side module name in
`input.js` and that provided as a parameter when running `jsruntime_test` -
otherwise, the side module is not loaded and nothing happens.

Deno.core.print("About to load dynamic module...\n");

// note that the module name must be consistent with the module name provided to the runtime_test
import('file:///home/ubuntu/deno/runtime_test/double.mjs')
  .then((module) => {
    // Do something with the module.
    Deno.core.print("Module loaded...\n");

    Deno.core.print("Running function from module...\n");
    let m = module.double(4) ;

    Deno.core.print("Returned from function...\n") ;

    for (let i = 0; i < m; i++) {
      Deno.core.print("Iterating in loop...\n") ;
    }
  });


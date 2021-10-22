const module_name = 'file:///home/ubuntu/deno/runtime_test/double.mjs';
const module = await import(module_name);

async function run() {
    let m = module.double(4) ;
    
    for (let i = 0; i < m; i++) {
        Deno.core.print("Iterating in loop...\n") ;
    }
}

Deno.core.print("About to run function...\n");
run() ;

//!  This example shows you how to load a side module and execute a script which dynamically
//!  loads the side module.

use deno_core::JsRuntime;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;
//use std::path::Path;
//use std::fs;
use url::Url;
use std::rc::Rc;
use tokio::runtime::Runtime;
use std::env;
use std::process;

// load a side module
fn load_side_module(rt: &Runtime, runtime: &mut JsRuntime, module_filename: String) {

  let module_filename_as_url = "file://".to_string() + &module_filename;
  let module_url = Url::parse(&module_filename_as_url).unwrap();
  let mut module_id = 0;
  let async_block = async {
    module_id = runtime.load_side_module(&module_url, None).await.unwrap();
    println!("Module id = {}", module_id);

  };
  rt.block_on(async_block);

  let mut receiver = runtime.mod_evaluate(module_id);

  let mod_eval_async = async {
    tokio::select! {
      maybe_result = &mut receiver => {
        maybe_result.expect("Module evaluation result not provided.")
      }

      event_loop_result = runtime.run_event_loop(false) => {
        event_loop_result?;
        let maybe_result = receiver.await;
        maybe_result.expect("Module evaluation result not provided.")
      }
    }
  };


  let mod_eval_result = rt.block_on(mod_eval_async);
  match mod_eval_result {
      Ok(_result) => println!("Module evaluated successfully..."),
      Err(error) => println!("Error evaluating module {}", error),
  }
}

fn parse_args(args: &[String]) -> Result<(&str, &str), &str> {
    if args.len() != 3 {
        return Err("Incorrect number of arguments - please provide 2 arguments: module name and script name");
    }
    let side_module_name = &args[1] ;
    let main_module_name = &args[2] ;

    Ok((side_module_name, main_module_name))
}

fn main() {

  // assume we have two arguments - the module to load and the script to run
  let args: Vec<String> = env::args().collect();
  let (module_filename, main_module_filename) = parse_args(&args).unwrap_or_else(|err| { 
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  let rt = tokio::runtime::Runtime::new().unwrap();

  // Initialize a runtime instance
  let loader = Rc::new(FsModuleLoader);
  let mut runtime = JsRuntime::new(RuntimeOptions {
    module_loader: Some(loader),
    ..Default::default()
  });

  load_side_module(&rt, &mut runtime, module_filename.to_string()) ;

  //let filename = Path::new(main_module_filename);
  //let script_file = fs::read_to_string(filename)
  //      .expect("Unable to read main module file");

  let module_filename_as_url = "file://".to_string() + &main_module_filename;
  let module_url = Url::parse(&module_filename_as_url).unwrap();
  let mut module_id = 0;
  let async_block = async {
    module_id = runtime.load_main_module(&module_url, None).await.unwrap();
    println!("Module id = {}", module_id);
  };

  rt.block_on(async_block);

  let mut receiver = runtime.mod_evaluate(module_id);

  //let execute_result = runtime.load_main_module("test-script", &script_file);
  //match execute_result {
  //    Ok(_result) => println!("Execute script successful..."),
  //    Err(error) => {
  //        println!("Error executing script {}", error);
  //        panic!()
  //    }
  //}

  println!("Passing control to deno runtime via event_loop...");
  let mod_eval_async = async {
    tokio::select! {
      maybe_result = &mut receiver => {
        maybe_result.expect("Module evaluation result not provided.")
      }

      event_loop_result = runtime.run_event_loop(false) => {
        event_loop_result?;
        let maybe_result = receiver.await;
        maybe_result.expect("Module evaluation result not provided.")
      }
    }
  };

  //let event_loop_async  = async {
  //  if let Ok(_event_loop_result) = runtime.run_event_loop(false).await {
  //    println!("Event loop terminated successfully...");
  //  } else {
  //    println!("Event loop terminated unsuccessfully");
  //  }
  //};
  //
  rt.block_on(mod_eval_async);
}


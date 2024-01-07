use std::env::{Args, args};

pub fn fetch() -> Vec<String> {
    let args: Args = args();
    let mut args_copy: Vec<String> = vec![];

    args.for_each(|item: String| { 
        println!("ARG PASSED: {:?}", item);
        args_copy.push(item)
    });

    println!("Amount of args passed: {}", args_copy.len());

    return args_copy
  }

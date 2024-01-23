use std::env::{Args, args};

pub fn fetch() -> Vec<String> {
    let args: Args = args();
    let mut args_copy: Vec<String> = vec![];

    args.for_each(|item: String| { 
        args_copy.push(item)
    });

    return args_copy
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_fetch() {
      let args: Vec<String> = fetch();

      // Number of default args passed is 1
      assert_eq!(args.len(), 1);
    }
  }

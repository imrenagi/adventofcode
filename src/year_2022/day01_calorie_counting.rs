use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;

pub fn calorie_counting(path: &str) -> io::Result<i32> {

  let buf_reader = BufReader::new(File::open(path)?);  
  let mut max = -1;
  let mut sum = 0;
  for res in buf_reader.lines() {
    let line = match res {
        Ok(l) => l, 
        Err(_) => panic!("error"),
    };
    if line == "" {    
      if sum > max {        
        max = sum;
      }
      sum = 0;

    } else {
      let val: i32 = line.parse().unwrap();
      sum += val;
    }    
  }
  if sum > max {
    max = sum
  }  
  Ok(max)
}
use std::ops::Range;

const INPUT_ALPHA: Range<i32> = 1..1001;

pub fn main() {
   
  let mut sum = 0;
  
  for number in INPUT_ALPHA {
    if number % 3 == 0 || number % 5 == 0 {
      sum+=number;
    }
  }
  
  println!("{sum}");
  
}

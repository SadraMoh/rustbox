pub fn main() {
  let mut prev = 1;
  let mut current = 2;

  let mut sum = current;
  
  loop {
      (prev, current) = (current, prev + current);

      if current % 2 == 0 {
        sum += current;
      }
      
      if current > 4_000_000 {
          break;
      }
  }
  
  println!("{sum}");
}

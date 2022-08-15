pub fn main() {
  let mut palindromes: Vec<i32> = Vec::new();

  for i in (900000..=999999).rev() {
      let res = i.to_string();

      if res == res.chars().rev().collect::<String>() {
          palindromes.push(i);
      }
  }

}

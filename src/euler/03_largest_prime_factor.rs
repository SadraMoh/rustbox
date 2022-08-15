static NUM: i64 = 600851475143;

pub fn main() {
    let mut factors: Vec<i64> = Vec::new();
    let mut num = NUM;
    let mut counter = 1;

    loop {
        if num % counter == 0 {
            factors.push(num);
            num = num / counter;
        }

        if counter > num {
            break;
        }

        counter += 1;
    }

    for factor in factors {
        if is_prime(factor) {
          println!("{factor}")
        }
    }
    
}

fn is_prime(i: i64) -> bool {

  for number in 1..i {
      if i % number == 0 {
        return false;
      }
  }

  return true
}
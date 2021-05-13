enum FizzBuzzValue {
  Fizz,
  Buzz,
  FizzBuzz,
  NotDivisible(i32),
}

#[derive(Debug)]
enum Error {
  GotNegative,
}

fn main() {
  print_fizzbuzz(-1);
}

fn print_fizzbuzz(x: i32) {
  match fizzbuzz(x).unwrap() {
    FizzBuzzValue::FizzBuzz => {
      println!("FizzBuzz");
    }
    FizzBuzzValue::Fizz => {
      println!("Fizz");
    }
    FizzBuzzValue::Buzz => {
      println!("Buzz");
    }
    FizzBuzzValue::NotDivisible(num) => {
      println!("{}", num);
    }
  }
}

fn fizzbuzz(x: i32) -> Result<FizzBuzzValue, Error> {
  if x < 0 {
    Err(Error::GotNegative)
  } else if x % 3 == 0 && x % 5 == 0 {
    Ok(FizzBuzzValue::FizzBuzz)
  } else if x % 3 == 0 {
    Ok(FizzBuzzValue::Fizz)
  } else if x % 5 == 0 {
    Ok(FizzBuzzValue::Buzz)
  } else {
    Ok(FizzBuzzValue::NotDivisible(x))
  }
}

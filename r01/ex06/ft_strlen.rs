use std::env;

fn ft_strlen(str: String) -> u32 {
  let mut counter: u32 = 0;

  for _char in str.chars() {
    counter += 1;
  }
  return counter;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let str: String = args[1].parse().unwrap();

  print!("{}", ft_strlen(str))
}
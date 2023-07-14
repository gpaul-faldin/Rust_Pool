use std::env;

fn ft_putchar(c: char) -> () {
  print!("{}", c);
}

fn ft_putstr(str: String) -> () {
  for _char in str.chars() {
    ft_putchar(_char);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  
  let str: String = args[1].parse().unwrap();

  ft_putstr(str);
}
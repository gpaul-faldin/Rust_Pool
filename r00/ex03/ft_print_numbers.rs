fn ft_print_numbers() -> () {
  let mut counter: u8 = 0;
  while counter < 10 {
    print!("{}", counter);
    counter += 1
  }
}

fn main() {
  ft_print_numbers()
}
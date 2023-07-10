fn ft_print_alphabet() -> () {
  let mut letter: u8 = 97;
  while letter < 123 {
    print!("{}", letter as char);
    letter += 1;
  }
}

fn main() {
  ft_print_alphabet();
}
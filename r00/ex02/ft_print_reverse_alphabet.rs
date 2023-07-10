fn ft_print_reverse_alphabet() -> () {
  let mut letter: u8 = 122;
  while letter > 96 {
    print!("{}", letter as char);
    letter -= 1;
  }
}

fn main() {
  ft_print_reverse_alphabet()
}
fn ft_is_negative(n: i16) -> () {
  if n < 0 {
    print!("N");
  }
  else {
    print!("P");
  }
}

fn main() {
  ft_is_negative(150);
  ft_is_negative(0);
  ft_is_negative(-150);
}
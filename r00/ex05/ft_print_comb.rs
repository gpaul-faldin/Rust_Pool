fn ft_print_nbr(first: u8, second: u8, last: u8) -> () {

  print!("{}{}{}", first, second, last);
  if first != 7 {
    print!(", ");
  }

}

fn ft_print_comb() -> () {

  let mut first: u8 = 0;
  let mut second: u8 = 1;
  let mut last: u8 = 2;

  while first < 8 {
    while second < 9 {
      while last < 10 {
        ft_print_nbr(first, second, last);
        last += 1;
      }
      second += 1;
      last = second + 1;
    }
    second = first + 1;
    first += 1;
  }
}

fn main() {
  ft_print_comb()
}
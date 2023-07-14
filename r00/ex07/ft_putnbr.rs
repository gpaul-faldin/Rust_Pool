fn ft_putchar(c: char) {
    print!("{}", c);
}

fn ft_putnbr(mut nb: i32) -> () {


  if nb == -2147483648 {
    nb = 147483648;
    ft_putchar('-');
    ft_putchar('2');
  }

  if nb < 0 {
    nb = -nb;
    ft_putchar('-');
  }
  if nb < 10 {
    ft_putchar((nb + 48) as u8 as char);
  } else {
    ft_putnbr(nb / 10);
    ft_putnbr(nb % 10);
  }
}

fn main () {
  ft_putnbr(-2147483648);
  println!();
  ft_putnbr(-45);
  println!();
  ft_putnbr(2147483647);
  println!();
  ft_putnbr(42);
  println!();
  ft_putnbr(0);
}
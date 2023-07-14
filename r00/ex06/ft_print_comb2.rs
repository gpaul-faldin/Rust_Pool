fn ft_putchar(c: char) {
    print!("{}", c);
}

fn ft_print_nbr(a: u8, b: u8, c: u8, d: u8) {
    ft_putchar((a + 48) as char);
    ft_putchar((b + 48) as char);
    ft_putchar(' ');
    ft_putchar((c + 48) as char);
    ft_putchar((d + 48) as char);
    if a != 9 || b != 8 || c != 9 || d != 9 {
      print!(", ");
    }
}

fn ft_print_comb2() {

    for a in 0..=9 {
        for b in 0..=9 {
            for c in a..=9 {
                for d in if a == c { b + 1 } else { 0 }..=9 {
                    ft_print_nbr(a, b, c, d);
                }
            }
        }
    }
}

fn main() {
  ft_print_comb2();
}
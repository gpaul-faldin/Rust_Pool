use std::env;

fn ft_rev_in_tab(tab: &mut [i32], mut size: usize) -> () {

  let mut i: usize = 0;
  let mut tmp: i32;

  size -= 1;
  while i < size {
    tmp = tab[size];
    tab[size] = tab[i];
    tab[i] = tmp;
    i += 1;
    size -= 1;
  }
}

fn main() {
  let mut tab: [i32; 5] = [1, 2, 3, 4, 5];
  let mut size: usize = tab.len();

  for n in tab {
    print!("{}", n);
  }
  println!();

  ft_rev_in_tab(&mut tab, size);

  for n in tab {
    print!("{}", n);
  }

}
use std::env;

fn  ft_utltimate_div_mod(a: *mut i32, b: *mut i32) -> () {

  let tmp: i32;
  unsafe {
    tmp = *a / *b;
    *b = *a % *b;
    *a = tmp;
  }
}

fn  main() {

  let args: Vec<String> = env::args().collect();

  let mut a: i32 = args[1].parse().unwrap();
  let mut b: i32 = args[2].parse().unwrap();
  let a_ptr: *mut i32 = &mut a;
  let b_ptr: *mut i32 = &mut b;

  ft_div_mod(a_ptr, b_ptr);

  unsafe {
  println!("{}  {}", *a_ptr, *b_ptr);
  }

}
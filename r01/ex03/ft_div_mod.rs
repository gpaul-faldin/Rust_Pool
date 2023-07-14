use std::env;

fn  ft_div_mod(a: i32, b: i32, div: *mut i32, modulo: *mut i32) -> () {

  unsafe {
    if div.is_null() == false && modulo.is_null() == false {
      *div = a / b;
      *modulo = a % b;
    } else {
      println!("Error: Div or modulo is null");
    }
  }
}

fn  main() {

  let args: Vec<String> = env::args().collect();

  let a: i32 = unsafe { args[1].parse().unwrap() };
  let b: i32 = unsafe { args[2].parse().unwrap() };
  let mut div: i32 = 0;
  let div_ptr: *mut i32 = &mut div;
  let mut modulo: i32 = 0;
  let modulo_ptr: *mut i32 = &mut modulo;

  ft_div_mod(a, b, div_ptr, modulo_ptr);

  unsafe {
  println!("{}  {}", *div_ptr, *modulo_ptr);
  }

}
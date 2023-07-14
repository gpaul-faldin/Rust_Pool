fn ft_swap(a: *mut i32, b: *mut i32) -> () {

  let c: i32;

  unsafe {
    c = *a;
    *a = *b;
    *b = c;
  }

}

fn main() {
  let mut a: i32 = 42;
  let a_ptr: *mut i32 = &mut a;
  let mut b: i32 = -42;
  let b_ptr: *mut i32 = &mut b;

  unsafe {
    println!("{}", *a_ptr);
    println!("{}", *b_ptr);
  }

  ft_swap(a_ptr, b_ptr);

  unsafe {
    println!("{}", *a_ptr);
    println!("{}", *b_ptr);
  }
}
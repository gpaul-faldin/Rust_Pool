
fn ft_ft(nbr: *mut u32) -> () {

  unsafe {
    if nbr.is_null() == false {
      *nbr = 42;
    } else {
      println!("Error: The pointer is null");
    }
  }


}

fn main() {

  let mut value: u32 = 35;
  let ptr_value: *mut u32 = &mut value;

  ft_ft (ptr_value);
}
fn main() {
  let ret = foo(false);
  println!("{:?}\n", ret);

  let r2 = foo(true);
  println!("{:?}\n", r2);
  
}

fn foo(arg: bool) -> Option<i32> {
  let ret = 11;
  if arg {
    Some(ret)
  } else {
    None
  }
}


      

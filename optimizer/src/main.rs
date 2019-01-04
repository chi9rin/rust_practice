pub fn hoge(i: i32) -> i32 {
   let hoge_r = 0;
   hoge_r
}

fn foo(i: i32) -> i32 {
   let foo_r = i;
   foo_r
}

fn main() {
  println!("{}", hoge(100));
  println!("{}", foo(100));
  println!("{}", foo(101));
  
}

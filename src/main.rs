fn main() {
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  let v2 = vec![1, 2, 3, 4];

  let third: &i32 = &v[2];
  println!("The third element (of first vector) is {}", third);

  match v.get(2) {
    Some(third) => println!("The third element (of first vector) is {}", third),
    None => println!("There is no third element."),
  }

  // let does_not_exist = &v[100]; // will panic!
  // let does_not_exist = v.get(100); // will not panic but give None

  let sec: &i32 = &v2[1];
  println!("Second (of second vector) is: {}", sec);

  print_divider();
  let v = vec![100, 32, 57];
  for i in &v {
    println!("{}", i);
  }
  print_divider();
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }
  for i in &v {
    println!("{}", i);
  }

}

fn print_divider() {
  println!("{}", "-".repeat(50));
}

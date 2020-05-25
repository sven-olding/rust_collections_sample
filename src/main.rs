use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

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
  print_divider();

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  for cell in &row {
    println!("{:?}", cell);
  }
  print_divider();

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);
  print_divider();

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
  print_divider();

  exercise1();
  print_divider();
}

fn print_divider() {
  println!("{}", "-".repeat(50));
}

/*
Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position),
and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/
fn exercise1() {
  let mut list = vec![5, 17, 88, 42, 5, 14, 5, 17, 5, 120];

  list.sort();
  println!("list: {:?}", list);

  println!("Average: {}", mean(&list));

  println!("Median: {}", median(&list));

  println!("Mode: {}", mode(&list));
}

fn mean(list: &[i32]) -> f64 {
  let mut avg = 0;
  let len = list.len();
  for i in list {
    avg += i;
  }
  f64::from(avg) / len as f64
}

fn median(list: &[i32]) -> i32 {
  let len = list.len();
  let mid = ((len as f64 - 1.0) / 2.0).round() as usize;
  println!("{}", mid);
  list[mid]
}

fn mode(list: &[i32]) -> i32 {
  let mut map = HashMap::new();
  let mut max = (0, 0);

  for v in list {
    let count = map.entry(v).or_insert(0);
    *count += 1;
  }

  for (k, v) in map.iter() {
    if v > &max.1 {
      max = (**k, *v);
    }    
  }
  max.0
}

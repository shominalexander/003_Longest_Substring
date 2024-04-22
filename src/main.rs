fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 loop {
  println!("\n\nstring:"); 

  let string: String = request();

  if &string[..] == "exit" {
   break;   

  } else {//if &string[..] == "exit" {
   let     chars  : Vec<char> = string[..].chars().collect();
   let mut current: Vec<char> = Vec::new()                  ;
   let     length : usize     = string.len()                ;
   let mut longest: Vec<char> = Vec::new()                  ;

   for i in 0..length {
    let rest: usize = length - i;

    for j in i..length {
     let char = chars[j];

     if current.iter().position(|&x| x == char).is_some() {
      break;

     } else {//if current.iter().position(|&x| x == char).is_some() {
      current.push(char);

     }//} else {//if current.iter().position(|&x| x == char).is_some() {
    }//for j in i..length {

    if current.len() > longest.len() {
     longest = current.clone();

    }//if current.len() > longest.len() {

    current.clear();

    if longest.len() == rest {
     break;

    }//if longest.len() == rest {
   }//for i in 0..length {

   println!("\nlongest: {:?}", longest.len());
  }//} else {//if &string[..] == "exit" {
 }//loop {
}//fn main() {

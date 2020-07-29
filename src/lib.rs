use wasm_bindgen::prelude::*;

#[wasm_bindgen]
//pub fn say(s: &str) -> String {
//  println!("The Rust function say() received {}", s);
//  let r = String::from("hello ");
//  return r + s;
//}


pub fn say(a: i32, i: i32, p: i32) -> String {
  let r = String::from("Interest calculation is : ");
  let mut calc = 0;
  let mut aa = a;
  for x in (0..p).enumerate() {
    calc =(aa*i/100) + aa;
    aa = calc;
}
let r = String::from("Interest calculation is : ");
let result = format!("{} {}", r, calc);
//r=r+&calc.to_string();
  return result;
}

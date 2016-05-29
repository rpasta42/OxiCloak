
//gentypes
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedMut<T> = Rc<RefCell<T>>;

pub fn to_shared_mut<T>(x : T) -> SharedMut<T> {
   Rc::new(RefCell::new(x))
}

pub type SizeRange = (usize, usize);
pub type SizeRanges = Vec<SizeRange>;

//genutils
pub fn contains<T : PartialEq>(item : T, vec : Vec<T>) -> bool {
   for x in vec {
      if x == item { return true; }
   } false
}

pub fn vec_eq<T: PartialEq>(v1 : &Vec<T>, v2 : &Vec<T>) -> bool {
   if v1.len() != v2.len() { return false; }
   for (x, y) in v1.iter().zip(v2.iter()) {
      if *x != *y { return false; }
   } true
}

pub fn is_number_like(s : &str) -> bool {
   //Checks if first char is numeric. The rest of number might
   //be badly formatted (like 032d):
   //let badly_formatted = is_number_like(s) && !is_int(s) && !is_float(s)
   s.len() > 0 && char_at_fast(s, 0).is_digit(10)
}

pub fn is_int(s : &str) -> bool {
   let mut i = 0;
   while i < s.len() {
      let c = char_at(s, i).unwrap();
      let first_is_minus = i == 0 && c == '-' && s.len() > 1;
      if !c.is_digit(10) && !first_is_minus { return false; }
      i += 1;
   } true
}

pub fn is_float(s : &str) -> bool {
   //"." = false, "0.3" = true, "0.3." = false
   let mut i = 0;
   let mut have_dot = false;
   let good_len = s.len() > 1;

   while i < s.len() {
      let c = char_at_fast(s, i);
      let first_is_minus = i == 0 && c == '-' && good_len;
      if !c.is_digit(10) && !first_is_minus {
         let got_first_dot = c == '.' && !have_dot && good_len;
         if got_first_dot { have_dot = true; }
         else { return false; }
      }
      i += 1;
   } true
}

pub fn s_to_f64(s : &str) -> f64 { s.parse::<f64>().unwrap() }
pub fn s_to_f32(s : &str) -> f32 { s.parse::<f32>().unwrap() }
pub fn s_to_i64(s : &str) -> i64 { s.parse::<i64>().unwrap() }
pub fn s_to_usize(s : &str) -> usize { s.parse::<usize>().unwrap() }

use std::str::FromStr;
pub fn from_str<T : FromStr>(s : &str)
-> Result<T, <T as FromStr>::Err>
{ s.parse::<T>() }

//slice_str("hello", 1, 3) => "ell"
pub fn slice_str(s: &str, start: usize, end: usize) -> String {
   /*let mut sub: String = String::new();
   let mut started: bool = false;

   if start >= end { panic!("slice_str: start>=end"); }
   if end >= s.len() {  panic!("slice_str: end >= string end"); }

   for (i, c) in s.chars().enumerate() {
      if i >= end+1 { return &sub; }
      if started { sub.push(c); continue; }
      if i >= start { started = true; sub.push(c); }
   }
   &sub*/
   (&s[start..end+1]).to_string()
}

pub fn print_spaces(n: u8) {
   let mut i = 0;
   while i < n { print!(" "); i += 1; }
}

pub fn char_at(str : &str, n : usize) -> Option<char> {
   for (i, c) in str.chars().enumerate() {
      if i == n { return Some(c); }
   } None
}

pub fn char_at_fast(str : &str, n : usize) -> char {
   for (i, c) in str.chars().enumerate() {
      if i == n { return c; }
   } '\0'
}

pub fn is_none<T>(x : Option<T>) -> bool {
   if let Some(_) = x { false } else { true }
}

pub fn is_ok<T, Y>(x : Result<T, Y>) -> bool {
   if let Ok(_) = x { true } else { false }
}

//[0, 1, Some(0.1)] => [0, 0.1, 0.2, ..., 1]
//[0, 5, None] => [0, 1, 2, ..., 5]
//[5, 2, -2] => [5, 3]
#[allow(unused_variables)]
pub fn range<T>(start : T, end : T, step : Option<T>) -> Vec<T> {
   Vec::new()
}

pub fn float_range(start : f32, step : f32, end : f32) -> Vec<f32> {
   let mut i = start;
   let mut ret = Vec::new();
   while i < end {
      ret.push(i);
      i += step;
   }
   ret
}

//panic!("{}", Error::description(&why))
pub fn read_file(path_str : &str) -> Result<String, std::io::Error> {
   use std::io::prelude::*;
   use std::fs::File;
   use std::path::Path;
   //use std::error::Error;

   //println!("loading file {}", path_str);
   let path = Path::new(path_str);
   let file_res = File::open(&path);
   if let Err(why) = file_res { return Err(why); }

   let mut file = file_res.unwrap();
   let mut content = String::new();

   match file.read_to_string(&mut content) {
      Ok(_) => Ok(content.to_string()),
      Err(why) => Err(why)
   }
}

//https://doc.rust-lang.org/std/io/trait.Read.html
pub fn read_bin_file(f_path : &str) -> Result<Vec<u8>, std::io::Error> {
   use std::fs::File;
   use std::io::Read;

   let file_opt = File::open(f_path);
   if let Err(why) = file_opt { return Err(why); }
   let mut file = file_opt.unwrap();

   let mut ret = Vec::new();
   match file.read_to_end(&mut ret) {
      Ok(_) => Ok(ret),
      Err(why) => Err(why)
   }
}


pub fn pow(n : f64, p : u8) -> f64 {
   let mut ret = 1.0;
   for _ in 0..p {
      ret *= n;
   } ret
}

pub fn round(n : f64, precision : u8) -> f64 {
   let p = pow(10.0, precision);
   (n * p).round() / p
}







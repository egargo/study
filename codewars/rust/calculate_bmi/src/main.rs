// https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust


fn bmi(weight: u32, height: f32) -> &'static str {
  let bmi = weight as f32 / f32::powf(height, 2.0);

  if bmi <= 18.5 {
  "Underweight"
  } else if bmi <= 25.0 {
    "Normal"
  } else if bmi <= 30.0 {
    "Overweight"
  } else {
    "Obese"
  }
}

fn main() {
  println!("{:?}", bmi(50, 1.80));
}

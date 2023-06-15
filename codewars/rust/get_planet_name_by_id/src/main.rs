// https://www.codewars.com/kata/515e188a311df01cba000003/train/rust


fn get_planet_name(id: u32) -> String {
    match id {
        1 => return "Mercury".to_string(),
        2 => return "Venus".to_string(),
        3 => return "Earth".to_string(),
        4 => return "Mars".to_string(),
        5 => return "Jupiter".to_string(),
        6 => return "Saturn".to_string(),
        7 => return "Uranus".to_string(),
        8 => return "Neptune".to_string(),
        _ =>  panic!(),
    }
}

fn main() {
    println!("{}", get_planet_name(3));
}

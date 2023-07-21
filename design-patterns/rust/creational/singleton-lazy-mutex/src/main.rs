mod countries;
mod operatingsys;

fn main() {
    let mut country = countries::Countries::instance().lock().unwrap();

    country.country_name(String::from("Japan"), String::from("Tokyo"), 1);
    println!("Japan: {:?}", country.get_country());

    let mut os = operatingsys::OperatingSystems::instance().lock().unwrap();

    os.os_name(
        String::from("GNU/Linux"),
        String::from("Linux"),
        String::from("Open-source"),
    );
    println!("Linux: {:?}", os.get_os());
}

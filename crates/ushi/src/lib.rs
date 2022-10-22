pub mod database;
pub mod resin;

pub fn make_kabab_case(string: &str) -> String {
    let new_string = string.replace(" - ", "-");
    new_string.replace(" ", "-")
}

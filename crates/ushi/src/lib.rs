pub mod database;
pub mod resin;

pub fn make_kabab_case(string: &str) -> String {
    string.replace(" ", "-")
}

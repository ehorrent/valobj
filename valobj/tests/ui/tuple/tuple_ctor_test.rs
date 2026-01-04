#[valobj::value_object]
pub struct UserId(u64);

fn main() {
    let value = UserId(0);
}

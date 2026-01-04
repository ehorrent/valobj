#[valobj::value_object]
pub struct UserId(u64);

fn main() {
    let user_id = UserId::from(1);
    let _ = user_id.0;
}

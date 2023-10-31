use rand::Rng;

// return dummy data number of (name, age, uuid) pair
fn get_user() -> (String, u8, u128) {
    let names = ["john", "jane", "james"];
    let name_index = rand::thread_rng().gen_range(0..names.len());
    let name = names[name_index].to_string();

    let age = rand::thread_rng().gen_range(10..101);
    let uuid = 1234567890;

    return (name, age, uuid);
}

pub fn get_users(num: i32) -> Vec<(String, u8, u128)> {
    let mut users = Vec::new();
    for _ in 0..num {
        users.push(get_user());
    }
    return users;
}

fn main() {
    let s1 = String::from("30");
    let i = 23;
    let j: u32 = 34;
    caveman(&s1);
    caveman(&i);
    caveman(&j);
}

fn caveman<T>(exp: &T) -> &T {
    let exp_type = std::any::type_name::<T>().to_string();
    println!("{}", exp_type);
    if exp_type == "i32" {
        exp
    } else if exp_type == "alloc::string::String" {
        exp
    } else if exp_type == "u32" {
        exp
    } 
    else {
        panic!("༼ ºل͟º  ༽つ ಥ_ಥ");
    }
}
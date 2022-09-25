fn main() {
    let fahrenheit = 95;
    let celsius = convert_to_celsius(fahrenheit);
    let celsius = celsius.to_string();
    println!("celsius: {celsius}");
}

// 華氏を摂氏に変換
fn convert_to_celsius(fahrenheit: i32) -> i32 {
    return ((fahrenheit - 30) / 2) as i32
}
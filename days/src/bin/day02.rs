pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message); // Use `&` instead of `.clone()`.

    println!("{}", gift_message);
}

pub fn attach_message_to_present(message: &str) { // Use `&str` instead of `String`.
    println!("The present now has this message: {}", message);
}

# Day 1

It's 1st December and the countdown has just begun. The elves are busy preparing for the Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of santa's elves burst into Santa's office with a problem.

"Santa!" one of the elves shouted. "The code won't compile! We've hit a wall, and it's all Rust's fault!"

Santa, sipping his triple-shot peppermint latte, raised an eyebrow. "Rust's fault? Or your fault?"

"It's the ownership rules!" the other elve blurted. "I think we violated them, we're used to Python, where [variables](https://www.rustfinity.com/learn/rust/the-programming-basics/variables) just... work. Look at this!"

The elves tried their best, here is what they've written so far:

```rust
fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message);
 
    println!("{}", gift_message);
}
 
fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
```

However, the code won't compile. Can you help the elves attach the message to the present and print the message too?

Maybe... if there was only a way to get a clone of the message and pass it to the function. 🤔

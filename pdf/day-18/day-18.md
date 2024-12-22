---
marp: true
theme: uncover
class: invert
paginate: true
transition: cube
style: |
    * {
        font-family: "JetBrains Mono";
    }
---

# Day 18

###### Sleigh builder pattern

---

## The Story

The elves were huddled at a desk, deep in debate over their Neovim setups.

"I finally nailed my `init.lua`," said Sparky. "Switched to `bspwm`, too. Total game-changer."

"Pfft," Tinker scoffed. "Real pros stick to Vimscript. You don't need all those plugins."

---

## The Story

Jingle sipped his eggnog. "I wrote a whole game in Vim once. Problem? Couldn't press two keys at the same time. Now it's turn-based."

Before they could laugh, the door slammed open. Santa stormed in once again. "Do I pay you to yap about text editors?"

---

## The Story

The elves froze as Santa tossed a crumpled piece of code onto the table. "The sleigh builder is completely broken! I just tried to build a sleigh, and it's missing half the parts. Magical enhancements? Gone. Gift capacity? Zero. Someone explain this!"

---

## The Story

Sparky glanced at the code and gulped. "This... uh... looks like something from that old project nobody touched."

"Exactly!" Santa growled. "And now it's ruining everything. We're rewriting this in Rust. Get to work, before I replace you all with AI!"

---

## Your Mission

To help Santa build his new sleigh easily, we need to create him a `SleighBuilder` that can build and return `Sleigh` instances.

The `SleighBuilder` should have:

- An [associated function](https://www.rustfinity.com/learn/rust/structs/implementing-methods#associated-functions) `new` that creates a new `SleighBuilder` instance.
- `red`, `reindeer-powered`, `100`, and `false` are the default values for the sleigh.

---

## Your Mission

- A method named `color` that takes a `&str` and sets the color of the sleigh.
- A method named `engine` that takes a `&str` and sets the engine of the sleigh.
- A method named `gift_capacity` that accepts a `u32` and sets the gift capacity of the sleigh.

---

## Your Mission

- A method named `magical_enhancements` that sets the magical enhancements of the sleigh.
- A final method that is going to return a `Sleigh` instance called `build`.

Make sure that each method takes [ownership](https://www.rustfinity.com/learn/rust/ownership) of the `SleighBuilder` instance and returns it after mutation.

Have a look at the end of the file to see how Santa wants to use this API.

---

## Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Store the settings in the `SleighBuilder` struct and return a `Sleigh` instance when the `build` method is called.

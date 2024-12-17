use std::fmt;

// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

// 2. Finish the trait definition
pub trait Gift {
    fn wrap(&mut self);
}

// 3. Update the function signature
pub fn prepare_gift<T: Gift + fmt::Display>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

// 4. Implement the Gift trait for the gift structs
impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

// Implement Display for the gift structs
impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}

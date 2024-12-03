// We need to find the nice and naughty kids for santa
// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / (10 * 1) + (2 * 2) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty
    if good_deeds == 0 && bad_deeds == 0 {
        false
    } else {
        let (good_deeds, bad_deeds) = (good_deeds as f32, bad_deeds as f32);
        good_deeds * GOOD_WEIGHT / (good_deeds * GOOD_WEIGHT + bad_deeds * BAD_WEIGHT) >= 0.75
    }
}

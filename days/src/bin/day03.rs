// We need to find the nice and naughty kids for santa
// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }

    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
    let ratio = good_deeds / (good_deeds + bad_deeds);

    ratio >= 0.75
}

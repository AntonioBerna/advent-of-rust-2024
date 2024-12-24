// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Begin Solution

// Define the trait Anonymize
pub trait Anonymize {
    fn anonymize_email(&self) -> String;
}

// Implement the trait for String
impl Anonymize for String {
    fn anonymize_email(&self) -> String {
        // Split the email into local and domain parts
        if let Some(at_index) = self.find('@') {
            let (local_part, domain_part) = self.split_at(at_index);

            // Generate anonymized local part using Christmas emojis
            let anonymized_local: String = local_part
                .chars()
                .enumerate()
                .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                .collect();

            // Recombine the anonymized local part with the domain
            format!("{}{}", anonymized_local, domain_part)
        } else {
            // If the email is invalid, replace every character with emojis
            self.chars()
                .enumerate()
                .map(|(i, _)| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()])
                .collect()
        }
    }
}

// End Solution

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}

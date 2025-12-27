// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Your Solution here ...

pub trait Anonymize {
    fn anonymize_email(&self) -> String;
}

impl Anonymize for String {
    fn anonymize_email(&self) -> String {
        let (local, domain) = self.split_once('@').map_or((self.as_str(), ""), |(l, d)| (l, d));

        let local_char_count = local.chars().count();
        let domain_len = if domain.is_empty() { 0 } else { domain.len() + 1 };
        
        let mut anonymized = String::with_capacity(local_char_count * 4 + domain_len);

        CHRISTMAS_EMOJIS.iter()
            .cycle()
            .take(local_char_count)
            .for_each(|&emoji| anonymized.push(emoji));

        if !domain.is_empty() {
            anonymized.push('@');
            anonymized.push_str(domain);
        }

        anonymized
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymize_email() {
        let email = "santa@north.pole".to_string();
        assert_eq!(email.anonymize_email(), "🎅🤶🎄🎁🎅@north.pole");

       let email = "santa".to_string();
        assert_eq!(email.anonymize_email(), "🎅🤶🎄🎁🎅")
    }
}

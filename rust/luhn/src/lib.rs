/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();

    if code.is_empty() || code == "0" {
        return false;
    }

    let mut sum = 0;
    for (i, c) in code.chars().filter(|c| *c != ' ').rev().enumerate() {
        let Some(v) = c.to_digit(10) else {
            return false;
        };

        let d = if i != 0 && i % 2 != 0 {
            if v * 2 > 9 { v * 2 - 9 } else { v * 2 }
        } else {
            v
        };

        sum += d;
    }

    sum % 10 == 0
}

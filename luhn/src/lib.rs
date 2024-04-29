/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits = code.chars().try_fold(Vec::new(), |mut acc, c| {
        if c.is_ascii_digit() {
            acc.push(c as u8);
            return Ok(acc);
        }

        if !c.is_whitespace() {
            return Err("non digit char");
        }

        Ok(acc)
    });

    if digits.is_err() {
        return false;
    }

    let digits = digits.unwrap();

    if digits.len() < 2 {
        return false;
    }

    let sum: i32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| {
            let counter = i + 1;
            let digit = (*digit - b'0') as i32;

            if counter % 2 == 0 {
                let mut res = digit * 2;
                if res > 9 {
                    res -= 9;
                }
                return res;
            }

            digit
        })
        .sum();

    if sum % 10 == 0 {
        return true;
    }

    false
}

use validator::ValidationError;

const ALLOWED_SPECIALS: &str = "!@#$%^&*()_+-=[]{}";

pub fn password_complexity(raw_password: &str) -> Result<(), ValidationError> {
    let special_chars = ALLOWED_SPECIALS.chars().collect::<Vec<_>>();
    let mut upper = 0;
    let mut lower = 0;
    let mut special = 0;
    let mut digits = 0;

    for ch in raw_password.chars() {
        if ch.is_alphabetic() && ch.is_lowercase() {
            lower += 1;
            continue;
        }

        if ch.is_alphabetic() && ch.is_uppercase() {
            upper += 1;
            continue;
        }

        if ch.is_numeric() {
            digits += 1;
            continue;
        }

        if special_chars.contains(&ch) {
            special += 1;
        }
    }

    if upper >= 2 && lower >= 2 && special >= 2 && digits >= 2 {
        return Ok(());
    }

    Err(ValidationError::new("password")
        .with_message("password does not meet the complexity requirements".into()))
}

pub fn get_date_from_hash(token: &str) -> String {
    if token.len() != 72 {
        return "Invalid".to_string();
    }

    let day = &token[0..2];
    let year = &token[66..70];
    let month = &token[70..72];

    format!("{}-{}-{}", year, month, day)
}

pub fn get_original_hash(token: &str) -> String {
    if token.len() != 72 {
        return "Invalid".to_string();
    }

    token[2..66].to_string()
}

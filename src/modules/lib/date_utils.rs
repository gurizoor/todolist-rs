use chrono::Local;

/// Get current date as formatted string
pub fn get_current_date() -> String {
    Local::now().format("%Y-%m-%d-%u").to_string()
}

/// Parse date string into components
pub fn parse_date(date_str: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    date_str
        .split("-")
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

/// Check if day has changed between two dates
pub fn has_day_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let current_parts = parse_date(current)?;
    let previous_parts = parse_date(previous)?;
    
    Ok(current_parts[2] != previous_parts[2]
        || current_parts[1] != previous_parts[1]
        || current_parts[0] != previous_parts[0])
}

/// Check if week has changed between two dates
pub fn has_week_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let current_parts = parse_date(current)?;
    let previous_parts = parse_date(previous)?;
    
    Ok(current_parts[0] != previous_parts[0]
        || (current_parts[3] <= previous_parts[3]
            && (current_parts[2] != previous_parts[2]
                || current_parts[1] != previous_parts[1]))
        || previous_parts[2] + 7 <= current_parts[2])
}

/// Check if month has changed between two dates
pub fn has_month_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let current_parts = parse_date(current)?;
    let previous_parts = parse_date(previous)?;
    
    Ok(current_parts[0] != previous_parts[0]
        || current_parts[1] != previous_parts[1])
}

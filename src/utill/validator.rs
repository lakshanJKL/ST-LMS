use regex::Regex;
use uuid::Uuid;
use validator::ValidationError;
//pub type Result<T> = std::result::Result<T,UserServiceError>;

// pub  fn validate<T: Validate>(data: &T) -> Result<()> {
//     match data.validate() {
//         Ok(_) => Ok(()),
//         Err(e) => {
//             let errors = e
//                 .field_errors()
//                 .iter()
//                 .map(|(field, errors)| {
//                     let messages: Vec<Cow<str>> = errors
//                         .iter()
//                         .map(|err| err.message.clone().unwrap_or_else(|| Cow::Borrowed("Invalid value")))
//                         .collect();
//                     format!("{}: {}", field, messages.join(", "))
//                 })
//                 .collect::<Vec<String>>()
//                 .join("; ");
//             Err(UserServiceError::ValidationError(errors))
//         }
//     }
// }

// custom text validation
pub fn custom_text_check(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Ok(());
    }

    let regex_value = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    if regex_value.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid search text: Only letters, spaces are allowed"))
    }
}

// custom password validation
pub fn custom_uuid_check(value: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(value).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid UUID format"))
    }
}
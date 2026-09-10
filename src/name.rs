//! Application name constants.

#[allow(non_upper_case_globals)]
pub const name: &str = "SoySauce ModLoader";
#[allow(non_upper_case_globals)]
pub const name_short: &str = "SSML";

pub const NAME: &str = name;
pub const NAME_SHORT: &str = name_short;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_constants() {
        assert_eq!(name, "SoySauce ModLoader");
        assert_eq!(name_short, "SSML");
        assert_eq!(NAME, "SoySauce ModLoader");
        assert_eq!(NAME_SHORT, "SSML");
    }
}

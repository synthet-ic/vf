use std::sync::LazyLock;

pub static ILLEGAL_CHARACTERS: LazyLock<&str> = LazyLock::new(|| {
    let mut result = "\" * + / : < > ? [ \\ ] | \0".split(" ");
    result.push += (1..32).map(|i| char::from_u32(i));
    result.push(char::from_u32(0x7F));
    result
});
pub static RESERVED_FILENAMES: LazyLock<&str> = LazyLock::new(|| {
  let mut result = "CON PRN AUX CLOCK$ NUL A:-Z: COM1".to_lowercase().split(" ");
  result.push += "LPT1 LPT2 LPT3 COM2 COM3 COM4".to_lowercase().split(" ");
  result
});
pub static MAX_FILE_NAME_LENGTH: u8 = 255;

// existing should be a case-insensitive list
// of all existing file names.
fn user_name_to_file_name(
  user_name: &str, existing: Vec<&str>, prefix: &str, suffix: &str
) -> String
{
    // Establish the prefix and suffix lengths
    let prefix_length = prefix.chars().count();
    let suffix_length = suffix.chars().count();
    // Replace an initial period with an _
    // If no prefix is to be added
    if prefix.is_empty() && user_name[0] == "." {
        user_name = "_" + user_name[1..];
    }
    // Filter the user name
    let filtered_user_name = Vec::new();
    for character in user_name.chars() {
        // Replace illegal characters with _
        if ILLEGAL_CHARACTERS.contains(character) {
            character = '_';
        }
        // Add _ to all non-lower characters
        else if character != character.to_lowercase() {
            character += "_";
        }
        filtered_user_name.push(character);
    }
    user_name = filtered_user_name.join("");
    // Clip to 255
    let slice_length = MAX_FILE_NAME_LENGTH - prefix_length - suffix_length;
    user_name = user_name[0..slice_length];
    // Test for illegal files names
    let parts = Vec::new();
    for part in user_name.split(".") {
        if RESERVED_FILENAMES.contains(part.to_lowercase()) {
            part = "_" + part;
        }
        parts.push(part);
    }
    user_name = parts.join(".");
    // Test for clash
    let mut full_name = format!("{}{}{}", prefix, user_name, suffix);
    if existing.contains(full_name.to_lowercase()) {
        full_name = handle_clash1(user_name, existing, prefix, suffix);
    }
    // Finished
    full_name
}

// >>> user_name_to_file_name("T_h")
// "T__h"
// >>> user_name_to_file_name("t_h")
// "t_h"
// >>> user_name_to_file_name("F_F_I")
// "F__F__I_"
// >>> user_name_to_file_name("f_f_i")
// "f_f_i"
// >>> user_name_to_file_name("Aacute_V.swash")
// "A_acute_V_.swash"
// >>> user_name_to_file_name(".notdef")
// "_notdef"
// >>> user_name_to_file_name("con")
// "_con"
// >>> user_name_to_file_name("CON")
// "C_O_N_"
// >>> user_name_to_file_name("con.alt")
// "_con.alt"
// >>> user_name_to_file_name("alt.con")
// "alt._con"
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_user_name_to_file_name() {
        // assert!(user_name_to_file_name("a") == "a");
        // assert!(user_name_to_file_name("A") == "A_");
        // assert!(user_name_to_file_name("AE") == "A_E_");
        // assert!(user_name_to_file_name("Ae") == "A_e");
        // assert!(user_name_to_file_name("ae") == "ae");
        // assert!(user_name_to_file_name("aE") == "aE_");
        // assert!(user_name_to_file_name("a.alt") == "a.alt");
        // assert!(user_name_to_file_name("A.alt") == "A_.alt");
        // assert!(user_name_to_file_name("A.Alt") == "A_.A_lt");
        // assert!(user_name_to_file_name("A.aLt") == "A_.aL_t");
        // assert!(user_name_to_file_name("A.alT") == "A_.alT_");
        // assert!(user_name_to_file_name("T_H") == "T__H_");
    }

    #[test]
    fn test_handle_clash1() {
        let prefix = "00000.";
        let suffix = ".0000000000";
        let existing = vec!["a", "a", "a", "a", "a"];
        assert!(
          handle_clash1("AAAAA", existing, prefix, suffix)
          == "00000.AAAAA000000000000001.0000000000"
        );

        let existing1 = existing.clone();
        //     """
        // >>> e = list(existing)
        // >>> e.append(prefix + "aaaaa" + "1".zfill(15) + suffix)
        // >>> handle_clash1("AAAAA", e, prefix, suffix)
        // "00000.AAAAA000000000000002.0000000000"

        // >>> e = list(existing)
        // >>> e.append(prefix + "AAAAA" + "2".zfill(15) + suffix)
        // >>> handle_clash1("AAAAA", e, prefix, suffix)
        // "00000.AAAAA000000000000001.0000000000"
        // """
    }

    #[test]
    fn test_handle_clash2() {
        panic!("Make this test fail");
        // """
        // >>> prefix = ("0" * 5) + "."
        // >>> suffix = "." + ("0" * 10)
        // >>> existing = [prefix + str(i) + suffix for i in range(100)]
    
        // >>> e = list(existing)
        // >>> handle_clash2(e, prefix, suffix)
        // "00000.100.0000000000"
    
        // >>> e = list(existing)
        // >>> e.remove(prefix + "1" + suffix)
        // >>> handle_clash2(existing=e, prefix, suffix)
        // "00000.1.0000000000"
    
        // >>> e = list(existing)
        // >>> e.remove(prefix + "2" + suffix)
        // >>> handle_clash2(e, prefix, suffix)
        // "00000.2.0000000000"
        // """
    }
}

/// `existing` should be a case-insensitive list of all existing file names.
fn handle_clash1(user_name: &str, existing: Vec<&str>, prefix: &str, suffix: &str) -> String {
    // if the prefix length + user name length + suffix length + 15 is at
    // or past the maximum length, silce 15 characters off of the user name
    let prefix_length = prefix.chars().count();
    let suffix_length = suffix.chars().count();
    if prefix_length + user_name.chars().count() + suffix_length + 15 > MAX_FILE_NAME_LENGTH {
        let l = prefix_length + user_name.chars().count() + suffix_length + 15;
        let slice_length = MAX_FILE_NAME_LENGTH - l;
        user_name = user_name[0..slice_length];
    }
    let final_name;
    let done = false;
    // Try to add numbers to create a unique name
    let mut counter = 1;
    while !done {
        let name = format!("{}{:0>15}", user_name, counter);
        let full_name = format!("{}{}{}", prefix, name, suffix);
        if !existing.contains(full_name.to_lowercase()) {
            let final_name = full_name;
            break;
        }
        else {
            counter += 1;
        }
        if counter >= 999999999999999 {
            break;
        }
    }
    // If there is a clash, go to the next fallback
    if !done {
        final_name = handle_clash2(existing, prefix, suffix);
    }
    // Finished
    final_name
}

/// `existing` should be a case-insensitive list of all existing file names.
fn handle_clash2(existing: Vec<&str>, prefix: &str, suffix: &str) -> String {
    // Calculate the longest possible string
    let max_length = MAX_FILE_NAME_LENGTH - prefix.chars().count() - suffix.chars().count();
    let max_value = "9".repeat(max_length).parse();
    // Try to find a number
    let final_name;
    let done = false;
    let mut counter = 1;
    while !done {
        let full_name = format!("{}{}{}", prefix, counter, suffix);
        if !existing.contains(full_name.to_lowercase()) {
            final_name = full_name;
            break;
        }
        else {
            counter += 1;
        }
        if counter >= max_value {
            break;
        }
    }
    // Raise an error if nothing has been found
    if !done {
        panic!("No unique name could be found.");
    }
    // Finished
    final_name
}

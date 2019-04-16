
/// Take two elements from the input and convert them
/// from hexadecimal representation to the corresponding
/// integer representation.
fn hex_to_ascii(hex_input: &str, ascii_chars: &mut Vec<u8>) {
    let mut pair = String::from("");

    for ch in hex_input.chars() {
        pair.push(ch);

        if pair.len() == 2 {
            match u32::from_str_radix(&pair, 16) {
                Ok(c) => {
                    ascii_chars.push(c as u8);
                }
                Err(e) => println!("Invalid char: {}", e),
            }
            pair.clear();
        }
    }
}

/// Convert binary string to sextet.
fn bin_to_sextet(base64_output: &mut String, bin_repr: &str) {
    let mut sextet: Vec<char> = vec![];

    // The Base64 index table alphabet is starts with A at index 0,
    // B at index 1, etc. Add appropriate offsets to shift the elements
    // to the corresponding ASCII values.
    let b64_offset_uppercase = 65;
    let b64_offset_lowercase = 71;
    let b64_offset_digit: i32 = -4;

    for bit in bin_repr.chars() {
        sextet.push(bit);

        if sextet.len() == 6 {
            let bin_str: String = sextet.iter().collect();

            base64_output.push_str(&format!(
                "{}",
                u32::from_str_radix(&bin_str, 2)
                    .ok()
                    .and_then(|n| {
                        if n <= 25 {
                            ::std::char::from_u32(n + b64_offset_uppercase)
                        } else if n >= 26 && n <= 51 {
                            ::std::char::from_u32(n + b64_offset_lowercase)
                        } else if n >= 52 && n <= 61 {
                            ::std::char::from_u32(
                                (n as i32 + b64_offset_digit) as u32)
                        } else {
                            Some('?')
                        }
                    })
                    .unwrap()
            ));

            sextet.clear();
        }
    }
}

/// Convert a hexadecimal input string to a corresponding
/// base64 representation.
fn to_base64(hex_input: &str) -> String {
    let mut ascii_chars: Vec<u8> = vec![];

    hex_to_ascii(&hex_input, &mut ascii_chars);

    // Convert integer to a binary string representation.
    let mut bin_repr = String::from("");

    for ac in ascii_chars {
        bin_repr.push_str(&format!("{:08b}", ac));
    }

    let mut base64_output = String::from("");

    bin_to_sextet(&mut base64_output, &bin_repr);

    base64_output
}

pub fn run() {
    let data: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b\
                      65206120706f69736f6e6f7573206d757368726f6f6d";

    println!("{}", to_base64(data));
}

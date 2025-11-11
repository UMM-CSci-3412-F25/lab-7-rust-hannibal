//DONE: Return the input string without vowels.

pub fn disemvowel(s: &str) -> String {
    let mut output:String = String::new();
    let chars = s.chars();
    for c in chars { // for each char (c) in the string (s)
        if !is_vowel(c) { //s[c].to_string()
            output.push(c); // write char to output
        }
    }
    return output;
}
    
/*
pub fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| !is_vowel(*c)).collect()
    
}
*/
fn is_vowel(c: char) -> bool {
    matches!(c,  'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' )
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(expected, disemvowel(input));
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }

    #[test]
    fn handle_punctuation() {
        assert_eq!(
            "n (nxplnd) lphnt!",
            disemvowel("An (Unexplained) Elephant!")
        );
    }

    #[test]
    fn handle_unicode() {
        assert_eq!(
            "Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳",
            disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳")
        );
        assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺", disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
        assert_eq!(
            " lttl Phncn: 𐤀	𐤈	𐤔	𐤕",
            disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕")
        );
        assert_eq!(
            "W cn hndl mj s wll! 🤣😃👍",
            disemvowel("We can handle emoji as well! 🤣😃👍")
        )
    }
}

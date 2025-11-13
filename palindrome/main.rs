fn main() {
    println!("{}", palindrome("Hello, world!"));
    println!("{}", palindrome("abcdedcba"));
}


//Sets a variable "reversed" equaled to the string reverse function.
//s == reversed gives a boolean and .to_string() returns it as a string.
fn palindrome(s: &str) -> String {
  let reversed = str_reverse(s);
  (s == reversed).to_string()
}

//Wrote str_reverse first. Takes the string of characters reverses then and then
//collects back into a string using collect.
fn str_reverse(s: &str) -> String {
   s.chars().rev().collect()
}

/*
#[cfg(test)]
mod tests {
  use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
  use predicates::prelude::predicate;
  use std::process::Command;

  #[test]
  fn str_rv_works() {
    let input = "Hello World";
    let expected = "dlroW olleH"

    assert_eq!(expected, str_reverse(input));
  }
}

*/

/*
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "palindrome.h"

char *str_reverse(char const *str) {
  int len, i;
  char *result;

  len = strlen(str);
  result = (char*) calloc(len+1, sizeof(char));
  for (i=0; i<len; ++i) {
    result[i] = str[len-i-1];
  }
  result[len] = '\0';

  return result;
}

char *palindrome(char const *str) {
  char *rev;
  int i;
  bool result = true;
  char *answer;

  rev = str_reverse(str);
  i = 0;
  while (result && str[i]) {
    if (str[i] != rev[i]) {
      result = false;
    }
    ++i;
  }

  if (result) {
    answer = (char*) calloc(4, sizeof(char));
    answer[0] = 'Y';
    answer[1] = 'e';
    answer[2] = 's';
    answer[3] = '\0';
  } else {
    answer = (char*) calloc(3, sizeof(char));
    answer[0] = 'N';
    answer[1] = 'o';
    answer[2] = '\0';
  }

  return answer;
}
*/

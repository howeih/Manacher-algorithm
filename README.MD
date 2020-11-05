Longest palindromic substring
<br>
https://en.wikipedia.org/wiki/Longest_palindromic_substring
<br>
In computer science, the longest palindromic substring or longest symmetric factor problem is the problem of finding a maximum-length contiguous substring of a given string that is also a palindrome. For example, the longest palindromic substring of "bananas" is "anana". The longest palindromic substring is not guaranteed to be unique; for example, in the string "abracadabra", there is no palindromic substring with length greater than three, but there are two palindromic substrings with length three, namely, "aca" and "ada". In some applications it may be necessary to return all maximal palindromic substrings (that is, all substrings that are themselves palindromes and cannot be extended to larger palindromic substrings) rather than returning only one substring or returning the maximum length of a palindromic substring.
<br>
Manacher (1975) invented a linear time algorithm for listing all the palindromes that appear at the start of a given string. However, as observed e.g., by Apostolico, Breslauer & Galil (1995), the same algorithm can also be used to find all maximal palindromic substrings anywhere within the input string, again in linear time. Therefore, it provides a linear time solution to the longest palindromic substring problem. Alternative linear time solutions were provided by Jeuring (1994), and by Gusfield (1997), who described a solution based on suffix trees. Efficient parallel algorithms are also known for the problem.



```
fn main() {
    let input1 = "ABBBB".to_owned();
    let result1 = Solution::longest_palindrome(input1);
    assert_eq!("BBBB", &result1);
    let input2 = "ABBABB".to_owned();
    let result2 = Solution::longest_palindrome(input2);
    assert_eq!("BBABB", &result2);
}

```
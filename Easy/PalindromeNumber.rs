impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        /*
        Another solution that is way easier is to make 'x' a string
        Reverse x and store it in another string

        compare both strings and return accordingly
        */

        if x < 0 {
            return false; // Palindrome not possible for negative nums, eg: -121 != 121-
        }

        let mut dig: Vec<i32> = vec![];
        let mut y = x;

        while (y > 0) {
            dig.push( y % 10); // Add the last digit to the vector
            y = y / 10;
        }

        // cloned() makes a copy of each element, collect() collects them into a new vector
        let dig2 :Vec<i32> = dig.iter().rev().cloned().collect();

        if dig == dig2 {
            return true;
        } else {
            return false;
        }
    }
}
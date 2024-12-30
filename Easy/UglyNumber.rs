// Ugly number only has 2,3 and 5 as prime factors
// If n is not reduced to one after dividing it as much as possible with all ugly factors than there is another prime factor and it is not ugly

impl Solution {
    pub fn is_ugly(n: i32) -> bool {

        let ugly :[i32 ; 3] = [2, 3, 5]; // Create array of ugly prime factors (size 3)
        let mut x = n;
        
        // Cover certain edge cases
        if (n <= 0) {
            return false;
        }

        // 1 is considered ugly as it has no prime factors
        if (n == 1) {
            return true; 
        }

        for i in ugly {
            // Loop while x can be divided by current factor
            while (x % i == 0) {
                x = x / i;
            }
        }

        return x == 1; // If all uglies are factors return true
    }
}
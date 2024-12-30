impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        
        let length = digits.len();

        for i in (0..length).rev() {
            if digits[i] == 9 {
                digits[i] = 0; 
            } else {
                digits[i] += 1;
                return digits;
            }
        }
        // If loop was exited entire vec was 9s
        digits[0] = 1;
        digits.push(0); // Push 0 if entire number is 9s
        return digits;
    }
}

/*    FIRST SOL IDEA
        // If the last digit is anything but 9, increment it and return
        if digits[length - 1] != 9 {
            digits[length - 1] += 1;
            return digits; // Return the incremented array
        } else {
            let mut ctr = 0; // Counter variable to track how many 9s present;

            // Iterate from the last digit onward (Inclusise..Exclusive)
            for i in (0..length).rev() {
                if digits[i] == 9 {
                    digits[i] = 0; 
                    ctr += 1; // Increment the counter
                }
            }
            // If not 99999...
            if ctr != length {
                let index_to_increment = length - ctr - 1;
                // Safely increment the corresponding digit
                if index_to_increment >= 0 {
                    digits[index_to_increment] += 1;
                }
            } else {
                digits[0] = 1;
                digits.push(0);
            }   
            return digits;
            
        }         
    }
    */
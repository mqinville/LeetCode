impl Solution {

    pub fn to_binary(mut x: i32) -> Vec<i32> {
        let mut bin_x : Vec<i32> = vec![];

        while (x > 0) {
            bin_x.push(x%2);
            x /= 2; // Divide x by two
        }
        return bin_x; // Return the binary vector
    }

    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        // Get binary equivalents
        let mut bin_x : Vec<i32> = Self::to_binary(x);
        let mut bin_y : Vec<i32> = Self::to_binary(y);
        let mut ctr = 0; // Counter to track hamming distance
        let dif = (bin_x.len() as isize - bin_y.len() as isize).abs(); // Calculate difference in lenghts

        // Resize the vectors to be of equal length
        if bin_x.len() < bin_y.len() {
            bin_x.resize(bin_x.len() + dif as usize, 0);
        } else {
            bin_y.resize(bin_y.len() + dif as usize, 0);
        }
        
        for i in 0..bin_x.len() {
            if bin_x[i] != bin_y[i] {
                ctr += 1; // Increment counter if dif digits
            }
        }
        return ctr;        
    }
}
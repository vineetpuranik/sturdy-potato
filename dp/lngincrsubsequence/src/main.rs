//Longest increasing subsequence
fn main() {
    //create a vector for input sequence
    let nums: Vec<i32> = vec![5, 7, 4, -3, 9, 1, 10, 4, 5, 8, 9, 3];

    //print the length of the longest increasing subsequence from the numbers
    println!("{}", length_of_lis(nums));
}

//function that calculates the length of the longest increasing subsequnce
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut result;

    //if there are no elements in the input sequence
    if nums.is_empty() {
        result = 0;
    }
    //there is 1 or  more than 1 elements in the input sequence
    else {
        //create a vector that will maintain the length of LIS for each index in the sequence
        //initially all the entries in this vector will be 1
        //this corresponds to the fact that intially at each index, the length of the longest increasing subsequnce will atleast be 1
        let mut l = vec![1; nums.len()];

        //iterate through all the elements in the input sequence
        let mut i = 0;

        loop {
            let current_element = nums[i];
            if i > 0 {
                let mut j = 0;

                loop {
                    //check if element at jth index is less than element at ith index
                    //and lis at index i is less than 1 + lis at index j
                    let current_element_j = nums[j];
                    if current_element > current_element_j {
                        let new_lis = 1 + l[j];
                        if new_lis > l[i] {
                            l[i] = new_lis;
                        }
                    }
                    j += 1;
                    //break out of inner loop when we have reached the element at index i
                    if j == i {
                        break;
                    }
                }
            }

            //println!("Index {}, Element {}, LIS {}", i, nums[i], l[i]);
            i += 1;
            //break out of loop when we have reached the last element in the input sequence
            if i == nums.len() {
                break;
            }
        }

        i = 0;
        result = 1;

        loop {
            let current_lis = l[i];

            if current_lis > result {
                result = current_lis;
            }

            i += 1;

            if i >= nums.len() {
                break;
            }
        }
    }
    result
}

//Algo :
// Sub problem in words: L(i) be the length of the longest increasing subsequence for the first i elements
// Get the recurrence relation.
// Key is we need to know the length of the longest increasing subsequence for every ending character

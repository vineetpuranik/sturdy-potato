//Longest increasing subsequence
//Input to the method which will calculate the longest increasing subsequence will be an array of numbers
//Since the numbers can be negative or positive in the array we can use the signed integer type

fn main() {
    //input array of numbers
    let inp = [5, 7, 4, -3, 9, 1, 10, 4, 5, 8, 9, 3];

    //print the length of the longest increasing subsequence from the numbers
    println!("{}" , len_lis(inp));
}

//function that calculates the length of the longest increasing subsequnce
//TODO : the function should be able to accept an array numbers of variable size
//this means we will have to rewrite this function using a vector

fn len_lis(inp:[i32; 12]) -> u32 {
    let inp_first_element = inp[0];
    println!("First number in the input array is {inp_first_element}");
    123
}


//Algo : 
// Sub problem in words: L(i) be the length of the longest increasing subsequence for the first i elements
// Get the recurrence relation.
// Key is we need to know the length of the longest increasing subsequence for every ending character



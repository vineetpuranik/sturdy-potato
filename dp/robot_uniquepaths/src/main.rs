//Number of unique paths for a robot starting from 0,0 of a m*n grid to reach m-1,n-1 (bottom right)
//m >=1 and n<=100 (atleast 1 row and 1 column)

fn main() {
    //m corresponds to number of rows
    let m: i32 = 3;
    //n corresponds to number of columns
    //let n: i32 = 2;
    let n: i32 = 7;

    println!("Result {}", unique_paths(m, n));
}

fn unique_paths(m: i32, n: i32) -> i32 {

    let rows: usize = m.try_into().unwrap();
    let columns: usize = n.try_into().unwrap();

    //create a vector with m rows and n columns
    let mut rbt_up_dp = vec![vec![0; columns]; rows];

    for i in 0..rbt_up_dp.len() {
        for j in 0..rbt_up_dp[i].len() {
            if i == 0 && j == 0 {
                rbt_up_dp[i][j] = 1;
            } else if i == 0 && j > 0 {
                rbt_up_dp[i][j] = 1;
            } else if i > 0 && j == 0 {
                rbt_up_dp[i][j] = 1;
            } else if i > 0 && j > 0 {
                rbt_up_dp[i][j] = rbt_up_dp[i - 1][j] + rbt_up_dp[i][j - 1];
            }
        }
    }

    //return result
    rbt_up_dp[rows - 1][columns - 1]
}

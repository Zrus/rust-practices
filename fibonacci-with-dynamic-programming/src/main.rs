fn main() {
    println!("{}", f(90));
}

fn f_memo(n: usize, memo: &mut Vec<usize>) -> usize {
    if n == 1 || n == 2 {
        return 1;
    }
    if memo[n] != 0 {
        return memo[n];
    }
    let result = f_memo(n - 1, memo) + f_memo(n - 2, memo);
    memo[n] = result;
    result
}

fn f(n: usize) -> usize {
    let mut memo: Vec<usize> = vec![0; n + 1];
    f_memo(n, &mut memo)
}

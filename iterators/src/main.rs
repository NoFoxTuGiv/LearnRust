fn main() {
    let data = [1, 2, 3, 4, 5, 6];

    let sum: i32 = data
        .iter()
        .filter(|x| *x % 2 == 0)
        .sum();

    println!("{}", &sum);
}

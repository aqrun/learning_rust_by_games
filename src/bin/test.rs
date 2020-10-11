fn main() {
    let width = 2;
    let height = 3;
    let mut a: Vec<Vec<String>> = vec![vec![String::from(""); width]; height];

    for i in 0..height {
        for j in 0..width {
            let s = format!("{}:{}", i + 1, j + 1);
            a[i][j] = s;
        }
    }
    println!("{:?}", a);
    println!("{}", a[2][1]);
}
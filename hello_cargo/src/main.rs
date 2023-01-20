// Write a rust function to take a vector of integers and
// return the sum of the integers.

fn sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = sum(&v);
    println!("Sum of {:?} is {}", v, sum);
}

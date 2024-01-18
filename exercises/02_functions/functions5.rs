// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

//Au vu de l'erreur, et au vu de la fonction il suffisait d'enlever le point-virgule apres le "num" pour return dans la fonction


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

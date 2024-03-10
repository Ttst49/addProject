use add_one;


fn main() {
    let nombre = 10;
    println!(
        "Hello world! {} plus one equal {}",
        {nombre},
        {add_one::add_one(nombre)}
    )
}

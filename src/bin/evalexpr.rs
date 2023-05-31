pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    println!("{}", evalexpr::eval(&args)?);

    Ok(())
}

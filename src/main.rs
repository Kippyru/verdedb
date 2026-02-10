use std::{
    fs::File,
    io::{self, Write},
};

fn main() -> std::io::Result<()> {
    let mut tecla = String::new();

    io::stdin().read_line(&mut tecla).expect("error al leer");

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(tecla.trim().parse::<i32>().unwrap());

    let second: &i32 = &v[1];
    println!("el segundo elemento es: {second}");

    let mut file = File::options()
        .append(true)
        .create(true)
        .open("./src/test.txt")?;

    for i in &v {
        println!("{i}");
        write!(&mut file, "{i} ")?;
    }
    writeln!(&mut file, " ")?;
    file.flush()?;
    Ok(())
}

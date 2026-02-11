use distancia::{DistanciaMetrica, comp};

mod distancia;
mod vector;

fn main() {
    let a = vec![1.0_f32, 2.0, 3.0];
    let b = vec![4.0_f32, 5.0, 6.0];

    let l2 = comp(&a, &b, DistanciaMetrica::L2).unwrap();
    println!("L2: {}", l2);

    let coseno = comp(&a, &b, DistanciaMetrica::Coseno).unwrap();
    println!("coseno: {}", coseno);

    let escalar = comp(&a, &b, DistanciaMetrica::Escalar).unwrap();
    println!("Producto escalar: {}", escalar);
}

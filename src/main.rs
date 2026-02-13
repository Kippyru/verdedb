use distancia::DistanciaMetrica;

use crate::vector::DbVector;

mod distancia;
mod vector;

fn test_vector(dim: usize, seed: f32) -> Vec<f32> {
    (0..dim).map(|i| i as f32 + seed).collect()
}

fn main() {
    let dim = 384;
    let mut db = DbVector::new(dim);

    for i in 0..10 {
        db.insertar(test_vector(dim, i as f32)).unwrap();
    }

    let query = test_vector(dim, 10.0);

    let resultados = db.buscar(&query, 5, DistanciaMetrica::Coseno).unwrap();

    println!("resultadoss: {:?}", resultados);
}

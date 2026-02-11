#[derive(Debug, Clone, Copy)]
pub enum DistanciaMetrica {
    L2,
    Coseno,
    Escalar,
}

#[derive(Debug)]
pub enum DistanciaError {
    DimensionMismatch,
    ZeroVector,
}

pub fn comp(a: &[f32], b: &[f32], metrica: DistanciaMetrica) -> Result<f32, DistanciaError> {
    if a.len() != b.len() {
        return Err(DistanciaError::DimensionMismatch);
    }

    match metrica {
        DistanciaMetrica::L2 => Ok(l2(a, b)),
        DistanciaMetrica::Coseno => coseno(a, b),
        DistanciaMetrica::Escalar => escalar(a, b),
    }
}

fn l2(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum()
}
fn coseno(a: &[f32], b: &[f32]) -> Result<f32, DistanciaError> {
    let mut producto_escalar = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        producto_escalar += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return Err(DistanciaError::ZeroVector);
    }
    Ok(producto_escalar / (norm_a.sqrt() * norm_b.sqrt()))
}

fn escalar(a: &[f32], b: &[f32]) -> Result<f32, DistanciaError> {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

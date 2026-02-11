use crate::distancia::{DistanciaError, DistanciaMetrica, comp};
#[derive(Debug)]
pub enum DbError {
    DimensionMismatch,
    DistanciaError(DistanciaError),
}
#[derive(Debug)]
pub struct DbVector {
    dimension: usize,
    vectores: Vec<Vec<f32>>,
}

impl DbVector {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            vectores: Vec::new(),
        }
    }
    pub fn insertar(&mut self, vector: Vec<f32>) -> Result<usize, DbError> {
        if vector.len() != self.dimension {
            return Err(DbError::DimensionMismatch);
        }
        self.vectores.push(vector);
        Ok(self.vectores.len() - 1) //esta vendria ser la id
    }

    pub fn buscar(
        &self,
        query: &[f32],
        k: usize,
        metrica: DistanciaMetrica,
    ) -> Result<Vec<(usize, f32)>, DbError> {
        if query.len() != self.dimension {
            return Err(DbError::DimensionMismatch);
        }

        let mut resultados = Vec::with_capacity(self.vectores.len());

        for (id, vector) in self.vectores.iter().enumerate() {
            let score = comp(query, vector, metrica).map_err(DbError::DistanciaError)?;

            resultados.push((id, score));
        }
        match metrica {
            DistanciaMetrica::L2 => {
                resultados.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            }
            DistanciaMetrica::Coseno | DistanciaMetrica::Escalar => {
                resultados.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            }
        }
        resultados.truncate(k);
        Ok(resultados)
    }
}

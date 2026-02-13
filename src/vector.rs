use crate::distancia::{DistanciaError, DistanciaMetrica, comp};
#[derive(Debug)]
pub enum DbError {
    DimensionMismatch,
    DistanciaError(DistanciaError),
}
#[derive(Debug)]
pub struct DbVector {
    dimension: usize,
    len: usize,
    vectores: Vec<f32>,
}

impl DbVector {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            len: 0,
            vectores: Vec::new(),
        }
    }
    pub fn insertar(&mut self, vector: Vec<f32>) -> Result<usize, DbError> {
        if vector.len() != self.dimension {
            return Err(DbError::DimensionMismatch);
        }
        self.vectores.extend_from_slice(&vector);
        let id = self.len;
        self.len += 1;
        Ok(id)
    }

    fn get_vector(&self, id: usize) -> &[f32] {
        let start = id * self.dimension;
        let end = start + self.dimension;
        &self.vectores[start..end]
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

        let mut resultados = Vec::with_capacity(self.len);

        for id in 0..self.len {
            let vector = self.get_vector(id);

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

use polars::prelude::*;
pub fn normalize_grades(grade: Expr) -> Expr {
    grade.map(|s| {
        let chunks: Float32Chunked = s.f32()?.apply(|value| match value.unwrap() {
            value if value > 100.0 => Some(value / 10.0),
            value if value <= 100.0 => Some(value / 1.0),
            _ => None
        });

        Ok(Some(chunks.into_series()))
    }, GetOutput::default())
}
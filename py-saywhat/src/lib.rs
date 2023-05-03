use pyo3::prelude::*;

mod distance {
    use pyo3::prelude::*;

    #[pyfunction]
    #[pyo3(signature = (s1, s2, substitution_cost = 1, transpositions = false))]
    pub fn edit_distance(
        s1: String,
        s2: String,
        substitution_cost: usize,
        transpositions: bool,
    ) -> PyResult<usize> {
        use saywhat;
        Ok(saywhat::edit_distance(
            &s1.chars().collect::<Vec<char>>(),
            &s2.chars().collect::<Vec<char>>(),
            substitution_cost,
            transpositions,
        ))
    }
}

#[pymodule]
fn saywhat(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::distance::edit_distance, m)?)?;
    Ok(())
}

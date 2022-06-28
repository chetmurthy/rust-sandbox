
use pyo3::prelude::*;
use pyo3::types::{ PySlice, PyTuple } ;
use pyo3::exceptions::PyException;

#[derive(FromPyObject)]
enum SliceOrInt<'a> {
    Slice(&'a PySlice),
    Int(isize),
}


#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct Thing {
    members : Vec<(String, i64)>,
}

#[pymethods]
impl Thing {

    #[new]
    pub fn new(v : Vec<(String, i64)>) -> PyResult<Thing> {
        let it = Thing { members : v.clone() } ;
        Ok(it)
    }

    fn __getitem__(&self, idx: SliceOrInt, py: Python) -> PyResult<(String, i64)> {
        match idx {
            SliceOrInt::Slice(_) => panic!("slice"),
            SliceOrInt::Int(idx) => {
                (0 <= idx && idx < self.members.len() as isize).then(|| ())
                    .ok_or(PyException::new_err(format!("__getitem__ called on invalid index {}", idx))) ? ;
                let m = &self.members[idx as usize] ;
                let t0 = m.0.clone() ;
                let t1 = m.1 ;
                let t = (t0, t1) ;
                Ok(t)
            }
        }
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn hello_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Thing>()?;
    Ok(())
}


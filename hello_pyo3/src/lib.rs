
use pyo3::prelude::*;
use pyo3::types::{ PySlice } ;
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


enum SliceResult<T> {
    It(T),
    Slice(Vec<T>)
}
impl<T : IntoPy<PyObject>> IntoPy<PyObject> for SliceResult<T> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            SliceResult::It(it) => it.into_py(py),
            SliceResult::Slice(v) => {
                v.into_py(py)
            }
        }
    }
}

struct MyMember(String, i64);

impl IntoPy<PyObject> for MyMember {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let py0 = self.0.into_py(py) ;
        let py1 = self.1.into_py(py) ;
        //let pyt : PyObject = pyo3::callback::convert(py, (py0, py1)).unwrap() ;
        let pyt : PyObject = (py0, py1).into_py(py) ;
        pyt
    }
}

#[pymethods]
impl Thing {

    #[new]
    pub fn new(v : Vec<(String, i64)>) -> PyResult<Thing> {
        let it = Thing { members : v.clone() } ;
        Ok(it)
    }

    fn __getitem__(&self, idx: SliceOrInt, py: Python) -> PyResult<SliceResult<(String, i64)>> {
        match idx {
            SliceOrInt::Slice(slice) => {
                let psi = slice.indices(self.members.len() as i64)? ;
                let (start, stop, step) = (psi.start, psi.stop, psi.step) ;
                let m : Vec<(String, i64)> =
                    self.members[start as usize..stop as usize].iter()
                    .step_by(step as usize)
                    .map(|p| (p.0.clone(), p.1))
                    .collect() ;
                let m = SliceResult::Slice(m) ;
                Ok(m)
            },
            SliceOrInt::Int(idx) => {
                (0 <= idx && idx < self.members.len() as isize).then(|| ())
                    .ok_or(PyException::new_err(format!("__getitem__ called on invalid index {}", idx))) ? ;
                let m = &self.members[idx as usize] ;
                let m = SliceResult::It((m.0.clone(), m.1)) ;
                Ok(m)
            }
        }
    }

    fn foo(&self, idx: isize, py: Python) -> (String, i64) {
        let m = &self.members[idx as usize] ;
        let t0 = m.0.clone() ;
        let t1 = m.1 ;
        let t = (t0, t1) ;
        t
    }

    fn foo2(&self, idx: isize, py: Python) -> PyResult<SliceResult<(String, i64)>> {
        let m = &self.members[idx as usize] ;
        let m = SliceResult::It((m.0.clone(), m.1)) ;
        Ok(m)
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn hello_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Thing>()?;
    Ok(())
}


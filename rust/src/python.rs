//! PyO3 surface for Premyslid tools and other Python consumers.
//!
//! Built only with `--features python`. `cargo test` stays free of a Python link.

use pyo3::exceptions::{PyLookupError, PyValueError};
use pyo3::prelude::*;

use crate::date::{biological_age, Date};
use crate::error::Error;
use crate::person::Person;
use crate::world::World;

fn to_py(err: Error) -> PyErr {
    match err {
        Error::InvalidArgument(msg) => PyValueError::new_err(msg),
        Error::OutOfRange(msg) => PyLookupError::new_err(msg),
    }
}

#[pyclass(name = "Date", module = "suvorov_core")]
#[derive(Clone, Copy)]
pub struct PyDate {
    inner: Date,
}

#[pymethods]
impl PyDate {
    #[new]
    fn new(year: i32, month: i32, day: i32) -> Self {
        Self {
            inner: Date::new(year, month, day),
        }
    }

    #[getter]
    fn year(&self) -> i32 {
        self.inner.year
    }

    #[getter]
    fn month(&self) -> i32 {
        self.inner.month
    }

    #[getter]
    fn day(&self) -> i32 {
        self.inner.day
    }

    fn valid(&self) -> bool {
        self.inner.valid()
    }

    fn __repr__(&self) -> String {
        format!("Date({}, {}, {})", self.inner.year, self.inner.month, self.inner.day)
    }

    fn __richcmp__(&self, other: &PyDate, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.inner == other.inner,
            pyo3::basic::CompareOp::Ne => self.inner != other.inner,
            pyo3::basic::CompareOp::Lt => self.inner < other.inner,
            pyo3::basic::CompareOp::Le => self.inner <= other.inner,
            pyo3::basic::CompareOp::Gt => self.inner > other.inner,
            pyo3::basic::CompareOp::Ge => self.inner >= other.inner,
        }
    }
}

impl From<Date> for PyDate {
    fn from(inner: Date) -> Self {
        Self { inner }
    }
}

#[pyclass(name = "Person", module = "suvorov_core")]
#[derive(Clone)]
pub struct PyPerson {
    inner: Person,
}

#[pymethods]
impl PyPerson {
    #[new]
    fn new(
        names: Vec<String>,
        allegiances: Vec<u32>,
        birth_date: PyDate,
        birth_location_id: u32,
        current_location_id: u32,
    ) -> Self {
        Self {
            inner: Person::new(
                names,
                allegiances,
                birth_date.inner,
                birth_location_id,
                current_location_id,
            ),
        }
    }

    #[getter]
    fn names(&self) -> Vec<String> {
        self.inner.names.clone()
    }

    #[getter]
    fn allegiances(&self) -> Vec<u32> {
        self.inner.allegiances.clone()
    }

    #[getter]
    fn birth_date(&self) -> PyDate {
        PyDate::from(self.inner.birth_date)
    }

    #[getter]
    fn birth_location_id(&self) -> u32 {
        self.inner.birth_location_id
    }

    #[getter]
    fn current_location_id(&self) -> u32 {
        self.inner.current_location_id
    }
}

#[pyclass(name = "World", module = "suvorov_core")]
pub struct PyWorld {
    inner: World,
}

#[pymethods]
impl PyWorld {
    #[new]
    fn new(year: i32, month: i32, day: i32) -> PyResult<Self> {
        Ok(Self {
            inner: World::new(year, month, day).map_err(to_py)?,
        })
    }

    fn add_polity(&mut self, name: String) -> PyResult<u32> {
        self.inner.add_polity(name).map_err(to_py)
    }

    fn polity_name(&self, polity_id: u32) -> PyResult<String> {
        self.inner
            .polity_name(polity_id)
            .map(str::to_string)
            .map_err(to_py)
    }

    fn add_location(&mut self, name: String) -> PyResult<u32> {
        self.inner.add_location(name).map_err(to_py)
    }

    fn location_name(&self, location_id: u32) -> PyResult<String> {
        self.inner
            .location_name(location_id)
            .map(str::to_string)
            .map_err(to_py)
    }

    fn add_person(&mut self, person: PyRef<PyPerson>) -> PyResult<u32> {
        self.inner.add_person(person.inner.clone()).map_err(to_py)
    }

    fn person(&self, person_id: u32) -> PyResult<PyPerson> {
        self.inner
            .person(person_id)
            .map(|p| PyPerson { inner: p.clone() })
            .map_err(to_py)
    }

    fn person_age(&self, person_id: u32) -> PyResult<i32> {
        self.inner.person_age(person_id).map_err(to_py)
    }

    fn set_person_location(&mut self, person_id: u32, location_id: u32) -> PyResult<()> {
        self.inner
            .set_person_location(person_id, location_id)
            .map_err(to_py)
    }

    fn advance_one_day(&mut self) {
        self.inner.advance_one_day();
    }

    fn date(&self) -> PyDate {
        PyDate::from(self.inner.date())
    }

    fn person_count(&self) -> usize {
        self.inner.person_count()
    }
}

#[pyfunction]
#[pyo3(name = "biological_age")]
fn py_biological_age(birth: PyDate, on: PyDate) -> PyResult<i32> {
    biological_age(birth.inner, on.inner).map_err(to_py)
}

#[pymodule]
fn suvorov_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDate>()?;
    m.add_class::<PyPerson>()?;
    m.add_class::<PyWorld>()?;
    m.add_function(wrap_pyfunction!(py_biological_age, m)?)?;
    Ok(())
}

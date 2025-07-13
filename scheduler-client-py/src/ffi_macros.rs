use pyo3::prelude::*;

/// Macro to quickly create a Python wrapper for a Rust struct
/// Usage:
/// py_wrapper!(MyPyClass, MyRustStruct, {
///     getters: [field1, field2],
///     setters: [field3, field4],
///     consumers: [(method_name, ReturnType)],
///     statics: [(static_method, ReturnType)]
/// });
#[macro_export]
macro_rules! py_wrapper {
    (
        $py_class:ident,
        $rust_type:ty,
        {
            getters: [$($getter:ident),*],
            setters: [$($setter:ident),*],
            consumers: [$(($consumer:ident, $consumer_ret:ident)),*],
            statics: [$(($static_method:ident, $static_ret:ident)),*]
        }
    ) => {
        #[pyclass]
        pub struct $py_class {
            pub(crate) inner: $rust_type,
        }

        #[pymethods]
        impl $py_class {
            // Generate getters
            $(
                #[getter]
                fn $getter(&self) -> String {
                    self.inner.$getter().to_string()
                }
            )*

            // Generate setters (builder pattern)
            $(
                fn $setter(mut slf: PyRefMut<Self>, value: String) -> PyRefMut<Self> {
                    slf.inner = std::mem::take(&mut slf.inner).$setter(value);
                    slf
                }
            )*

            // Generate consuming methods
            $(
                fn $consumer(mut slf: PyRefMut<Self>) -> $consumer_ret {
                    let inner = std::mem::replace(&mut slf.inner, Default::default());
                    $consumer_ret {
                        inner: inner.$consumer(),
                    }
                }
            )*

            // Generate static methods
            $(
                #[staticmethod]
                fn $static_method() -> $static_ret {
                    $static_ret {
                        inner: <$rust_type>::$static_method(),
                    }
                }
            )*

            fn __repr__(&self) -> String {
                format!("{}({:?})", stringify!($py_class), self.inner)
            }
        }
    };
}

// Future enhancement: Automatic module exports
#[macro_export]
macro_rules! py_exports {
    ($module:ident: $($class:ident),*) => {
        #[pymodule]
        mod $module {
            $(
                #[pymodule_export]
                use super::$class;
            )*
        }
    };
}

pub use py_exports;
pub use py_wrapper;

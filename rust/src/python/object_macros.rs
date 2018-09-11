#[macro_export]
macro_rules! initialize_python_object {
    // The objective of this macro is to provide a "simpler" way to initialize an python object
    // outside of a constructor. As to do it there are a couple of actions to be taken and has to be
    // into an unsafe block it's better to keep it under-control, have it written once and have rustc
    // to propagate it where is needed
    //
    // Example of usage:
    //      initialize_python_object!(py, cls, |token| Self {
    //          token,
    //          swagger_schema: SwaggerSchema::new_from_url(url).unwrap()
    //      })
    // is equivalent to write
    //      {
    //          unsafe {
    //              let obj = PyRawObject::new(
    //                  py, cls.as_type_ptr(),
    //                  cls.as_type_ptr()
    //              )?;
    //              obj.init(|token| Self {
    //                  token,
    //                  swagger_schema: SwaggerSchema::new_from_url(url).unwrap()
    //              });
    //              Ok(PyObject::from_owned_ptr(py, obj.into_ptr()))
    //          }
    //      }
    ($py:expr, $cls:expr, $init_block:expr) => {{
        unsafe {
            let obj = PyRawObject::new($py, $cls.as_type_ptr(), $cls.as_type_ptr())?;
            let _ = obj.init($init_block);
            Ok(PyObject::from_owned_ptr($py, obj.into_ptr()))
        }
    }};
}

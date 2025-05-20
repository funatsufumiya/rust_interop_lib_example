use std::os::raw::c_float;

#[unsafe(no_mangle)]
pub extern "C" fn add(left: c_float, right: c_float) -> c_float {
    left + right
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_relative_eq;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_float_relative_eq!(result, 4.0);
    }
}

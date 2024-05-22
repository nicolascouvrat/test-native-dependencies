mod bindings;

use bindings::version;

pub fn a_rust_endpoint() {
    println!("hello rust");
    unsafe {
        version();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        a_rust_endpoint()
    }
}

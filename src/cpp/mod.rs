extern "C" {
    fn adder(x: i32, y: i32) -> i32;
}

fn rust_adder(x: i32, y: i32) -> i32 {
    unsafe {
        return adder(x, y);
    }
}

// cargo test cpp
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_adder() {
        let x = 1;
        let y = 2;
        let result = rust_adder(x, y);
        println!("{} + {} = {}", x, y, result);
        assert_eq!(result, 3);
    }

    #[bench]
    fn bench_adder(b: &mut Bencher) {
        b.iter(|| {
            rust_adder(1, 4);
        });
    }
}

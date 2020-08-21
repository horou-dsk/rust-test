trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object { 1 }
        else if &self == &object { 0 }
        else { -1 }
    }
}

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait Deb {
    fn wc(&self) -> u8;
}

impl Deb for A {
    fn wc(&self) -> u8 {
        return 1;
    }
}

impl Deb for B {
    fn wc(&self) -> u8 {
        return 2;
    }
}

pub fn run() {
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    let dev_i = A {};
    println!("maximum of arr is {} {}", max(&arr), dev_i.wc());
}

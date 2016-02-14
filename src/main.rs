fn main() {
    // println!("Answer is {}", mult_acc4(4, 5, 6));
    println!("Answer is {}", mult_acc4(4, 5, 6));
    // println!("Half is {}", half(5));
    // println!("Half is {}", half(1));
    // println!("Answer is {}", odd(1));
}
trait Integer {
    fn odd(&self) -> bool;
    fn half(self) -> Self;
    fn add(&self, &Self) -> Self;
    fn eq1(&self) -> bool;
}

impl Integer for i32 {
    fn odd(&self) -> bool {
        if self % 2 == 0 {
            false
        } else {
            true
        }
    }
    fn half(self) -> Self {
        self / 2
    }
    fn add(&self, ta: &Self) -> Self {
        self + ta
    }
    fn eq1(&self) -> bool {
        *self == 1
    }
}
// fn mult_acc4(r: i32, n: i32, a: i32) -> i32 {
fn mult_acc4<T: Integer>(r: T, n: T, a: T) -> T {
    let mut r = r;
    let mut n = n;
    let mut a = a;
    loop {
        // if odd(n) {
        if n.odd() {
            // r = r + a;
            r = r.add(&a);
            // if n == 1 {
            if n.eq1() {
                return r;
            }
        }
        // n = half(n);
        n = n.half();
        a = a.add(&a);
    }
}

// fn odd(n: i32) -> bool {
//     if n % 2 == 0 {
//         false
//     } else {
//         true
//     }
// }
//
// fn half(n: i32) -> i32 {
//     n / 2
// }

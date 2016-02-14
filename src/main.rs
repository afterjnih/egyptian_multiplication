fn main() {
    println!("Answer is {}", mult_acc4(4, 5, 6));
    println!("Half is {}", half(5));
    println!("Half is {}", half(1));
    println!("Answer is {}", odd(1));
}

fn mult_acc4(r: i32, n: i32, a: i32) -> i32 {
    let mut r = r;
    let mut n = n;
    let mut a = a;
    loop {
        if odd(n) {
            r = r + a;
            if n == 1 {
                return r;
            }
        }
        n = half(n);
        a = a + a;
    }
}

fn odd(n: i32) -> bool {
    if n % 2 == 0 {
        false
    } else {
        true
    }
}

fn half(n: i32) -> i32 {
    n / 2
}

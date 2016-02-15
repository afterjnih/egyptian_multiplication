use std::fmt;
fn main() {
    // println!("Answer is {}", mult_acc4(4, 5, 6));
    let r = Integer { num: 4 };
    let n = Integer { num: 5 };
    let a = Integer { num: 6 };
    println!("Answer is {:?}", mult_acc4(r, n, a));
}
trait Caliculatable {
    fn odd(&self) -> bool;
    fn half(&self) -> Self;
    fn add(&self, &Self) -> Self;
    fn eq1(&self) -> bool;
}

struct Integer<T> {
    num: T,
}

impl<T> fmt::Debug for Integer<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.num)
    }
}

impl Caliculatable for Integer<i32> {
    fn odd(&self) -> bool {
        self.num % 2 != 0
    }
    fn half(&self) -> Self {
        Integer { num: self.num / 2 }
    }
    fn add(&self, operand: &Self) -> Self {
        Integer { num: self.num + operand.num }
    }
    fn eq1(&self) -> bool {
        self.num == 1
    }
}

fn mult_acc4<T: Caliculatable>(r: T, n: T, a: T) -> T {
    let mut r = r;
    let mut n = n;
    let mut a = a;
    loop {
        if n.odd() {
            r = r.add(&a);
            if n.eq1() {
                return r;
            }
        }
        n = n.half();
        a = a.add(&a);
    }
}

// class Foo {
//     public:
//         Foo() {

//         }

//         void first(function<void()> printFirst) {

//             // printFirst() outputs "first". Do not change or remove this line.
//             printFirst();
//         }

//         void second(function<void()> printSecond) {

//             // printSecond() outputs "second". Do not change or remove this line.
//             printSecond();
//         }

//         void third(function<void()> printThird) {

//             // printThird() outputs "third". Do not change or remove this line.
//             printThird();
//         }
//     };

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

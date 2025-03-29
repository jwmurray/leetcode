#[derive(Debug, PartialEq)]
struct LinkedNumber {
    value: u32,
    next: Option<Box<LinkedNumber>>,
}

const radix: u32 = 10; // We are working in base-10, so the radix is 10

impl LinkedNumber {
    pub fn new(value: u32) -> Self {
        LinkedNumber::new_linked_number_recursive(value, None)
    }

    fn new_linked_number_recursive(value: u32, next: Option<Box<LinkedNumber>>) -> Self {
        if value < radix {
            // Base case: create the last node
            LinkedNumber { value, next }
        } else {
            let digit = value % radix;
            let new_value = value / radix;

            // Recursively create the next node and link it
            LinkedNumber::new_linked_number_recursive(
                new_value,
                Some(Box::new(LinkedNumber { value: digit, next })),
            )
        }
    }

    pub fn add(&self, a: LinkedNumber) -> Self {
        LinkedNumber::new_linked_number_recursive(0, None)
    }

    pub fn get_value(&self) -> u32 {
        let mut sum = 0;
        let mut current_number: &LinkedNumber = self;
        let mut multiplier = 1;
        loop {
            sum += current_number.value * multiplier;
            multiplier *= radix;
            if let Some(next_linked_number) = current_number.next.as_ref() {
                current_number = next_linked_number.as_ref();
            } else {
                break;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_linked_number() {
        let num0 = LinkedNumber::new(0);
        assert_eq!(num0.get_value(), 0);
        let num3 = LinkedNumber::new(3);
        assert_eq!(num3.get_value(), 3);
        let num342 = LinkedNumber::new(342);
        assert_eq!(num342.get_value(), 342);
        // let num465 = LinkedNumber::new(465);
        // let sum807 = LinkedNumber::new(807);

        // assert_eq!(num342.add(num465), sum807);
    }
}

struct MinStack {
    stack: Vec<(i32, i32)>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn push(&mut self, x: i32) {
        let current_min = self.get_min();
        let new_min = if x < current_min { x } else { current_min };
        self.stack.push((x, new_min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        if let Some(pair) = self.stack.last() {
            return pair.0;
        }
        0
    }

    fn get_min(&self) -> i32 {
        if let Some(pair) = self.stack.last() {
            return pair.1;
        }
        std::i32::MAX
    }
}

/*
 Your MinStack object will be instantiated and called as such:
 let obj = MinStack::new();
 obj.push(x);
 obj.pop();
 let ret_3: i32 = obj.top();
 let ret_4: i32 = obj.get_min();
*/
// cargo watch -x "test _0155_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_case_1() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        assert_eq!(obj.top(), 0);
        assert_eq!(obj.get_min(), -2);
    }
    #[test]
    fn run_test_case_2() {
        let mut obj = MinStack::new();
        obj.push(2);
        obj.push(0);
        obj.push(3);
        obj.push(0);
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.get_min(), 0);
        obj.pop();
        assert_eq!(obj.get_min(), 2);
    }
}

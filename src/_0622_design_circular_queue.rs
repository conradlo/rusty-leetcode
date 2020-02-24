// 4ms, 2.2MB
struct MyCircularQueue {
    head: usize,
    tail: usize,
    size: usize,
    array: Vec<i32>,
    _empty: bool,
    _new_init: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        let size = k as usize;
        MyCircularQueue {
            head: 0,
            tail: 0,
            size,
            array: vec![0; size],
            _empty: true,
            _new_init: true,
        }
    }
    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() || self.size < 1 {
            return false;
        }
        if self.is_empty() {
            self._empty = false;
        }
        if self._new_init {
            self._new_init = false;
        } else {
            self.tail = (self.tail + 1) % self.size;
        }
        self.array[self.tail] = value;
        true
    }
    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.head == self.tail {
            self._empty = true;
        }
        self.head = (self.head + 1) % self.size;
        true
    }
    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.array[self.head]
        }
    }
    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.array[self.tail]
        }
    }
    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        // self.head == self.tail
        self._empty
    }
    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        !self.is_empty() && (self.tail + 1) % self.size == self.head
    }
}

/*
 Your MyCircularQueue object will be instantiated and called as such:
 let obj = MyCircularQueue::new(k);
 let ret_1: bool = obj.en_queue(value);
 let ret_2: bool = obj.de_queue();
 let ret_3: i32 = obj.front();
 let ret_4: i32 = obj.rear();
 let ret_5: bool = obj.is_empty();
 let ret_6: bool = obj.is_full();
*/
// cargo watch -x "test _0622_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_case_0() {
        let mut obj = MyCircularQueue::new(0);
        assert_eq!(obj.is_empty(), true);
        assert_eq!(obj.is_full(), false);
        assert_eq!(obj.en_queue(2), false);
        assert_eq!(obj.de_queue(), false);
        assert_eq!(obj.front(), -1);
        assert_eq!(obj.rear(), -1);
    }
    #[test]
    fn run_test_case_1() {
        let mut obj = MyCircularQueue::new(1);
        assert_eq!(obj.is_empty(), true);
        assert_eq!(obj.is_full(), false);
        assert_eq!(obj.en_queue(9), true);
        assert_eq!(obj.is_full(), true);
        assert_eq!(obj.is_empty(), false);
        assert_eq!(obj.front(), 9);
        assert_eq!(obj.rear(), 9);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.is_empty(), true);
        assert_eq!(obj.is_full(), false);
        assert_eq!(obj.front(), -1);
        assert_eq!(obj.rear(), -1);
    }

    #[test]
    fn run_test_case_2() {
        let mut obj = MyCircularQueue::new(3);
        assert_eq!(obj.en_queue(1), true);
        assert_eq!(obj.en_queue(2), true);
        assert_eq!(obj.en_queue(3), true);
        assert_eq!(obj.en_queue(4), false);
        assert_eq!(obj.front(), 1);
        assert_eq!(obj.rear(), 3);
        assert!(obj.is_full(), true);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.en_queue(4), true);
        assert_eq!(obj.rear(), 4);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.de_queue(), true);
        assert_eq!(obj.de_queue(), false);
        assert_eq!(obj.is_empty(), true);

        assert_eq!(obj.front(), -1);
        assert_eq!(obj.rear(), -1);
    }
}

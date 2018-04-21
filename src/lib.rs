extern crate cons_list;

use cons_list::ConsList;

#[derive(Debug)]
pub struct BatchedQueue<T> {
    f: ConsList<T>,
    r: ConsList<T>,
}

impl<T: Clone> BatchedQueue<T> {
    pub fn empty() -> BatchedQueue<T> {
        BatchedQueue {
            f: ConsList::new(),
            r: ConsList::new(),
        }
    }

    fn rev(lis: &ConsList<T>) -> ConsList<T> {
        lis.iter()
            .fold(ConsList::new(), |acc, x| acc.append(x.clone()))
    }

    fn checkf(f: ConsList<T>, r: ConsList<T>) -> BatchedQueue<T> {
        if f.is_empty() {
            BatchedQueue {
                f: BatchedQueue::rev(&r),
                r: ConsList::new(),
            }
        } else {
            BatchedQueue { f: f, r: r }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    pub fn snoc(&self, x: T) -> BatchedQueue<T> {
        match self {
            BatchedQueue { f, r } => BatchedQueue::checkf(f.clone(), r.append(x)),
        }
    }

    pub fn head(&self) -> Result<T, String> {
        match self {
            BatchedQueue { f, r: _ } if f.is_empty() => Err("empty".to_string()),
            BatchedQueue { f, r: _ } => Ok(f.head().unwrap().clone()),
        }
    }

    pub fn tail(&self) -> Result<BatchedQueue<T>, String> {
        match self {
            BatchedQueue { f, r: _ } if f.is_empty() => Err("empty".to_string()),
            BatchedQueue { f, r } => Ok(BatchedQueue::checkf(f.tail(), r.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use BatchedQueue;

    #[test]
    fn it_works() {
        let q0: BatchedQueue<i64> = BatchedQueue::empty();
        let q1 = q0.snoc(1).snoc(2).snoc(3);
        let q2 = q1.tail().unwrap();
        let q3 = q2.snoc(4);
        let q4 = q3.tail().unwrap();
        let q5 = q4.tail().unwrap();
        assert_eq!(q3.head(), Ok(2));
        assert_eq!(q4.head(), Ok(3));
        assert_eq!(q5.head(), Ok(4));
    }
}

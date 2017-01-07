extern crate libc;

use std::mem;
use std::ptr;

// Internal node
struct Node<T> {
    data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>
}

// Struct that holds a doubly-linked list
pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        let p_head = 0 as *mut Node<T>;
        let p_tail = 0 as *mut Node<T>;
        List::<T>{ head: p_head, tail: p_tail }
    }
}

impl<T> List<T> {
    pub fn len(&mut self) -> usize {
        unsafe {
            let mut current = self.head;
            let mut count = 0 as usize;
            while !current.is_null() {
                current = (*current).next;
                count += 1;
            }
            count
        }
    }
}

impl<T> List<T> {
    pub fn push(&mut self, value: T) {
        unsafe {
            let node = libc::malloc(
                mem::size_of::<Node<T>>() as libc::size_t)
                as *mut Node<T>;
            ptr::write(node, Node {
                data: value,
                next: ptr::null_mut(),
                prev: ptr::null_mut()
            });

            if self.head.is_null() {
                self.head = node;
                self.tail = node;
            } else {
                (*self.tail).next = node;
                (*node).prev = self.tail;
                self.tail = node;
            }
        }
    }
}

impl<T> List<T> where T: Copy {
    pub fn pop<'a>(&mut self) -> Result<T, &'a str> {
        unsafe {
            if self.head.is_null() {
                return Err("Empty list");
            }

            if self.head == self.tail {
                let data = (*self.head).data;
                self.head = 0 as *mut Node<T>;
                self.tail = 0 as *mut Node<T>;
                Ok(data)
            } else {
                let data = (*self.tail).data;
                let mut current = self.tail;
                self.tail = (*current).prev;
                (*self.tail).next = 0 as *mut Node<T>;
                (*current).prev = 0 as *mut Node<T>;
                libc::free(current as *mut libc::c_void);
                Ok(data)
            }
        }
    }
}

impl<T> List<T> {
    pub fn unshift(&mut self, value: T) {
        unsafe {
            let node = libc::malloc(
                mem::size_of::<Node<T>>() as libc::size_t)
                as *mut Node<T>;
            ptr::write(node, Node {
                data: value,
                next: ptr::null_mut(),
                prev: ptr::null_mut()
            });

            if self.head.is_null() {
                self.head = node;
                self.tail = node;
            } else {
                (*self.head).prev = node;
                (*node).next = self.head;
                self.head = node;
            }
        }
    }
}

impl<T> List<T> where T: Copy {
    pub fn shift<'a>(&mut self) -> Result<T, &'a str> {
        unsafe {
            if self.head.is_null() {
                return Err("Empty list");
            }

            if self.head == self.tail {
                let data = (*self.head).data;
                self.head = 0 as *mut Node<T>;
                self.tail = 0 as *mut Node<T>;
                Ok(data)
            } else {
                let data = (*self.head).data;
                let mut current = self.head;
                self.head = (*current).next;
                (*self.head).prev = 0 as *mut Node<T>;
                (*current).next = 0 as *mut Node<T>;
                libc::free(current as *mut libc::c_void);
                Ok(data)
            }
        }
    }
}

impl<T> List<T> where T: Copy {
    pub fn at<'a>(&mut self, index: usize) -> Result<T, &'a str> {
        unsafe {
            let mut count = 0 as usize;
            let mut current = self.head;
            while !current.is_null() {
                if count == index {
                    return Ok((*current).data);
                }
                current = (*current).next;
                count += 1;
            }
            Err("Index out of range")
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.head;
            while !current.is_null() {
                self.head = (*current).next;

                if !self.head.is_null() {
                    (*self.head).prev = 0 as *mut Node<T>;
                    (*current).next = 0 as *mut Node<T>;
                    libc::free(current as *mut libc::c_void);
                }

                current = self.head;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_push_back() {
        let mut list = List::new() as List<i32>;
        assert_eq!(0, list.len());

        list.push(100);
        list.push(200);
        list.push(300);

        assert_eq!(3, list.len());
    }

    #[test]
    #[should_panic]
    fn test_at() {
        let mut list = List::new() as List<i32>;

        list.push(100);
        list.push(200);
        list.push(300);

        assert_eq!(100, list.at(0).unwrap());
        assert_eq!(300, list.at(2).unwrap());

        // Error!
        assert!(list.at(9999).unwrap() == 0);
    }

    #[test]
    #[should_panic]
    fn test_pop() {
        let mut list = List::new() as List<i32>;

        // Error!
        assert!(list.pop().unwrap() == 0);

        list.push(100);
        list.push(200);
        list.push(300);

        assert_eq!(300, list.pop().unwrap());
        assert_eq!(2, list.len());
    }

    #[test]
    fn test_unshift() {
        let mut list = List::new() as List<i32>;
        assert_eq!(0, list.len());

        list.unshift(100);
        list.unshift(200);
        list.unshift(300);

        assert_eq!(3, list.len());
    }

    #[test]
    fn test_shift() {
        let mut list = List::new() as List<i32>;

        list.unshift(100);
        list.unshift(200);
        list.unshift(300);

        assert_eq!(300, list.shift().unwrap());
        assert_eq!(2, list.len());
    }
}

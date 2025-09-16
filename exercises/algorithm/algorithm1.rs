/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /**
     * compare the heads of two lists and return the smaller one
     * and remove the head from the list
     * return the node_head_ptr and the node_tail_ptr
     */
    pub fn pick_maller_head(list_a: &mut LinkedList<T>, list_b: &mut LinkedList<T>) -> (NonNull<Node<T>>, NonNull<Node<T>>) {
        if unsafe {
			(*list_a.start.unwrap().as_ptr()).val <= (*list_b.start.unwrap().as_ptr()).val
		} {
            // list_a is smaller
            // list_a: [head_a] -> ... -> [tail_a]
            // =============================
            // head:   [head_a] -> None
            // list_a: [head_a + 1] -> ... -> [tail_a]
			let head = list_a.start.unwrap();
			let next: Option<NonNull<Node<T>>> = unsafe { (*head.as_ptr()).next };
            // remove the head from the list
			if next.is_some() {
				unsafe { (*head.as_ptr()).next = None };
			}
			list_a.start = next;
			list_a.length -= 1;
			(head, head)
		}
        else {
            // list_b is smaller
            // list_b: [head_b] -> ... -> [tail_b]
            // =============================
            // head:   [head_b] -> None
            // list_b: [head_b + 1] -> ... -> [tail_b]
            let head = list_b.start.unwrap();
            let next: Option<NonNull<Node<T>>> = unsafe { (*head.as_ptr()).next };
            if next.is_some() {
                unsafe { (*head.as_ptr()).next = None };
            }
            list_b.start = next;
            list_b.length -= 1;
            (head, head)
        }
    }

    /**
     * remove the head from the list
     * and update the length of the list
     */
    fn remove_head(list: &mut LinkedList<T>) {
        let next = unsafe { (*list.start.unwrap().as_ptr()).next };
        list.start = next;
        list.length -= 1;
    }

	pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self 
	where 
		T: PartialOrd 
	{
		// 处理边界情况
		if list_a.length == 0 {
			return list_b;
		}
		if list_b.length == 0 {
			return list_a;
		}

        let result_length = list_a.length + list_b.length;
        // 方法执行后，选择较小的头节点作为结果的头，并从原列表中删除该节点，返回头和尾
        // 头节点不需要调整，因为已经是最小节点
		let (result_head_ptr, mut result_tail_ptr) = LinkedList::pick_maller_head(&mut list_a, &mut list_b);

		// 继续合并剩余节点
		while let (Some(node_a), Some(node_b)) = (list_a.start, list_b.start) {
			let val_a = unsafe { &(*node_a.as_ptr()).val };
			let val_b = unsafe { &(*node_b.as_ptr()).val };

			if val_a <= val_b {
				// 将 node_a 从 list_a 中删除，并添加到结果链表的尾部
				unsafe { (*result_tail_ptr.as_ptr()).next = Some(node_a) };
				result_tail_ptr = node_a;
                LinkedList::remove_head(&mut list_a);
			} else {
				// 将 node_b 从 list_b 中删除，并添加到结果链表的尾部
				unsafe { (*result_tail_ptr.as_ptr()).next = Some(node_b) };
				result_tail_ptr = node_b;
                LinkedList::remove_head(&mut list_b);
			}
		}

		// 处理剩余的节点
		if let Some(remaining_a) = list_a.start {
            // tail 指向 list_a 的最后一个节点
			unsafe { (*result_tail_ptr.as_ptr()).next = Some(remaining_a) };
			result_tail_ptr = list_a.end.unwrap();
		}
		if let Some(remaining_b) = list_b.start {
            // tail 指向 list_b 的最后一个节点
			unsafe { (*result_tail_ptr.as_ptr()).next = Some(remaining_b) };
			result_tail_ptr = list_b.end.unwrap();
		}

		LinkedList {
			length: result_length, // +1 for the initial head
			start: Some(result_head_ptr),
			end: Some(result_tail_ptr),
		}
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
        assert_eq!(list_c.length, target_vec.len() as u32);
	}
}

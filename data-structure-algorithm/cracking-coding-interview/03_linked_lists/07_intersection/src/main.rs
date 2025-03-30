use question::NodeRef;
use rust_lib::linked_list_node::create_linked_list_from_array;

mod question;

//
// Th is algorithm takes O ( A + B ) time, where A and B a re the lengths of
// the two linked lists. It ta kes 0 ( 1 ) add itional space.
//

fn create_list() -> (Option<NodeRef>, Option<NodeRef>) {
    let vals = vec![-1, -2, 0, 1, 2, 3, 4, 5, 6, 7, 8];
    let list1 = create_linked_list_from_array(&vals);

    let vals2 = vec![12, 14, 15];
    let list2 = create_linked_list_from_array(&vals2);

    if let (Some(ref l1), Some(ref l2)) = (list1.clone(), list2.clone()) {
        // Get the last node of list2
        let mut tail2 = l2.clone();
        while tail2.borrow().next.is_some() {
            let next = tail2.borrow().next.clone(); // Extract next before mutating
            tail2 = next.unwrap();
        }

        // Find the intersection node at index 5 in list1 (the node containing `5`)
        if let Some(intersection_node) = question::get_kth_node(Some(l1.clone()), 5) {
            // Attach the last node of list2 to the node containing `5` in list1
            tail2.borrow_mut().next = Some(intersection_node);
        }
    }

    (list1, list2)
}

fn main() {
    let list = create_list();

    println!(
        "List 1: {}",
        list.0.as_ref().unwrap().borrow().print_forward()
    );
    println!(
        "List 2: {}",
        list.1.as_ref().unwrap().borrow().print_forward()
    );

    let res = question::find_intersection(list.0, list.1);

    if let Some(res_node) = res.clone() {
        println!(
            "found intersection: {}",
            res_node.as_ref().borrow().print_forward()
        );
    }
}

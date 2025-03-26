package main

import "fmt"

func main() {
	list := SinglyLinkedList{}
	list.Append(10)
	list.Append(20)
	list.Append(30)

	fmt.Println("Singly Linked List:")
	list.Display()
	// Output: 10 -> 20 -> 30 -> nil

	dlist := DoublyLinkedList{}
	dlist.Append(10)
	dlist.Append(20)
	dlist.Append(30)

	fmt.Println("Doubly Linked List (Forward):")
	dlist.DisplayForward()
	// Output: 10 <-> 20 <-> 30 <-> nil

	fmt.Println("Doubly Linked List (Backward):")
	dlist.DisplayBackward()
	// Output: 30 <-> 20 <-> 10 <-> nil
}

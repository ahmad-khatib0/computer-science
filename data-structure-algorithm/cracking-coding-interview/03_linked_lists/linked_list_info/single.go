package main

import "fmt"

// Node represents a node in the singly linked list
type Node struct {
	data int
	next *Node
}

// SinglyLinkedList represents the linked list
type SinglyLinkedList struct {
	head *Node
}

// Append adds a new node at the end
func (ll *SinglyLinkedList) Append(data int) {
	newNode := &Node{data: data}

	if ll.head == nil {
		ll.head = newNode
		return
	}

	current := ll.head
	for current.next != nil {
		current = current.next
	}
	current.next = newNode
}

// Delete removes the first occurrence of a node with matching data
func (ll *SinglyLinkedList) Delete(data int) {
	// Case 1: Empty list
	if ll.head == nil {
		return
	}

	// Case 2: Delete head node
	if ll.head.data == data {
		ll.head = ll.head.next
		return
	}

	// Case 3: Find and delete non-head node
	current := ll.head
	for current.next != nil {
		if current.next.data == data {
			current.next = current.next.next // Skip the node to delete
			return
		}
		current = current.next
	}
}

// Display prints the linked list
func (ll *SinglyLinkedList) Display() {
	current := ll.head
	for current != nil {
		fmt.Printf("%d -> ", current.data)
		current = current.next
	}
	fmt.Println("nil")
}

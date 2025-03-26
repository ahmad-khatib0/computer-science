package main

import "fmt"

// DNode represents a node in the doubly linked list
type DNode struct {
	data int
	prev *DNode
	next *DNode
}

// DoublyLinkedList represents the doubly linked list
type DoublyLinkedList struct {
	head *DNode
	tail *DNode
}

// Append adds a new node at the end
func (dll *DoublyLinkedList) Append(data int) {
	newNode := &DNode{data: data}

	if dll.head == nil {
		dll.head = newNode
		dll.tail = newNode
		return
	}

	newNode.prev = dll.tail
	dll.tail.next = newNode
	dll.tail = newNode
}

// Delete removes the first occurrence of a node with matching data
func (dll *DoublyLinkedList) Delete(data int) {
	// Case 1: Empty list
	if dll.head == nil {
		return
	}

	// Case 2: Delete head node
	if dll.head.data == data {
		dll.head = dll.head.next
		if dll.head != nil {
			dll.head.prev = nil
		} else {
			dll.tail = nil // List became empty
		}
		return
	}

	// Case 3: Find and delete non-head node
	current := dll.head
	for current != nil {
		if current.data == data {
			// Update previous node's next pointer
			if current.prev != nil {
				current.prev.next = current.next
			}

			// Update next node's prev pointer
			if current.next != nil {
				current.next.prev = current.prev
			} else {
				// We're deleting the tail
				dll.tail = current.prev
			}
			return
		}
		current = current.next
	}
}

// DisplayForward prints the list from head to tail
func (dll *DoublyLinkedList) DisplayForward() {
	current := dll.head
	for current != nil {
		fmt.Printf("%d <-> ", current.data)
		current = current.next
	}
	fmt.Println("nil")
}

// DisplayBackward prints the list from tail to head
func (dll *DoublyLinkedList) DisplayBackward() {
	current := dll.tail
	for current != nil {
		fmt.Printf("%d <-> ", current.data)
		current = current.prev
	}
	fmt.Println("nil")
}

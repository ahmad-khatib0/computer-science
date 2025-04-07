use approach::{Animal, AnimalQueue, AnimalType, Cat, Dog};

mod approach;

//
// Animal Shelter: An animal shelter, which holds only dogs and cats, operates on a
// strictly "first in, first out " basis. People must adopt either the "oldest"
// (based on arrival time) of all animals at the shelter, or they can select whether
// they would prefer a dog or a cat (and will receive the old est animal of that type).
// They can not select which specific animal they would like. Create the data structures
// to maintain this system and implement operations such as enqueue, dequeueAny, dequeueDog,
// and dequeueCat. You may use the built-in Linked list data structure.
//

fn main() {
    let mut animals = AnimalQueue::new();

    animals.enqueue(AnimalType::Cat(Cat::new("Callie")));
    animals.enqueue(AnimalType::Cat(Cat::new("Kiki")));
    animals.enqueue(AnimalType::Dog(Dog::new("Fido")));
    animals.enqueue(AnimalType::Dog(Dog::new("Dora")));
    animals.enqueue(AnimalType::Cat(Cat::new("Kari")));
    animals.enqueue(AnimalType::Dog(Dog::new("Dexter")));
    animals.enqueue(AnimalType::Dog(Dog::new("Dobo")));
    animals.enqueue(AnimalType::Cat(Cat::new("Copa")));

    println!("{}", animals.dequeue_any().unwrap().name());
    println!("{}", animals.dequeue_any().unwrap().name());
    println!("{}", animals.dequeue_any().unwrap().name());

    animals.enqueue(AnimalType::Dog(Dog::new("Dapa")));
    animals.enqueue(AnimalType::Cat(Cat::new("Kilo")));

    while animals.size() > 0 {
        println!("{}", animals.dequeue_any().unwrap().name());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue_any() {
        let mut animals = AnimalQueue::new();

        // Enqueue animals
        animals.enqueue(AnimalType::Cat(Cat::new("Callie")));
        animals.enqueue(AnimalType::Cat(Cat::new("Kiki")));
        animals.enqueue(AnimalType::Dog(Dog::new("Fido")));

        // Test dequeue order (oldest first)
        assert_eq!(animals.dequeue_any().unwrap().name(), "Cat: Callie");
        assert_eq!(animals.dequeue_any().unwrap().name(), "Cat: Kiki");
        assert_eq!(animals.dequeue_any().unwrap().name(), "Dog: Fido");
    }

    #[test]
    fn test_mixed_operations() {
        let mut animals = AnimalQueue::new();

        animals.enqueue(AnimalType::Dog(Dog::new("Dora")));
        animals.enqueue(AnimalType::Cat(Cat::new("Kari")));
        animals.enqueue(AnimalType::Dog(Dog::new("Dexter")));

        assert_eq!(animals.size(), 3);

        // Dequeue and verify
        assert_eq!(animals.dequeue_any().unwrap().name(), "Dog: Dora");
        assert_eq!(animals.size(), 2);

        // Add more and test
        animals.enqueue(AnimalType::Cat(Cat::new("Copa")));
        assert_eq!(animals.dequeue_any().unwrap().name(), "Cat: Kari");
    }

    #[test]
    fn test_empty_queue() {
        let mut animals = AnimalQueue::new();
        assert!(animals.dequeue_any().is_none());
        assert_eq!(animals.size(), 0);
    }

    #[test]
    fn test_animal_ordering() {
        let mut cat1 = Cat::new("Callie");
        let mut cat2 = Cat::new("Kiki");

        cat1.set_order(1);
        cat2.set_order(2);

        assert!(cat1.is_older_than(&cat2));
    }

    #[test]
    fn test_type_specific_dequeue() {
        let mut animals = AnimalQueue::new();
        animals.enqueue(AnimalType::Dog(Dog::new("Fido")));
        animals.enqueue(AnimalType::Cat(Cat::new("Callie")));

        // Dequeue Dog directly (returns Option<Dog>)
        let dog = animals.dequeue_dogs().unwrap();
        assert_eq!(dog.name(), "Dog: Fido");

        // Dequeue Cat directly (returns Option<Cat>)
        let cat = animals.dequeue_cats().unwrap();
        assert_eq!(cat.name(), "Cat: Callie");
    }
}

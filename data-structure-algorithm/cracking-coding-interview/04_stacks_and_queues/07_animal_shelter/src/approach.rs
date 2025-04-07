use std::collections::VecDeque;

pub trait Animal {
    fn name(&self) -> String;
    fn order(&self) -> usize;
    fn set_order(&mut self, order: usize);
    fn is_older_than(&self, other: &dyn Animal) -> bool {
        self.order() < other.order()
    }
}

// Animal types as structs with enum discriminant
#[derive(Debug)]
pub enum AnimalType {
    Dog(Dog),
    Cat(Cat),
}

impl Animal for AnimalType {
    fn name(&self) -> String {
        match self {
            AnimalType::Dog(d) => d.name(),
            AnimalType::Cat(c) => c.name(),
        }
    }

    fn order(&self) -> usize {
        match self {
            AnimalType::Dog(d) => d.order,
            AnimalType::Cat(c) => c.order,
        }
    }

    fn set_order(&mut self, order: usize) {
        match self {
            AnimalType::Dog(d) => d.order = order,
            AnimalType::Cat(c) => c.order = order,
        }
    }
}

// Concrete animal types
#[derive(Debug)]
pub struct Dog {
    name: String,
    order: usize,
}

impl Dog {
    pub fn new(name: &str) -> Self {
        Dog {
            name: name.to_string(),
            order: 0,
        }
    }
}

impl Animal for Dog {
    fn name(&self) -> String {
        format!("Dog: {}", self.name)
    }

    fn order(&self) -> usize {
        self.order
    }

    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

#[derive(Debug)]
pub struct Cat {
    name: String,
    order: usize,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        Cat {
            name: name.to_string(),
            order: 0,
        }
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        format!("Cat: {}", self.name)
    }

    fn order(&self) -> usize {
        self.order
    }

    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

pub struct AnimalQueue {
    dogs: VecDeque<Dog>,
    cats: VecDeque<Cat>,
    order: usize,
}

impl AnimalQueue {
    pub fn new() -> Self {
        AnimalQueue {
            dogs: VecDeque::new(),
            cats: VecDeque::new(),
            order: 0,
        }
    }

    pub fn enqueue(&mut self, animal: AnimalType) {
        let mut animal = animal;
        animal.set_order(self.order);
        self.order += 1;

        match animal {
            AnimalType::Dog(d) => self.dogs.push_back(d),
            AnimalType::Cat(c) => self.cats.push_back(c),
        }
    }

    pub fn dequeue_any(&mut self) -> Option<AnimalType> {
        match (self.dogs.front(), self.cats.front()) {
            (Some(dog), Some(cat)) => {
                if dog.is_older_than(cat) {
                    self.dogs.pop_front().map(AnimalType::Dog)
                } else {
                    self.cats.pop_front().map(AnimalType::Cat)
                }
            }
            (Some(_), None) => self.dogs.pop_front().map(AnimalType::Dog),
            (None, Some(_)) => self.cats.pop_front().map(AnimalType::Cat),
            (None, None) => None,
        }
    }

    pub fn size(&self) -> usize {
        self.dogs.len() + self.cats.len()
    }

    pub fn dequeue_dogs(&mut self) -> Option<Dog> {
        self.dogs.pop_front()
    }

    pub fn dequeue_cats(&mut self) -> Option<Cat> {
        self.cats.pop_front()
    }
}

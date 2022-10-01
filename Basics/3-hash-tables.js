//Hash tables

let user = {
  age: 55,
  name: "Keylie",
  magic: true,
  scream() {
    console.log("ahhhhhhhhh");
  },
};
user.age; //O(1)
user.spell = "abra kadabra"; //O(1)

//when you have collision with hash tables, the writting and reading is: O(n/k) where the k is the size of the table

// hash table cost
(inserting) => O(1);
(lookup) => O(1);
(deleting) => O(1);
(searcing) => O(1);
//deleting is o(1) rather than O(n) in arrays because we don't have to reorder the table after deleting,

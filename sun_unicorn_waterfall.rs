//1
pub use std::string;

//2
pub struct Backpacker {
    id: u32,
    name: string::String,
    age: u8,
    location: string::String,
    travels: Vec<string::String>,
}

//3
impl Backpacker {
    pub fn id(&self) -> u32 {
        self.id
    }

//4
    pub fn name(&self) -> &string::String {
        &self.name
    }

//5
    pub fn age(&self) -> u8 {
        self.age
    }

//6
    pub fn location(&self) -> &string::String {
        &self.location
    }

//7
    pub fn travels(&self) -> &Vec<string::String> {
        &self.travels
    }

//8
    pub fn set_name(&mut self, name: string::String) {
        self.name = name;
    }

//9
    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

//10
    pub fn set_location(&mut self, location: string::String) {
        self.location = location;
    }

//11
    pub fn add_travel(&mut self, travel: string::String) {
        self.travels.push(travel);
    }
}

//12
impl Backpacker {
   pub fn new(id: u32,
              name: string::String,
              age: u8,
              location: string::String,
              travels: Vec<string::String>) -> Backpacker {
        Backpacker{id: id, 
                   name: name, 
                   age: age, 
                   location: location, 
                   travels: travels}
    }
}
    
//13
pub fn find_backpackers(max_age: u8) -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to find backpackers would go here

    backpackers
}

//14
pub fn find_backpackers_with_id(id: u32) -> Option<Backpacker> {
    let mut backpacker: Option<Backpacker> = None;

    // Logic to find backpacker with id would go here

    backpacker
}

//15
pub fn remove_backpacker_with_id(backpackers: &mut Vec<Backpacker>, id: u32) {
    let index = backpackers.iter().position(|val| val.id == id);
    if let Some(index) = index {
        backpackers.remove(index);
    }
}

//16
pub fn update_backpacker_name(backpackers: &mut Vec<Backpacker>, id: u32, name: string::String) {
    let index = backpackers.iter().position(|val| val.id == id);
    if let Some(index) = index {
        let mut backpacker = &mut backpackers[index];
        backpacker.set_name(name);
    }
}

//17
pub fn update_backpacker_age(backpackers: &mut Vec<Backpacker>, id: u32, age: u8) {
    let index = backpackers.iter().position(|val| val.id == id);
    if let Some(index) = index {
        let mut backpacker = &mut backpackers[index];
        backpacker.set_age(age);
    }
}

//18
pub fn update_backpacker_location(backpackers: &mut Vec<Backpacker>, 
                                  id: u32, 
                                  location: string::String) {
    let index = backpackers.iter().position(|val| val.id == id);
    if let Some(index) = index {
        let mut backpacker = &mut backpackers[index];
        backpacker.set_location(location);
    }
}

//19
pub fn add_backpacker_travel(backpackers: &mut Vec<Backpacker>, 
                             id: u32, 
                             travel: string::String) {
    let index = backpackers.iter().position(|val| val.id == id);
    if let Some(index) = index {
        let mut backpacker = &mut backpackers[index];
        backpacker.add_travel(travel);
    }
}

//20
pub fn get_backpackers() -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to get backpackers would go here

    backpackers
}

//21
pub fn save_backpackers(backpackers: &mut Vec<Backpacker>) {
    // Logic to save backpackers would go here
}

//22
pub fn update_backpackers(backpackers: &mut Vec<Backpacker>) {
    // Logic to update backpackers would go here
}

//23
pub fn delete_backpackers(backpackers: &mut Vec<Backpacker>) {
    // Logic to delete backpackers would go here
}

//24
pub fn get_backpacker_with_id(backpackers: &mut Vec<Backpacker>, id: u32) -> Option<Backpacker> {
    let mut backpacker: Option<Backpacker> = None;

    // Logic to get backpacker with id would go here

    backpacker
}

//25
pub fn get_backpackers_with_age(backpackers: &mut Vec<Backpacker>, age: u32) -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to get backpackers with age would go here

    backpackers
}

//26
pub fn get_backpackers_with_location(backpackers: &mut Vec<Backpacker>, 
                                     location: string::String) -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to get backpackers with location would go here

    backpackers
}

//27
pub fn get_backpackers_with_travel(backpackers: &mut Vec<Backpacker>, 
                                   travel: string::String) -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to get backpackers with travel would go here

    backpackers
}

//28
pub fn get_backpacker_total_travels(backpackers: &mut Vec<Backpacker>) -> Vec<(Backpacker, u32)> {
    let mut backpackers_travels = Vec::new();

    // Logic to get backpacker total travels would go here

    backpackers_travels
}

//29
pub fn get_travels_with_backpackers(backpackers: &mut Vec<Backpacker>) -> Vec<string::String> {
    let mut travels = Vec::new();

    // Logic to get travels with backpackers would go here

    travels
}

//30
pub fn get_backpackers_with_travels_over(backpackers: &mut Vec<Backpacker>, 
                                         min_travels: u32) -> Vec<Backpacker> {
    let mut backpackers = Vec::new();

    // Logic to get backpackers with travels over would go here

    backpackers
}
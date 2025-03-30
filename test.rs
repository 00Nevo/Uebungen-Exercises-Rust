//some code from ChatGpt because the repository is not showing as a rust repo despite having already a .rs file in it...

// Define a struct to represent a Person
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Method to create a new Person
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    // Method to print a greeting based on the person's name
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

// Define an enum for different languages
enum Language {
    English,
    Spanish,
    French,
}

// Implement the Language enum with a method to return a greeting
impl Language {
    fn greet_in_language(&self, person: &Person) {
        match self {
            Language::English => println!("Hello, {}!", person.name),
            Language::Spanish => println!("¡Hola, {}!", person.name),
            Language::French => println!("Bonjour, {}!", person.name),
        }
    }
}

fn main() {
    // Create a new person
    let person = Person::new("Alice", 30);
    
    // Greet the person
    person.greet();
    
    // Create an instance of the Language enum
    let lang = Language::Spanish;

    // Greet the person in Spanish
    lang.greet_in_language(&person);
    
    // Change language and greet in French
    let lang_french = Language::French;
    lang_french.greet_in_language(&person);
}

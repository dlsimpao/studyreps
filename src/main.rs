use std::collections::HashMap;

fn create_flashcard(deck: &mut HashMap<String, String>, term: String, definition: String) -> &mut HashMap<String, String>{
    deck.insert(term, definition);
    deck
}

fn main() {
    let mut deck = HashMap::new();
    create_flashcard(&mut deck, String::from("What is the powerhouse of the cell?"), String::from("Mitochondria"));
    create_flashcard(&mut deck, String::from("What is the powerhouse of the cell2?"), String::from("Mitochondria"));
    create_flashcard(&mut deck, String::from("What is the powerhouse of the cell3?"), String::from("Mitochondria"));
    create_flashcard(&mut deck, String::from("What is the powerhouse of the cell4?"), String::from("Mitochondria"));
    
    for i in range(deck){
        println!("{:?}", deck);
    }
}

#![allow(warnings)]

use std::io::stdin; 
use std::collections::HashMap;
use std::str::Bytes;
use lazy_static::lazy_static;

//lazy_static!{
//    static ref HASHMAP: HashMap<String, Book> = { let mut m = HashMap::new(); m};
//}

#[derive(Clone,PartialEq)]
struct Book{
    Title:String,
    pages:i32
}

#[derive(PartialEq)]
enum  OPERATIONS{
    add,
    remove,
    change,
    show,
    end
}


fn main() {
    
    println!("Hemlo, this dumb rust ,library, u add book i store book");
    // TODO add menu

    let mut m:HashMap<String, Book> = HashMap::new();
    let mut operation : OPERATIONS = getOPERATION();

    while operation != OPERATIONS::end{

        match operation {
            OPERATIONS::add => {
                        let ref newBook = createBook();
                        m.insert(newBook.Title.to_string(), newBook.clone());
                    },
            OPERATIONS::remove => {
                        let bookToRemove = getInput("Input the name of book to delte");
                        m.remove(&bookToRemove);
                    },
            OPERATIONS::change =>{
                        changeBook(m.clone());
            }
            OPERATIONS::show => {
                        showAllBooks(m.clone());
            }

            OPERATIONS::end => break,
            _ => println!("Invalid oppewation, dumb...")
        }

        operation = getOPERATION();

    }

}

fn createBook() -> Book{

    let name = getInput("Input the Book name");
    let page_number:i32 = getInput("Input page Number").parse().unwrap();
    let newBook =  Book{ Title: name, pages : page_number };

    return newBook;

}

fn changeBook(map : HashMap<String, Book>) -> HashMap<String,Book>{

    let name = getInput("Input name of book to change");
    let newPageNumber = getInput("Input new Page number").parse().unwrap();

    let current_book = map.get(&name);

    if current_book == None{
        println!("Not a book in the libray, books are dumb"); return map;   
    }

    let mut newMap = map.clone();
    let title :String = current_book.unwrap().Title.clone();
    newMap.insert(title, Book{Title : current_book.unwrap().Title.clone(), pages: newPageNumber});

    return newMap;
}

fn showAllBooks(map : HashMap<String, Book>) {



    for (key, value) in map.into_iter() {
        println!("Book is {} with this many pages {}", key, value.pages);
    }
    
}

fn getOPERATION() -> OPERATIONS{
    let mut line = String::new();
    println!("Impute the opeariotn according to the meniu");
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    let value : u32 = line.parse().unwrap();

    match value {
        1 => return OPERATIONS::add,
        2 => return OPERATIONS::remove,
        3 => return OPERATIONS::change,
        4 => return OPERATIONS::show,
        5 => return OPERATIONS::end,
        _ => return OPERATIONS::end
    }
}

fn getInput(entry_string:&str) -> String{
    println!("{}",entry_string);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}
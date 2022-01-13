extern crate rand;

use std::fmt;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

fn back_on_15 (n1:i32) -> (){
    if n1 % 15 == 0{
        println!("FizzBuzz")
    }else if(n1 % 3 == 0){
        println!("Fizz")
    }else{
        println!("Buzz")
    }
}

fn should_be_true_or_false(n2:i32) -> bool{
    if n2 % 15 == 0{
        true
    }else if(n2 % 3 == 0){
        true
    }else{
        false
    }
}


fn setted_on_a_for_loop(n:i32) ->(){
    for n in 1..=n{
        println!("{}", n);
    }
}


fn main() {

    //Declaring a String, Declaring a number

    let number: i32 = 10;
    let float_number: f32 = 10.0;
    let new_str = "Matteo";
    let converted_new_str_in_string = new_str.to_string();
    let set_a_new_string = String::new();

    //Can even set a variable and then assign it
    let my_not_setted_variable: i32;
    my_not_setted_variable = 5;


    //Set a array, vector...Difference are array are not mutable, vector are mutable

    let mut new_vector = Vec::new();
    new_vector.push(7);
    println!("My Vector is {:?}", new_vector);
    let my_array = [1, 2, 3, 4, 5, 6];
    let my_array2: [i32; 5] = [1, 2, 3, 4, 5];
    let mut my_vector = vec![1, 2, 3, 4, 5, 6];
    my_vector.push(7);


    //Some method to convert
    let new_number: i32 = 30;
    let convert_new_number = new_number as i8;
    let a_string = "Hello";
    let converted_a_string = a_string.to_string();


    //Convert all the types in a String
    struct Anumber {
        number: i32
    }
    // Using this standard implementation, we don't have to convert our number in a String because implementation,
    // have already the method .to_string()...Try to delete it to check
    impl fmt::Display for Anumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.number)
        }
    }

    let a_number = Anumber { number: 10 };
    println!("{}", a_number);
    println!("");


    //formatted print
    println!("{a_name} years {a_number}", a_name = "Matteo", a_number = 30);
    println!("");


    //create space from left margin
    println!("{new_number:>width$}", new_number = 10, width = 6);
    println!("");


    //Only 2 decimals from a float number
    let float_number: f32 = 1.2345567889;
    println!("{:.2}", float_number);
    println!("");


    //binary
    println!("{:b}", new_number);
    println!("");


    //Hex
    println!("{:x}", new_number);
    println!("");


    //Octal
    println!("{:o}", new_number);
    println!("");


    //For loop for a number
    for i in 1..10 {
        println!("{}", i);
    }
    println!("");


    //For loop for a string. Can iterate letters one by one
    let used_for_for = "Hello boys and girls";
    for i in used_for_for.chars() {
        println!("{}", i);
    }
    println!("");


    //We can iterate on split and print a new line, everytime we found a ' ' in our string
    for i in used_for_for.split(' ') {
        println!("{}", i);
    }
    println!("");


    //Take a slice of that

    let my_string_slice = &used_for_for[1..5];
    println!("{}", my_string_slice);
    println!("");


    //We can do the same for a vector or an array

    let my_newest_array = [1, 2, 3, 4, 5, 6, 7];
    let mut my_newest_vector = vec![10, 11, 12, 13, 14, 15];

    for i in my_newest_array {
        println!("{:?}", i);
    }
    println!("");

    let my_newest_vector_slice = &my_newest_vector[2..4];
    println!("{:?}", my_newest_vector_slice);
    println!("");


    //Set a Tuple and print the elements

    let my_first_tuple = ("Rust", 2022);
    let my_second_tuple = ("Satellite", 2022);
    println!("{}", my_first_tuple.0);
    println!("{}", my_second_tuple.1);
    println!("");


    //if, else if and else usage

    let mut if_else_number = 0;

    for i in if_else_number..15 {
        if i < 10 && i > 3 {
            if_else_number += 1;
            println!("And Numbers {}", i);
        } else if (i == 3) || (i == 12) {
            println!("Or Numbers {}", i);
        } else if (i == 14) {
            println!("STOP");
        } else {
            println!("Others {}", i);
        }
    }
    println!("");


    //loop usage

    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("Skipped");
            continue;
        } else if (count == 5) {
            println!("Skipped Again");
            break;
        }
        println!("Count is {}", count);
    }
    println!("");

    //Something about match

    let match_number = 20;

    match match_number {
        20 => { println!("Thats the  number") },
        _ => { println!("Nothing") },
    }
    println!("");

    //or maybe something a bit more complex is

    for i in 0..10{
        match i {
            5 => { println!("Hey it's five") },
            2 | 6 | 8 => { println!("{}", i) },
            _ => { println!("Number {}", i) },
        }
    }
    println!("");


    //Something about while

    let mut new_while_number = 0;

    while new_while_number < 20 {
        if new_while_number < 10 {
            println!("Under 10");
        } else {
            println!("Over 10");
        }
        new_while_number += 1;
    }
    println!("");

    // if let
    //Go deep in the documentation by holding ctrl, and click on Option. The documentation say
    // "Returns true if the option is a [Some] value."
    //So we can work on this implementation passing a type like this.
    //Then we use the if let Some() and we pass the variable to check.
    //Working as a match

    let option_number = Some(10);
    let letter: Option<i32> = None;

    if let Some(i) = option_number {
        println!("Match");
    } else {
        println!("Unmatched");
    }

    if let Some(i) = letter {
        println!("Yesss It's a match {}", i);
    } else {
        println!("Nothing to Do");
    }

    println!("");

    //while let

    let mut while_let_number = Some(0);

    while let Some(i) = while_let_number {
        if i == 5 {
            println!("Hey is five");
            while_let_number = None;
        } else if (i == 3) || (i == 4) {
            println!("Are three and four");
            while_let_number = None;
        } else {
            println!("Number => {}", i);
            while_let_number = Some(i + 1);
        }
    }
    println!("");


    //Using function for FizzBuzz example.....functions are setted on top

    let first_of_back_on_15_function = 15;
    let second_of_back_on_15_function = 9;
    let third_of_back_on_15_function = 1;

    back_on_15(first_of_back_on_15_function);       //We don't need of println!() because is already setted on the function.
    back_on_15(second_of_back_on_15_function);      // See the difference between this one and the other one below
    back_on_15(third_of_back_on_15_function);
    println!("");


    //Or we can use a boolean for example, as result of the function

    let first_of_function_boolean_example = 3;
    let second_of_function_boolean_example = 15;                //Here we need a println!() to make it work.
    let third_of_function_boolean_example = 11;
    println!("{}", should_be_true_or_false(first_of_function_boolean_example));
    println!("{}", should_be_true_or_false(second_of_function_boolean_example));
    println!("{}", should_be_true_or_false(third_of_function_boolean_example));
    println!("");


    // or we can use a function setted with a for loop like this

    setted_on_a_for_loop(15);
    println!("");


    //Closure are very helpfully function and can take value from elements.
    //Base Sintax is simple, first of all declare closure as a variable, then |i: type | -> type{ i }
    //Or we can even declare our closure like: variable = |i    |   i;    In this case i will be setted to the appropriate type

    let closure_number = 1;
    let closure = |i: i32| -> i32{ i + 1 };
    println!("My Closure number is {}", closure(closure_number));
    println!("");


    //Bit complex example...In this case we are going to iterate into a range of 1:100 and if, number is equal to our variables we print a message
    //To make a prove of this delete the comment from the second print.

    let my_closure_number = 25;
    let second_closure_number = 32;
    for i in 1..100 {
        // declaring our types
        let my_new_closure = |i: i32| -> i32{ i };

        //No declaring types
        let my_second_closure_number = |i| i + 1;
        if i == my_closure_number || i == second_closure_number {
            println!("My Closure number is {}", my_new_closure(my_closure_number));
            println!("");
            println!("My second closure number is {}", my_second_closure_number(i));
        }//else{println!("{}", i);}
    }
    println!("");


    /*
    EX1: Create a program that ask 2 input to the customer, name and age, then print a message that will tell them the year of them 100' birthday

    //Extra1
    Ask a third number and print the same number for the number give

     */

    let mut tell_me_name = String::new();
    println!("Hey Tell me your name");
    std::io::stdin().read_line(&mut tell_me_name).unwrap();
    println!("");

    let mut tell_me_age = String::new();
    println!("Now tell me your age");
    std::io::stdin().read_line(&mut tell_me_age).unwrap();
    let convert_tell_me_age: i32 = tell_me_age.trim().parse().unwrap();
    println!("");

    let hundred = 100;
    let now_date = 2022;

    let result = hundred - convert_tell_me_age + now_date;
    println!("Hey {} you'll be 100 in {}", tell_me_name, result);
    println!("");

    let mut tell_me_third = String::new();
    println!("tell me a new number ");
    std::io::stdin().read_line(&mut tell_me_third).unwrap();
    let convert_tell_me_third = tell_me_third.trim().parse().unwrap();

    for i in 0..convert_tell_me_third {
        println!("{}", convert_tell_me_third);
    }
    println!("");


    /*
    EX2: Create a program that ask 1 input to the customer, and print if the number is odd or even

    //Extra1
    If number is a multiple of 4, print a different message

    //Extra2
    set a new number call it check. if first number is divided perfectly for check print a message

     */

    let mut new_number = String::new();
    println!("Hey give a number");
    std::io::stdin().read_line(&mut new_number).unwrap();
    let convert_new_number: i8 = new_number.trim().parse().unwrap();

    //how to change a word into a variable, we don't need it for this exercise but just to et you know how it works
    let convert_string_newnumber = "Hey give a number";
    let adjusted_string = convert_string_newnumber.replace("a", "us");
    println!("{}", adjusted_string);


    //We don't really need of closure but it's just for understand how they works
    if convert_new_number % 2 == 0 {
        let my_own_closure = |i: i8| -> i8 { i + 1 };
        println!("My Own Closure is {} ", my_own_closure(convert_new_number));
        println!("Number {} is even", convert_new_number);

        //println!("It's even");
    }

    if (convert_new_number % 4 == 0) {
        let my_second_own_closure = |i: i8| -> i8 { i - 1 };
        println!("My Second Own Closure is {} ", my_second_own_closure(convert_new_number));
        println!("Number {} is divisible for 4", convert_new_number);
    }

    let check = 4;
    if convert_new_number % check == 0 {
        println!("It's even compatible with check number...PERFECT!");
    }
    println!();


    //EX3: Set The Today day into a struct and then print it

    struct MyStructure {
        day: i8,
        month: i8,
        year: i16
    }

    impl fmt::Display for MyStructure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {} {}", self.day, self.month, self.year)
        }
    }

    let first_element_of_mystructure = 12;
    let second_element_of_mystructure = 1;
    let third_element_of_mystructure = 2022;

    let result = MyStructure { day: first_element_of_mystructure, month: second_element_of_mystructure, year: third_element_of_mystructure };
    println!("Today Date is {}", result);
    println!("");


    //EX4: Take a list, let a = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89] and write a program that will print out all the numbers, less than 5
    /*
    Extra1 : instead printing elements one by one create a new list a print all togheter

    Extra2: Ask user a number and print all the number of the array that are smaller of that.

    Extra3: Iterate with .iter(), into_iter and iter_mut

     */

    let my_new_array = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let mut my_new_vector = Vec::new();

    for i in my_new_array {
        if i < 5 {
            my_new_vector.push(i);
            println!("Numbers under 5 in array are {}", i);
        }
    }
    println!("This is the newest array {:?}", my_new_vector);
    println!("");

    let mut ask_number_ex4 = String::new();
    println!("Ok Give me a number ");
    std::io::stdin().read_line(&mut ask_number_ex4).unwrap();
    let convert_ask_number_ex4 = ask_number_ex4.trim().parse().unwrap();

    let mut my_second_vector = Vec::new();

    for i in my_new_array {
        if i < convert_ask_number_ex4 {
            my_second_vector.push(i);
        }
    }

    println!("This is are all the value, smaller than the input, in the array {:?}", my_second_vector);
    println!("");

    //We can iterate in different ways, in this ways we can use the native collection later, like this

    for i in my_new_vector.iter() {
        match i {
            3 => { println!("That's three") },
            _ => { println!("{}", i) }
        }
    }
    println!("");
    println!("{:?}", my_new_vector);
    println!("");

    //In this way we are going to kill our native collections

    for i in my_new_vector.into_iter() {
        match i {
            3 => { println!("We got three") },
            _ => { println!("{}", i) }
        }
    }
    //println!("{:?}", my_new_vector);
    println!("");

    //Or we can use a iter_mut to change a value into the native collections.

    let mut a_new_vector = vec!["Sam", "Kelly", "Tom"];
    for i in a_new_vector.iter_mut() {
        match i {
            &mut "Sam" => { println!("Lela") },
            _ => { println!("{}", i) }
        }
    }
    println!("");


    //Using modules...Keyword is mod, Modules are entire part of code, by default are private an can be moved inside the program.
    //That means, Modules are a collections of elements: functions, impl, struct ... by default are private, but the elements inside should be
    //called from outside, by setting the elements inside the module as pub.
    //This allow to the elements to be called from outside. As you can see from the example below, we just setted 2 functions and a struct
    //First function is not pub, so you cannot call it from outside, instead the other function and the struct are setted as pub.
    //Note just the last thing about struct...struct have one more level of visibility for the elements declared inside.
    //So even the elements inside the struct must to be setted as pub

    mod my_mod {
        // Items in modules default to private visibility.
        fn private_function() {
            println!("Will not be printed but you can call from outside by `my_mod::private_function()`");
        }

        pub fn not_private_functions() {
            println!("You can see this because is pub ")
        }

        #[derive(Debug)]
        pub struct MyPubStruct {
            pub x: i32,
            pub y: String
        }
    }

    //Now to call from outside, you can use this
    my_mod::not_private_functions();
    //my_mod::private_function();
    println!("");

    let my_mod_number: i32 = 10;
    let my_mod_string = "hello".to_string();

    println!("{:?}", my_mod::MyPubStruct { x: my_mod_number, y: my_mod_string });

    println!("");


    //Random number, set rand in cargo.toml[dependencies] and use rand::Rng on top
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    println!("");



    //self and super for modules are super helpfully,
    //because they can remove ambiguity when accessing items
    fn my_self_and_super_outside_function(){
        println!("Outside Function");
    }

    mod my_new_module{
        pub fn my_self_and_super_first_function(){
            println!("We are inside module, you can call this via my_new_module::my_self_and_super_first_function");
        }
        pub mod my_second_new_module{
            pub fn my_self_and_super_second_function(){
                println!("Now we are nested into the first mod, and yo can call this via my_new_module::my_second_new_module::my_self_and_super_second_function")
            }

            pub fn indirect_calling_with_self_and_super(){
                // self refers to the local scope, in fact with self,we can call the functions inside the
                //same module and not outside....like this.
                self::my_self_and_super_second_function();
                //Different from super, in fact super refers to the global scope,and we can call with
                //this method only th function out of the module
                super::my_self_and_super_first_function();
            }
        }
    }

    my_new_module::my_self_and_super_first_function();
    my_new_module::my_second_new_module::my_self_and_super_second_function();
    println!("");
    //Now you can call the elements from self and super like this
    my_new_module::my_second_new_module::indirect_calling_with_self_and_super();
    println!("");




    //Some standard library

    let standard_library_string = "Hello Boys and Girls, Welcome in Rust".to_string();
    println!("{}", standard_library_string);


    //Iterate for the space and reverse
    for x in standard_library_string.split_whitespace().rev(){
        println!("{}", x);
    }
    println!("");
    //Same of this
    for x in standard_library_string.split(' ').rev(){
        println!("{}", x);
    }
    println!("");



    //Change a part of a string
    let change_string: &'static str = "Hey You, you are hey high?";
    let new_changed_string = change_string.replace("hey", "too");
    println!("{}", change_string);
    println!("");
    println!("{}", new_changed_string);
    println!("");


    //Sometimes we just need to relevate error in our programs instead call panic! , so we can do this
    // using l'Option enum....
    // Options<T> have 2 variables, Some and None
    //None indicate the failure, Some indicate the truth....
    //We can use it like this

    fn checked_division(n1:i32 , n2:i32)-> Option<i32>{
        if n2 == 0{
            None
        }else{
          Some(n1/n2)
        }
    }

    //Something really interesting is HashMap.
    //HashMap store value by keys, and keys should be boolean, integers, String... Just f32 and f64 are not allowed
    //To declare a new HashMap we can do HashMap::new() .
    //We can work in this way


    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("Matteo", "33323455");
    my_hashmap.insert("Danilo", "32783925");
    my_hashmap.insert("Roberta", "333123445");
    my_hashmap.insert("Silvio", "348765454");
    my_hashmap.insert("Filomena", "327098765");

    //match a HashMap
    match my_hashmap.get(&"Matteo"){
         Some(&number)=> {println!("Matteo Number is {:?}", my_hashmap.get("Matteo"))},
        _ => {println!("We don't have Matteo")}
    }
    println!("");


    //to Remove something from the HashMap
    my_hashmap.remove(&"Matteo");
    println!("Matteo is here? {:?}", my_hashmap.get(&"Matteo"));
    println!("");


    //Iterate in a HashMap
    for(x, y) in my_hashmap.iter(){
        println!("Name: {}, Number: {}", x, y);
    }
    println!("");



    //EXERCISE: Ask for a new username and a password and set it into a HashMap then print it



    let mut first_question_hashmap = String::new();
    println!("Set your username");
    std::io::stdin().read_line(&mut first_question_hashmap).unwrap();


    let mut second_question_hashmap = String::new();
    println!("Now tell me you password");
    std::io::stdin().read_line(&mut second_question_hashmap).unwrap();

    let mut exercise_hashmap = HashMap::new();
    exercise_hashmap.insert(first_question_hashmap, second_question_hashmap);

    //This is to take everything from the
    for(x, y) in exercise_hashmap.iter(){
        println!("Username: {}, Password: {}", x, y);
    }
    println!("");

}


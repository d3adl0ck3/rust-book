pub fn all() {
    creating_a_new_vector();
    updating_a_vector();
    reading_elements_of_a_vector();
    iterating_over_the_values_in_a_vector();
    using_an_enum_to_store_multiple_types();
    creating_a_new_string();
    updating_a_string();
    indexing_into_strings();
    methods_for_iterating_over_strings();
    storing_keys_with_associated_values_in_hash_maps();
    updating_a_hash_map();
}
fn creating_a_new_vector() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1,2,3];
}
fn updating_a_vector() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
fn reading_elements_of_a_vector() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("No third element!?  Shenanigans!"),
    }

    let doesnt_exist = v.get(100);
    // don't try this - will panic!
    // let doesnt_exist = &v[100];
    match doesnt_exist {
        Some(nope) => println!("The nope element is {nope}"),
        None => println!("No nope element!?  Yeah, that's abbout right."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let _first = &v[0];

    v.push(6);
    // Can't do this since it causes the mutable reference to live longer and we can't push the 6
    //println!("The first element is: {_first}");
}
fn iterating_over_the_values_in_a_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }
}
fn using_an_enum_to_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("smurfy")),
    SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        println!("{:?}",cell);
    }
    // row vec is dropped here and, if applicable, drop is called on each element
}
fn creating_a_new_string() {
    let mut _s = String::new();
    let data = "initial contents";

    let _s = data.to_string();
    let _s = "initital contents".to_string();
    let _s = String::from("initial contents");
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}
fn updating_a_string() {
    fn appending_to_a_string_with_push_str_and_push() {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{s}");
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
        let mut s = String::from("lo");
        s.push('l');
        println!("{s}");
    }
    fn concatenation_with_the_plus_operator_or_the_format_macro() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{s3}");
        
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("{s}");
        
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // format uses all references so no ownership changes occur
        let s = format!("{s1}-{s2}-{s3}");
        println!("{s}");
    }
    appending_to_a_string_with_push_str_and_push();
    concatenation_with_the_plus_operator_or_the_format_macro();
}
fn indexing_into_strings() {
    let _s1 = String::from("hello");
    // No good cannot index into bytes directly
    //let h = _s1[0];
    // Aside UTF-8 is a pain, some characters are 1 others are 2 bytes so here there be monsteras
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // slice the first 4 bytes which is two characters
    println!("{s}");
    // let s = &hello[0..1]; // will panic since we would cut a character in half
}
fn methods_for_iterating_over_strings() {
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // This results in 6 chars two of which are 'diacritics' and are not handled without some
    // special handling
    for c in "नमस्ते".chars() {
        println!("{c}");
    }
}
fn storing_keys_with_associated_values_in_hash_maps() {
    use std::collections::HashMap;
    fn creating_a_new_hash_map()->HashMap<String,i32> {

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}",scores);
        scores
    }
    fn accessing_values_in_a_hash_map() {
        let scores = creating_a_new_hash_map();
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("{score}");
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }
    fn hashmaps_and_ownership() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
        // println!("{field_name} {field_value}");
    }

    
    creating_a_new_hash_map();
    accessing_values_in_a_hash_map();
    hashmaps_and_ownership();
}
fn updating_a_hash_map() {
    use std::collections::HashMap;
    fn overwriting_a_value() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores); 
    }
    fn adding_a_key_and_value_only_if_a_key_isnt_present() {
        let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    }
    fn updating_a_value_based_on_the_old_value() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count =map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}",map);
    }
    overwriting_a_value();
    adding_a_key_and_value_only_if_a_key_isnt_present();
    updating_a_value_based_on_the_old_value();
}
fn problems() {
    // Given a list of integers return the median (middle value when sorted) and mode (most common
    // value)
    
    // Convert strings to pig latin - first consonant is moved to end and 'ay' appended.  If a
    // vowel is first then just append 'hay'
    // first -> irst-fay
    // apple -> apple-hay
    
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a 
    // department or all people in the company by department, sorted alphabetically.
}

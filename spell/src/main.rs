/**!
 *ASSUMPTIONS
 *
 *- The first charactor must be correct
 *- All words with lowercase or uppercase are the same:for example, Word, word and woRd are the same word
 *- If there are several most frequent words after correction, we only output the last-found one
 *
 */

extern crate spell;
use spell::node::Node;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader,Read,stdin};


fn main() {

    let mut filepath = String::new();
    let mut freq_map:HashMap<String, usize> = HashMap::new();
    let mut trie = Node::new();

    for argument in env::args() {
        filepath = argument;
    }
    let mut file;
    match File::open(filepath){
        Err(_) => panic!("No such files!"),
        Ok(f) => file = f
    }
    let mut lines = String::new();

    match file.read_to_string(&mut lines){
        Err(_) => panic!("File is empty!"),
        Ok(_) => println!("Read file succeed!")

    }

    trie_map_builder(lines, &mut trie, &mut freq_map);

     let mut lines = BufReader::new(stdin()).lines();

     while let Some(Ok(line)) = lines.next() {
         for word in line.split_whitespace(){
            print!("Input: {:?} ", word);
            println!("Correction: {:?}", corrector(word.to_lowercase(), &mut trie, &freq_map));
         }
     }

}

fn trie_map_builder(lines: String, trie: &mut Node, freq_map: &mut HashMap<String, usize>){
    for line in lines.lines(){
         for word in line.split_whitespace(){
            let counter = freq_map.entry(word.to_lowercase()).or_insert(0);
            *counter += 1;
            let mut chars = word.chars();
            trie.build(&mut chars);
         }
     }
}

#[cfg(test)]
mod builder_tests {
    use spell::node::Node;
    use std::collections::HashMap;
    use super::trie_map_builder;

    #[test]
    fn freq_map_test() {
        let lines: String = "hello world hello word hello world".to_string();
        let mut freq_map:HashMap<String, usize> = HashMap::new();
        let mut trie = Node::new();
        trie_map_builder(lines, &mut trie, &mut freq_map);
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(), 3);
        map.insert("world".to_string(), 2);
        map.insert("word".to_string(), 1);
        for (key, val) in map.iter(){
            assert_eq!(freq_map.contains_key(key), true);
            assert_eq!(freq_map.get(key).unwrap(), val);
        }
    }

}


//find the fittest word
fn find_fit(v:Vec<String>, map: &HashMap<String, usize>) -> String{
    let mut max:usize = 0;
    let mut s = String::new();
    for i in 0..v.len(){
        match map.get(&v[i]){
            Some(f) => {
                if *f > max {
                    max = *f;
                    s = v[i].clone();
                }
            },
            None =>{}
        }
    }
    s
}

#[cfg(test)]
mod find_fit_tests {
    use super::find_fit;
    use std::collections::HashMap;

    #[test]
    fn find_fit_test() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(), 3);
        map.insert("world".to_string(), 2);
        map.insert("word".to_string(), 1);
        let v1 = vec!["word".to_string(), "world".to_string()];
        let v2 = vec!["word".to_string(), "hello".to_string()];
        assert_eq!("world", find_fit(v1, &map));
        assert_eq!("hello", find_fit(v2, &map));
    }

    #[test]
    fn none_input_test() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(), 3);
        map.insert("world".to_string(), 2);
        map.insert("word".to_string(), 1);
        let v = vec![];
        assert_eq!("", find_fit(v, &map));
    }

    #[test]
    fn none_output_test() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(), 3);
        map.insert("world".to_string(), 2);
        map.insert("word".to_string(), 1);
        let v = vec!["aloha".to_string()];
        assert_eq!("", find_fit(v, &map));
    }
}


fn corrector(word: String, mut trie: &mut Node, freq_map: &HashMap<String, usize>) -> String{
    let mut result:Vec<String> = Vec::new();
    let mut path:Vec<char> = Vec::new();

    if freq_map.contains_key(&word){
        word
    }else{
        let mut chars = word.chars();
        trie.search(&mut chars, &mut path, &mut result, 2);
        if result.len() > 1{
            result.sort();
            result.dedup();
        }
        find_fit(result, &freq_map)
    }
}

#[cfg(test)]
mod corrector_tests {
    use super::corrector;
    use spell::node::Node;
    use std::collections::HashMap;

    #[test]
    fn corrector_test() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("hello".to_string(), 3);
        map.insert("world".to_string(), 2);
        map.insert("word".to_string(), 1);
        let mut trie = Node::new();
        trie.build(&mut "hello".chars());
        trie.build(&mut "word".chars());
        trie.build(&mut "world".chars());
        assert_eq!("word", corrector("wo".to_string(), &mut trie, &map));
        assert_eq!("word", corrector("woc".to_string(), &mut trie, &map));
        assert_eq!("world", corrector("wor".to_string(), &mut trie, &map));
        assert_eq!("", corrector("w".to_string(), &mut trie, &map));
    }
}

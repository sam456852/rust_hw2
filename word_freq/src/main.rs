/**!
 *ASSUMPTIONS
 *
 *- A complete word would not be divided in two lines.
 *
 *- Url, numbers, charactor, words connected with hyphen and abbreviation(such as 1, U.S, music-books, mother's) is thought to be one word.
 *
 *- Words with uppercase or lowercase are counted as the same word.
 *
 *- Words of flural forms and different tense are considered as different words.
 *
 *- There is no function and meaningless string like "adsdad" or "ssda!@$sad" in the text.
 *
 */

use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::collections::HashMap;

fn main() {

    // for (word, count) in &map{
    //     println!("{}: {}", word, count);
    // }
    sort_and_print(stdout(), read_in(stdin()));


}

fn read_in<R: Read>(reader: R) -> HashMap<String, usize> {
    let mut map:HashMap<String, usize> = HashMap::new();
    //let mut v:Vec<&str> = vec![];
    let mut lines = BufReader::new(reader).lines();
    let x: &[_] = &['$', '.', ',', '!', '?', '(', ')', '{', '}', '[', ']', ':', ';', '\n', '\t', '\r', '"', '_', '-', '\'', '*', '+'];

    while let Some(Ok(line)) = lines.next() {
        //let mut itr = line.split(' ');
        for word in line.split_whitespace(){
            for next in word.split("--"){
                let counter = map.entry(next.trim_matches(x).to_lowercase()).or_insert(0);
                *counter += 1;
            }
            //println!("{}", word.trim_matches(x).to_lowercase());
        }
    }
    map.remove("");
    return map
}
#[cfg(test)]
mod read_in_tests {
    use super::read_in;
    use std::io::Cursor;
    use std::collections::HashMap;


    #[test]
    fn read_test() {
        let mut expected: HashMap<String, usize> = HashMap::new();
        expected.insert(String::from("hello"), 2);
        expected.insert(String::from("world"), 2);
        assert_read(expected.to_owned(), "Hello Hello World World");
        assert_read(expected.to_owned(), "Hello !HeLlo ---- World. World-, ++ **");
    }

    #[test]
    fn words_num_test() {
        let input = "Hello !HeLlo ---- World. World-, ++ ** nice gOod ())+ ,math-book!";
        let mock_read = Cursor::new(input);
        let readin = read_in(mock_read);
        assert_eq!(5, readin.len());
    }

    fn assert_read(expected: HashMap<String, usize>, input: &str) {
        let mock_read = Cursor::new(input);
        let readin = read_in(mock_read);
        assert_eq!(expected, readin);
    }
}

fn sort_and_print<W: Write>(mut writer: W, map: HashMap<String, usize>){
    let mut count_vec: Vec<_> = map.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for item in count_vec{
        write!(writer, "word:{}  times:{}\n", item.0, item.1).unwrap();
    }

}

#[cfg(test)]
mod sort_and_print_tests {
    use super::sort_and_print;
    use std::io::Cursor;
    use std::collections::HashMap;

    #[test]
    fn print_test() {
        let mut expected: HashMap<String, usize> = HashMap::new();
        expected.insert(String::from("hello"), 2);
        expected.insert(String::from("world"), 1);
        assert_write("word:hello  times:2\nword:world  times:1\n", expected);

    }

    fn assert_write(expected: &str, results: HashMap<String, usize>) {
        let mut writer = Cursor::new(vec![]);
        sort_and_print(&mut writer, results);
        assert_eq!(expected.as_bytes(), &*writer.into_inner());
    }
}

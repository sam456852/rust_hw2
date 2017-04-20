use std::collections::HashMap;
use std::str::Chars;

pub struct Node {
    pub children: HashMap<char, Node>,
}



impl Node {


    pub fn new() -> Self{
        Node{
            children: HashMap::new()
        }
    }


    //check whether the node is empty
    pub fn is_empty(&self) -> bool{
        self.children.len() == 0
    }


    //build trie
    pub fn build(&mut self, chars: &mut Chars){

        match chars.next() {

            Some(c) => {
                if !self.children.contains_key(&c){
                    self.children.insert(c, Node::new());
                }

                if let Some(x) = self.children.get_mut(&c){
                    x.build(chars);
                }
            },
            None => {
                self.children.insert('$', Node::new());

            }
        }

    }
    //print out the trie
    pub fn print_keys(&mut self){
        if ! self.is_empty(){
            for key in self.children.keys(){
                print!("{:?}", key);
            }
            for val in self.children.values_mut(){
                val.print_keys();
            }
        }
    }

    //search for the correted words and tol is max edit distance
    pub fn search(&mut self, mut chars: &mut Chars, path: &mut Vec<char>, mut result: &mut Vec<String>, tol: i64){
        if tol < 0{

        }else{
            match chars.next() {
                Some(c) => {
                    for (key, val) in self.children.iter_mut(){

                        let toln;
                        let mut newchars = chars.clone();
                        let mut newpath = path.clone();
                        if *key != c {
                            toln = tol - 1;
                        }else{
                            toln = tol;
                        }

                        newpath.push(*key);
                        let tol1 = toln;

                        val.search(&mut newchars, &mut newpath, &mut result, tol1);

                        for (newkey, newval) in val.children.iter_mut(){
                            let mut newpath1 = newpath.clone();
                            newpath1.push(*newkey);
                            let newchars1 = newchars.clone();
                            let newtol1 = toln - 1;

                            let s:String = newchars1.collect();
                            let tc = c.clone().to_string();
                            let temp = tc + &s;
                            let mut chars1 = temp.chars();


                            newval.search(&mut chars1, &mut newpath1, &mut result, newtol1);
                        }
                        if newchars.clone().count() > 0{
                            let mut newchars2 = newchars.clone();
                            newchars2.next();

                            let mut newpath2 = newpath.clone();
                            let newtol2 = toln - 1;


                            val.search(&mut newchars, &mut newpath2, &mut result, newtol2);
                        }
                        if newchars.clone().count() > 1{
                            let mut newchars3 = newchars.clone();
                            let tmp = newchars3.next();

                            let mut newpath3 = newpath.clone();

                            let newtol3 = toln - 1;

                            let s:String = newchars3.collect();
                            let tc1 = c.clone().to_string();
                            let temp = tmp.unwrap().to_string() + &tc1 + &s;

                            let mut chars3 = temp.chars();

                            val.search(&mut chars3, &mut newpath3, &mut result, newtol3);
                        }
                    }
                },
                None => {
                    if self.children.contains_key(&'$'){
                        result.push(vec_char_to_string(path.to_vec()));
                    }

                }
            }
        }
    }

}

#[cfg(test)]
mod empty_tests {
    use super::*;

    #[test]
    fn empty_test() {
        let trie = Node::new();
        assert_eq!(true, trie.is_empty());
    }

    #[test]
    fn none_empty_test() {
        let mut trie = Node::new();
        trie.build(&mut "hello".chars());
        assert_eq!(false, trie.is_empty());
    }
}

//change Vec<char> into String
fn vec_char_to_string(v: Vec<char>) -> String{
    let s : String = v.into_iter().collect();
    s
}

#[cfg(test)]
mod vec_char2string_test {
    use super::vec_char_to_string;

    #[test]
    fn test() {
        let v = vec!['h', 'e', 'l', 'l', 'o'];
        assert_eq!("hello", vec_char_to_string(v));
    }
}

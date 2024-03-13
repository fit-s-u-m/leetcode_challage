use::std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hash = HashMap::new();
        if s.len()!=t.len() {return false;}
        for charac in s.chars(){
            if let Some(x) = hash.get_mut(&charac) {
                 *x  = *x+ 1;
            }
            else {hash.insert(charac,1);}
        }
        for charac in t.chars(){
            match hash.get_mut(&charac){
                Some(x) =>{
                    if *x==0 {return false}
                    *x -= 1;
                },
                None => return false,
            }
        }
        return true;
    }
}

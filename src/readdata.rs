use regex::Regex;
use std::fs;
use std::collections::HashMap;

pub fn process_data(input: &str) {
    //input should be a chunk:
    let cnt_raw = fs::read_to_string(input).expect("Input not found");

    //PRE-PROCESS:::convert all to lower case:
    let cnt = cnt_raw.to_lowercase();
    let cnt_str = cnt.as_str();

    //PRE-PROCESS:::replace all special characters with empty string:
    let rgx_special_char = Regex::new(r"[$&+,:;=?@#|'<>.^*()%!-]").unwrap();
    let rgx_special_char_out = rgx_special_char.replace_all(&cnt_str, "");

    //PRE-PROCESS:::replace all line breaks with spaces:
    let rgx_ln_brk = Regex::new(r"\n").unwrap();
    let rgx_ln_brk_out = rgx_ln_brk.replace_all(&rgx_special_char_out, " ");

    //PROCESS:::capture all 3-string sequences:
    let rgx = Regex::new(r"((?:\S+\s){2}\S+)\s").unwrap();

    let mut map = HashMap::new();

    //iterate over capture groups: https://docs.rs/regex/1.4.3/regex/#example-iterating-over-capture-groups
    //increment value based on key; if key does not exist then insert it.
    for n in rgx.captures_iter(&rgx_ln_brk_out) {
        //println!("{:?}", &n[1]);
        let count = map.entry(n[1].to_string()).or_insert(0);
        *count += 1;
    }

    for(k,v) in &map{
        println!("{}: {}", k,v);
    }
    

    //Count elements and freq:
    // let mut metrics_map = HashMap::new();
    // for item in rgx.captures_iter(&rgx_ln_brk_out) {
    //     let counter = metrics_map.entry(item[1]).or_insert(0)
    //     *counter += 1;
    // };

    // println!("{:#?}", metrics_map);
}

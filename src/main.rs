use std::io;
use std::collections::HashMap;
use std::vec::Vec;
fn main() {
    q_1_1();
}
fn q_1_1(){
    println!("Cryptopals Set 1 Challenge 1!");
    hex2base64();
}
//Cryptopals Set 1 Challenge 1
fn hex2base64(){
    let hexmap: HashMap<&str,&str> = [("0","0000"),("1","0001"),("2","0010"),("3","0011"),("4","0100"),("5","0101"),("6","0110"),("7","0111"),("8","1000"),("9","1001"),("A","1010"),("B","1011"),("C","1100"),("D","1101"),("E","1110"),("F","1111")].iter().cloned().collect();
//49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
    let mut _input = String::new();
    let mut _output = String::new();
    io::stdin().read_line(&mut _input)
        .expect("Failed to read line");
    let mut tmp_vec: Vec<String> = Vec::with_capacity(6);
    let mut output: Vec<char> = Vec::new();
    for c in _input.chars(){
        if let Some(&byte) =  hexmap.get::<str>(&c.to_string().to_uppercase()) {
            let mut byte_vec: Vec<String> = byte.chars().map(|x| x.to_string()).collect();
            let rem = if tmp_vec.len() %24; 
            if rem > 0{

                tmp_vec.append(&mut byte_vec.get(0..2).unwrap().iter().cloned().collect());
                
            }

            if (tmp_vec.len() == 24){}
                        let mut ap = bin2dec(&mut tmp_vec).chars().collect();
                        output.append(&mut ap);
                        tmp_vec.truncate(0);
            }
            println!("LEN {:?}", tmp_vec.len());
            if tmp_vec.len() ==4 {
                //let mut u: Vec<char> = byte_vec.splice(..2,new.iter().cloned()).collect(); //not good rust
                tmp_vec.append(&mut byte_vec.get(0..2).unwrap().iter().cloned().collect());
                let mut ap = bin2dec(&mut tmp_vec).chars().collect();
                output.append(&mut ap);
                tmp_vec.truncate(0);
                tmp_vec.append(&mut byte_vec.get(2..4).unwrap().iter().cloned().collect()); //how do i do [2:]
            }else{
                tmp_vec.append(&mut byte_vec);
                println!("LEN ELSE {:?} {:?}", tmp_vec.len(),tmp_vec);
                if tmp_vec.len() == 6 {
                }
            
            }
        }
    }
    if tmp_vec.len() > 0 {
        tmp_vec.append(&mut vec!["0".to_string(),"0".to_string()]);
        let mut ap = bin2dec(&mut tmp_vec).chars().collect();
        output.append(&mut ap);
    }
            
        //let index = hex.iter().position(|&r| r == c).unwrap();

        
 //   let mut _b_input = _input.as_bytes();

    println!("In Base64 that is {:?}", output);
}
fn bin2dec(mut _byte_vec: &Vec<String>) -> (String){
    let mapping: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    println!("Tst Base64 that is {:?} {:?}", _byte_vec.len(),_byte_vec);
    
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    for bit in _byte_vec{
           if bit == "1" {
            s += 2^i; 
        }
        i += 1;
    }
    ///println!("s {:?} {:?}", s,mapping.chars().nth(s as usize).unwrap());
    return mapping.chars().nth(s as usize).unwrap().to_string();
}
    

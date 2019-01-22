use std::io;
use std::iter::FromIterator;
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
    let _test = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    let mut _input = String::new();
    let mut _output = String::new();

    let mut _mode =String::new();
    println!("Interactive Y/N?");
    io::stdin().read_line(&mut _input)
        .expect("Failed to read line");
    if _mode != "N"{
        io::stdin().read_line(&mut _input)
        .expect("Failed to read line");
    }else{
        _input=String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    }

    let mut bytes: Vec<String> = Vec::new();
    let mut output: Vec<char> = Vec::new();
    for c in _input.chars(){
        if let Some(&byte) =  hexmap.get::<str>(&c.to_string().to_uppercase()) {
            let mut byte_vec: Vec<String> = byte.chars().map(|x| x.to_string()).collect();
            bytes.append(&mut byte_vec);
        }
    }
    for b in bytes.chunks(24){
        let b_len = b.len();
        //this padding is a hot mess 
        let mut b_vec = b.to_vec();
        let mut ap = bin2dec(&b_vec).chars().collect();
        output.append(&mut ap);
    }
    let finaloutput = String::from_iter(output);

    println!("In Base64 that is {:?}", finaloutput);
    if _mode == "N"{
        assert!(finaloutput == _test);
    }        
    
}
fn bin2dec(mut _byte_vec: &Vec<String>) -> (String){
    let mapping: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="; //should be global figure that out
    let mut base64 = String::new();
    
    let mut s: i32 = 0;
    let mut i: i32 = 0;    
    let mut zero =  vec![String::from("0")];
    for bit in _byte_vec.chunks(6){
        let mut bit_vec = bit.to_vec();
        //println!("{:?}", bit_vec);
        i = 0; 
        s = 0; 
        let len = 6-bit_vec.len();
        for _i in 0..len{
            bit_vec.append(&mut zero.iter().cloned().collect());
        }
        //println!("sizel is {:?}", bit_vec.len());
       // println!("size is {:?}", bit_vec);
        bit_vec.reverse();
        for b in bit_vec{

            if b == "1"{
                s += i32::pow(2,i as u32);
            }
            i+=1;
        }
        base64.push(mapping.chars().nth(s as usize).unwrap()); //.unwrap().to_string());
        //println!("SUM {:?}, {:?}", s,base64);
    }
    //println!("Tst Base64 that is {:?} {:?} {:?}", s,_byte_vec,base64);
    //for i <6 add ==
    return base64;
}
    

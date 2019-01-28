use std::io;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::vec::Vec;
fn main() {
    q_1_1();
}
fn q_1_2() {
    let _input1 = String::from("1c0111001f010100061a024b53535009181c");
    let _input2 = String::from("686974207468652062756c6c277320657965");
    let _output = String::from("746865206b696420646f6e277420706c6179");
    let mut _bin_in1 = hex_decoding(_input1);
    let mut _bin_in2 = hex_decoding(_input2);
    let diff: usize = _bin_in1.len() - _bin_in2.len();
    let mut result: Vec<String> = Vec::new(); //does not need to be mut

    // A BIT HACKY
    //_bin_in1.reverse();
    //  _bin_in2.reverse();
    println!("{:?}", _bin_in1);
    println!("{:?}", _bin_in2);

    if diff < 0 {
        result = fixed_xor(&_bin_in2, &_bin_in1, (diff as i32).abs());
    } else {
        result = fixed_xor(&_bin_in2, &_bin_in1, (diff as i32).abs());
    }
    let mut hex_result = binTo24(&mut result);
    println!("{:?}", hex_result);
}
fn q_1_1() {
    println!("Cryptopals Set 1 Challenge 1!");
    hex2base64();
}

fn fixed_xor(
    mut _smaller_str: &Vec<String>,
    mut _larger_str: &Vec<String>,
    diff: i32,
) -> (Vec<String>) {
    let mut bit_vec: Vec<String> = Vec::new();
    //this is idiotic - I need to figure out a better way
    let mut zero = vec![String::from("0")];
    let mut one = vec![String::from("1")];
    let mut _i: i32 = (_larger_str.len() - 1) as i32;

    for c in _larger_str {
        println!(
            "{:?},{:?}",
            _larger_str[_i as usize], _smaller_str[_i as usize]
        );
        let mut ch = vec![String::from(c.clone())];
        if _i < diff {
            bit_vec.append(&mut ch.clone());
        } else if _larger_str[_i as usize] == _smaller_str[_i as usize] {
            bit_vec.append(&mut zero.clone());
            println!("0");
        } else {
            bit_vec.append(&mut one.clone());
            println!("1");
        }
        _i -= 1;
    }
    bit_vec.reverse();
    return bit_vec;
}

fn hex_decoding(mut _input: String) -> (Vec<String>) {
    let hexmap: HashMap<&str, &str> = [
        ("0", "0000"),
        ("1", "0001"),
        ("2", "0010"),
        ("3", "0011"),
        ("4", "0100"),
        ("5", "0101"),
        ("6", "0110"),
        ("7", "0111"),
        ("8", "1000"),
        ("9", "1001"),
        ("A", "1010"),
        ("B", "1011"),
        ("C", "1100"),
        ("D", "1101"),
        ("E", "1110"),
        ("F", "1111"),
    ].iter()
        .cloned()
        .collect();
    let mut bytes: Vec<String> = Vec::new();
    let mut output: Vec<char> = Vec::new();
    for c in _input.chars() {
        println!("Bytes {:?}", c);

        if let Some(&byte) = hexmap.get::<str>(&c.to_string().to_uppercase()) {
            let mut byte_vec: Vec<String> = byte.chars().map(|x| x.to_string()).collect();
            bytes.append(&mut byte_vec);
        }
    }
    return bytes;
}
//Cryptopals Set 1 Challenge 1
fn hex2base64() {
    let _test = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    let mut _input = String::new();
    let mut _output = String::new();

    let mut _mode = String::new();
    println!("Interactive Y/N?");
    io::stdin()
        .read_line(&mut _mode)
        .expect("Failed to read line");
    if _mode != "N" {
        io::stdin()
            .read_line(&mut _input)
            .expect("Failed to read line");
        _input = _input.replace('\n', "")
    } else {
        _input=String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    }
    let mut bytes: Vec<String> = Vec::new();
    let mut output: Vec<char> = Vec::new();
    println!("Bytes {:?}", _input);
    bytes = hex_decoding(_input);
    println!("Bytes {:?}", bytes);

    output = binTo24(&bytes);
    let finaloutput = String::from_iter(output);

    println!("In Base64 that is {:?}", finaloutput);
    if _mode == "N" {
        assert!(finaloutput == _test);
    }
}
fn binTo24(mut bytes: &Vec<String>) -> (Vec<char>) {
    let mut output: Vec<char> = Vec::new();
    for b in bytes.chunks(24) {
        let b_len = b.len();
        //this padding is a hot mess
        let mut b_vec = b.to_vec();
        let mut ap = bin2dec(&b_vec).chars().collect();
        output.append(&mut ap);
    }
    return output;
}
fn bin2dec(mut _byte_vec: &Vec<String>) -> (String) {
    //should be global figure that out
    let mapping: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    let mut base64 = String::new();

    let mut s: i32 = 0;
    let mut i: i32 = 0;
    let mut zero = vec![String::from("0")];
    for bit in _byte_vec.chunks(6) {
        let mut bit_vec = bit.to_vec();
        //println!("{:?}", bit_vec);
        i = 0;
        s = 0;
        let len = 6 - bit_vec.len();
        for _i in 0..len {
            bit_vec.append(&mut zero.iter().cloned().collect());
        }
        //println!("sizel is {:?}", bit_vec.len());
        // println!("size is {:?}", bit_vec);
        bit_vec.reverse();
        for b in bit_vec {
            if b == "1" {
                s += i32::pow(2, i as u32);
            }
            i += 1;
        }
        base64.push(mapping.chars().nth(s as usize).unwrap());
    }
    //println!("Tst Base64 that is {:?} {:?} {:?}", s,_byte_vec,base64);
    //for i <6 add ==
    return base64;
}

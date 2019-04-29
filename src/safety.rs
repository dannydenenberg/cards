//mod safety {
//    mod cards;
//    mod art;
//
//    extern crate termion;
//
//    use cards::cards::{Deck, Card};
//
//    fn main() {
//        let stuf: Vec<String> = art::art.split("+-+-+-+-+-+-+-+-+-+-+-+").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
//        let mut stuff: Vec<String> = Vec::new();
//        for s in stuf {
//            stuff.push(s.replace("#", "9"));
//        }
//
//
//        let first = stuff[0].split("\n").collect::<Vec<&str>>();
//        let second = stuff[1].split("\n").collect::<Vec<&str>>();
//
////    println!("{:?}", first);
////
////    for i in 0..first.len() {
////        println!("{}{}", first[i], second[i]);
////    }
//
//        print_side_by_side(stuff);
//
//        println!("{}", "hello".replace("l", "&"));
//
//    }
//
//
//    // takes same dimension ascii drawings and prints them side by side
//    fn print_side_by_side(drawings: Vec<String>) {
//        // represents the lines of each drawing
//        let mut lines: Vec<Vec<String>> = Vec::new();
//
//        // go through the drawings, and put each line into its own vector in the lines vector
//        for pic in drawings {
//            lines.push(pic.split("\n").collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect());
//        }
//
//        //// ***NOTE: this assumes all of the drawings are of the same dimentions!***
//        for i in 0..lines[0].clone().len() {
//            // go through all of the pictures and print each line next to eachother
//            for k in lines.clone() {
//                print!("{}", k[i]);
//            }
//            println!();
//        }
//    }
//
//
//
//// old
//
//// takes same dimension ascii drawings and prints them side by side
////fn print_side_by_side1(drawings: Vec<String>) {
////    // represents the lines of each drawing
////    let mut lines: Vec<Vec<&str>> = Vec::new();
////
////    // go through the drawings, and put each line into its own vector in the lines vector
////    for pic in drawings {
////        lines.push(pic.split("\n").collect::<Vec<&str>>());
////    }
////
////    //// ***NOTE: this assumes all of the drawings are of the same dimentions!***
////    for i in 0..lines[0].clone().len() {
////        // go through all of the pictures and print each line next to eachother
////        for k in lines.clone() {
////            print!("{}", k[i]);
////        }
////        println!();
////    }
////}
//}
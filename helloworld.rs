// this is a single line comment

/* 
 This is a multi line comment 
 can also be used for inline comments
 */

 /// doc comment which is parsed into html library
 /// are being compiled into documentation running ```rustdoc```
 /// markdown friendly

fn main(){
    // {} is being replaced by arguments
    println!("{} World", "Hello"); //output "Hello World"
    // {} can be filled with numbers or variable names
    println!("{subject} {verb} {object}",object="a Rustacean", subject="I", verb="am");

    // variables
    let logical: bool = true;
    let float: f64 = 3.0; // regular
    let float2 = 3.0f64;  // Suffix
    let float3 = 3.0;     // default type

    let mut mutable = 12; // is mutable but only value not type
    

}
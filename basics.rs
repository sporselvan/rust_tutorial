fn main() {

    // eprint!("this is an error");  //printing error
    
    let mut some_variable = "foobar";   //mut turns variable mutable
    println!("{some_variable}");        //output : foobar
    some_variable = "changed";
    println!("{some_variable}");        //output : changed
    
    let (mut name,mut age,mut height) = ("hawk",23,5.8);    //declaring multiple variables as mutable
    println!("{name},{age},{height}");                      //output hawk,23,5.8
    (name,age,height) = ("ban",24,6.0); 
    println!("{name},{age},{height}");                      //output ban,24,6
    
    //signed integers
    let a:i8 = -128;        //i8 singned integer  | limit is 255(2^8-1 = 255) | -128 to +127
    let b:i16 = -30768;     //i16 signed integer  | -32768 to 32767  
    
    //unsigned integers
    let ab:u8 = 255 ;       //positive integers 0 t0 255
    let cd:u16 = 65535;     //limits from 0 to 65535
    let d = -90;
    println!("{a},{b},{ab},{cd},{d}");   
    
    //float
    let just_float = 0.5;
    let just_float_32 :f32 = 10.5;
    println!("{just_float},{just_float_32}");
    
    //booleans
    let is_true : bool = true;
    let is_false : bool = false;
    let decide = 5<7;
    println!("{is_true},{is_false},{decide}");
    
    //char
    let abc:char = 'f';           //alphabets for example : 'a' | not 'ab'
    let query:&str = "success";   //string should be covered by " " (double quotes)
    let emoji = "\u{1f600}"; 
    println!("{abc},{query}{emoji}");  
    
    //array
    let mut arr:[i32;4] = [21,24,25,26];  
    arr[1] = 0;
    arr[2] = 1;
    println!("{:?}",arr);        //print array data type using Debug Trait {:?}
    
    //array-iteration
    for item in arr.iter(){
        println!("array element {item}");
    };
    
    //multi dimensional array
    let multi:[[i32;6];2]  = [   
    [21,22,23,24,25,26],
    [28,27,26,25,24,23]
    ];
    println!("{:?}",multi);   
    println!("{},{}",multi[1][0],multi[1][1]);     //access 1st array elements from multi dimensional array using index [1][0],[1][1]
    
    //slice
    let a: [ i32; 4] = [1,2,3,4];
    let slices: &[i32] = &a[0..3];                 // Lower range is inclusive and upper range is exclusive
    println!("{:?}",slices);
    
    //vector is a dynamic growable array
    
    let mut some_vector = vec![1,2,3,4,5];
    some_vector.push(6);
    println!("{:?}",some_vector);
    
    //tuple 
    let tup = ("apple","a",1,true);
    println!("{tup:?}");
    
    
    // strings
    // Rust has two main types of strings: &str and String.
    // &str is static string and String is growable.
    
    let mut s:String = String::from("string example");      // &str is immutable. String is mutable.
    s.push_str(" added");                                   //push_str used for pushing character or entire string.
    println!("{s}");                                        // => string example added
    s.push('A');                                            // push used for pushing only character.
    println!("{s}");                                        // => string example addedA 
    
    let rand = String::from("Random");
    println!("{}",rand.capacity());         // => string length => 6
    println!("{}",rand.contains("and"));    // => true 
    println!("{}",rand.contains("rand"));   // => false. contains method is case sensitive.
    
    //Operators
    // == Equal || != not Equal || < less than || > greater than || <= less than equal || >= greater than equal 
    println!("{}",'e' == 'e');  // => true 
    println!("{}",2 > 5);       // => false
    println!("{}",5 >= 5);    // => true
    
    //Arithmetic operator
    // + addition || - Subtraction || / division || * multiply || % reminder function
    let (a,b) = (4,5);
    let add : i32 = a+b;
    let sub : i32 = b-a;
    let multi : i32 = a*b;
    let div : i32 = b/a;
    let modulus : i32 = b%a; 
    
    println!("addition :  {add}, Subtraction : {sub}, Multiplication :{multi}, division : {div},modulus :{modulus}");
    
    // Logical operator 
    // flow control
    // &&  and , || or ,  ! not
    let (result_1,result_2) = (true,true);
    
    //if
    if result_1 && result_2 == true {
        println!("success!😉");
    }
    
    let (result_3, result_4) = (true,false);
    if result_3 || result_4 == false {
        println!("result_4 is false");
    }
    
    if !result_4 {
        println!("result_4 is false 🤞");  
    }
    
    
}





fn main() {
    let x1: i32 = 5;
    let s1:String = String::from("hello");
    let s2 = s1.clone();
    println!("The value of s1 is: {:}, s2: {:}", s1, s2);

    let s3: &String = &s2;
    println!("The value of s3 is: {:}", s3);

    println!("pointer value of s1: {:p}", &s1);
    println!("pointer value of s2: {:p}", &s2);
    println!("pointer value of s3: {:p}", s3);

    println!("pointer value of x1: {:p}", &x1); 
}
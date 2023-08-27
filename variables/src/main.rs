// ----------- Will give error as it is not allowed to change the value of a variable by default -----------
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// ------------- This would not give error as we have used mut keyword to make the variable mutable --------------
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// ------------------ Shadowing ------------------
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// -------------- mul keyword will give error as it is not allowed to change the type of a variable --------------
// fn main() {
//     let spaces = "   ";
//     spaces = spaces.len();
//     println!("The value of spaces is: {spaces}");
// }
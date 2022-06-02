// macro_rules! print_object {
//     // `()` indicates that the macro takes no argument.
//     (print_object: <T> = None) => {
        
//         // The macro will expand into the contents of this block.
//         println!("{:?}", print_object);
//     };
// }

pub fn print_object(to_print: <T>){
    println("{:?}", to_print);
}

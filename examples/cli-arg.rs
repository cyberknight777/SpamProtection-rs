use spamprotection::info;
use std::env;

/* We would need to first to declare the spamprotection library with use declaration.
   We would also need to declare the env trait from std library as args variable will be collecting the arguments passed to the program.
*/

fn main()
{
    /* Here we will need to assign a Vector type to args variable to collect the arguments passed to program.
       Then we will need to assign String type to arg variable to parse the arguments passed.
    */
    let args: Vec<String> = env::args().collect();
    let arg: String = args[1].parse().expect("invalid account");

    /* Here's where we will start by declaring the info variable and assign it to the info module in spamprotection library
       which calls the full() method with given the arg variable as the value.
    */
    let info = info::full(arg);
    println!("{:#?}", info);
}

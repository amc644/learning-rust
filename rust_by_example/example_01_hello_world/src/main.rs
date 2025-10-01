fn main() {
    // https://doc.rust-lang.org/rust-by-example/hello.html

    // EXAMPLE 1-1: hello.html ----------------------------

    // println!("Hello, world!");

    // EXAMPLE 1-2: hello/comment.html --------------------

    // This is a `line comment`. It only takes one line

    // In VS Code you can comment or uncomment a line
    // By pressing `Ctrl + /` in a keyboard with US distribution
    // Or by pressing `Ctrl + ç` in a keyboard with Spanish distribution
    // Even if you select several lines of text
    // You can select this block of text and try with any of the comands above.

    /*
        This is a multiline comment. Multiline comments can
        be nested,  so you can put a multiline comment between
        a multiline comment as you can see in the next example
        below:
    */

    /* <- add another '/' before the 1st one to uncomment the whole block
        println!("Hello, world!");
        println!("I'm a Rustacean!");
    // */

    /*  <- Try again an check how the line comment is not affected:
    //  -----This line is not affected-----
        println!("Hello, world!");
        println!("I'm a Rustacean!");
    // */

    /* Also, in multiline comments you can select a piece of text and press
    `Shift + Alt + A` to put or remove block comments within a piece of code.
    Try it with the expresion below selecting these characters only:
    `2 + 3 + 4 + 5 + 7 + 8 +`. After that press the keyshortcut above.*/
    //let x = 1 + /* 2 + 3 + 4 + 5 + 7 + 8 + */ 9;
    //println!("x = {}", x);

    // EXAMPLE 1-3: hello/print.html -----------------------------------

    // Curly braces `{}` will stringify a value.
    // println!("{} days", 31);

    // Positional arguments:
    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    /* Named arguments:
    println!("{subject} {verb} {object}",
                object="the lazy dog.",
                subject="The quick brown fox",
                verb="jumps over");
    // */

    /* Invoking different formats by specifying the format character after a colon `:`
    println!("Base 10:               {}",   69420);
    println!("Base 2 (binary):       {:b}", 69420);
    println!("Base 8 (octal):        {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    // */

    /* Rith-justifying text:
    println!("{number:>5}", number = 1);
    let x = 1;
    println!("{x:>10}");
    // */

    /* Padding left and right:
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    // */

    // Use of named arguments in the format specifier by appending a `$`
    // println!("{number:0>width$}", number = 1, width = 10);

    // Only types that implement fmt::Display can be formatted with `{}`:
    /* (disable `dead_code` which warn against unused module)
    #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct `{}` won't print...", Structure(3));
    // */

    /* Capturing arguments form the variable
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    // */

    // EXAMPLE 1-3: hello/print.html -----------------------------------


















}

    // EXAMPLE 1-1: hello.html
    // EXAMPLE 1-2: hello/comment.html
    // EXAMPLE 1-3: hello/print.html - In progress, I need to read the part below
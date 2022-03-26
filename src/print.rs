pub fn run() {

    // Print to console

    println!("Hello From the print.rs file");

    // Basic Formatting

    println!("{} is from {}", "Maygane", "Baden");

    // Positional Arguments

    println!("{0} is from {1} and {0} likes to {2}", "Baden", "Mass", "Code");

    // Named Arguments

    println!("{name} likes to play {activity}", name = "Maygane", activity = "Cosplayer");

    // Variable Exo need Mut

    let mut x = 10;

    println!("x is {}", x);

    x = 20;

    println!("x is {}", x);
}
// ! Variables and Mutability
// * Variables in Rust are immutable by default
// * But we can still make it mutable using the `mut` keyword

// ! Declaring variables
fn variables() {
    // Example: Variables immutability
    // let x = 10;
    // x = 20; // Error: Cannot assign twice to immutable variable
    // * The rust compiler guarantees that when you state that a value won't change, it really won't change

    // We can make it mutable using the mut keyword
    let mut y = 10;
    println!("The value of y is {y}");
    y = 20;
    println!("The value of y is {y}");
}

// ! declaring a constants
// * Like Immutable variables, constants are values that are bound to a name that are not allowed to change,
// * But there are few diff between `constants` and `variables`
// * 1. Constants are not immutable by default. They are immutable.
// * 2. We use `const` keyword instead of `let` to declare constants.
// * 3. We are not allowed to use `mut` keyword in constants.
// * 4. The type of the const value must be annotated.
// * 5. A const cannot generally be initialized with a runtime value, such as data returned from a network call. It's initializer must be evaluable in a constants context at compile time.
fn constants() {
    const PRICE: u32 = 1200;
    const MAX_USERS: u32 = 60;
    const TIME_LIMIT: u32 = 40;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!(
        "Price: {PRICE}\nMax Users: {MAX_USERS}\nTime Limit: {TIME_LIMIT}\nThree Hours In Seconds: {THREE_HOURS_IN_SECONDS}"
    );
}

// ! Shadowing a variable
// * There is a unique features in Rust that we can shadow a variable.
// * We can shadow it using the same variable name with using let keyword
// * Shadowing variable is different from marking it using `mut` keyword.
// * We will get the compile time error if we don't use `let` keyword while shadowing a variable.
// Example: Shadowing a variable
fn shadowing_variable() {
    let x = 5;
    let x = x + 5;
    {
        let x = x * 2;
        println!("The value of inner x is: {x}"); // expected output: 20
    }
    println!("The value of outer x is: {x}"); // expected output: 10

    // * The other difference is when we mutable variable using `mut` keyword, we can't have more than one types for that variable but in Shadowing we can change the type also.
    let spaces: &str = "       ";
    let spaces: usize = spaces.len();
    println!("length of spaces: {spaces}");
}

fn main() {
    variables();

    constants();

    shadowing_variable();
}

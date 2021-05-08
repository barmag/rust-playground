# rust-playground
Personal playground for learning rust
## variables-ownership project
Following LinkedIn course: [Rust Essential Training](https://www.linkedin.com/learning/rust-essential-training)

Shadowing allows the reuse of variables with the same name. The shadowed variable can change the data type or mutability. The shadow variable can be limited to a specific scope, and the original variable is restored after getting out of the scope. Shadowing can be used to reuse common names, but if not used with intent can lead to wierd bugs.

In rust values are owned by one variable. In case of types with known size that can be placed on the stack the values are copied if assigned to a different variable. For type on the heap, the reference is moved to a new variable an the old variable cannot be used. This is done so rust can predict the lifetime of the value, and avoid double deletion. The clone method can be used to do a deep copy of the value, and maintain two variables independently. When invoking a function with a parameter the value is either copied if it is a stack value, or moved if it is a heap value. The data will be allocated after the function returns, and the invoking function will not be able to access the value.
Rust allows mutable reference so the invoked function can modify the referenced value and the invoking function can read the modification. An important limitation is rust allows only one mutable reference to be active at a time. Multiple mutable references are forbidden to prevent race conditions in multi-threaded scenarios.

## guessing-game
Following LinkedIn course: [Rust Essential Training](https://www.linkedin.com/learning/rust-essential-training)
- Using rust standard library, specifically std::io
- Using crates via cargo. Add a dependenct on rand crate.
Using rand library to generate a random number between 1 and 100. Recieve a guess from the user and print if the number is higher, lower, or equal to the generated number.

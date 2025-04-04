# Jacob Lord
# 03/26/2025

## Assignment
**Social Sentiment Scoring - Rust Program**

## Q1 - 5 Short Answer Questions

**1. How does the new language handle variable memory allocation, and what impact does this have on performance and memory safety?**

    The Rust programming language allocates memory for variables on the stack by default. With the variable being stored in the stack, the performance is very fast in contrast to if it were stored in the heap. With storage on the stack being the default, memory safety is protected as allocation and deallocation of the stack is managed directly by the system through its ownership and borrowing system. Every value has an owner, and ownership can be transferred and returned as necessary. This greatly assists in preventing memory related errors such as memory leaks and dangling objects by ensuring the memory that was in use is dropped once the variable's lifetime is finished without any programmer intervention. Unlike automatic garbage collection, like what is seen in Java, the ownership model has far greater runtime efficiency as there is not an entirely seperate process dedicated to cleaning up garbage because the programmer is forced to write clean, safe code otherwise the source code will not compile.


**2. Describe the language’s approach to type systems and type binding time. Is the language statically or dynamically typed, and how does that affect error detection and program flexibility?**

    Rust's approach to type systems is that it is strongly typed to ensure program behavior executes as expected, and the binding of types occurs during compilation. It is statically typed which is implied with the type binding occurring during compilation, which allows for effective error detection during compilation. This means that the majority of errors in programs are caught by the Rust compiler (rustc) and the build process will be terminated until the programmer corrects the errors. The language being statically typed decreases the overall flexibility of the program developed in favor of reliability as data types cannot be coerced into alternative types. However, Rust does allow for the shadowing of variables by re-utilizing the "let" keyword to allow for implicit conversion except in the case where many data types could be possible for a method being used on the variable, in which case the data type must be explicitly stated. This shadowing functionality allows for increased flexibility even with the language being statically typed.


**3. What are the subprogram calling conventions in the language? How are parameters passed (by value, by reference, etc.), and how does the return mechanism work?**

    The subprogram calling conventions in the Rust language consist of the function beginning with ```fn```  then a whitespace followed by the user-defined function name. The conventional style of naming function for rust is snake case. This function name is then followed by a pair of parenthesis to contain the function parameters. The body of the function is encapsulated by curly braces resulting in a function format resembling:

    fn test_fxn() { /*execution body*/ }
 
    Function parameters are in the format of the parameter name trailed by a colon then the data type of the parameter. Declaring the data type of each parameter is a requirement of the langauge. For example, if the parameter is a character, then the function would be of the form: 

    fn test_fxn( x : char ) { /* execution body */}

    For more than one parameter, a comma seperates each individual parameter. For example:

    fn test_fxn( x : char, y : i32 ) { /* execution body */}

    Rust function parameters are passed by value by default. Ownership of the value is passed to the function, so once the function finishes running the memory used to store the value is dropped and is unusable once the function ends unless the value is returned and stored in a different variable. For data types that implement the copy trait, a copy of the value is given to the function, and therefore original variable that was put into the function arguments is still usable after the function call finishes. The language also allows for passing arguments by reference by using the "&" symbol, so instead of providing the function ownership over the variable, the function can be provided access to use it without it being dropped once the function call finishes. The functionality of passing by reference in Rust is known as borrowing because ownership over the variable is not given, just access to the memory that is pointed to by the variable.

    When defining the function signature, the return value's data type must be declared on the same line by pointing to it with the "->" symbol. An example of this would be:

    fn test_fxn( x : char, y : i32 ) -> char { /* execution body */}

    The return type for the example function is a character and this is how the compiler knows if the return value of a function is valid.The return mechanism in works by returning ownership over a variable back to whatever assignment statement was used with the function call. If no assignment statement was used to store the value returned by the function call then the variable is dropped at the end of it and becomes unusable. 


**4. How does the language manage variable scope and lifetime—particularly for local, global, and any other static variables, if any?**

    The language manages variable scope with curly braces like you would see in C. The lifetime of local variables are managed via the ownership system, so once a variable is out of scope its lifetime ends and it is cleared from memory. If ownership is not passed and the variable is simply borrowed into a function's scope then its lifetime extends beyond the scope of the function. There are no explicit global variables like you would see in other langauges such as Python, but there are static variables. Static variable's lifetimes exist for the entire duration of the program.


**5. Identify one unique or innovative feature of the language and explain how it affects the way programmers write and structure code.**

    The most unique feature of the Rust programming language is its owernship and borrowing system. With languages such as C, heap memory management is left as the programmer's responsibility; however, for Rust the ownership and borrowing systems handle the memory management and force the programmer to write safe code or else the source code will not compile. By eliminating the need for manual allocation and deallocation, the code written is less cluttered and easier to read. 
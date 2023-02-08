# Rust 🦀
### Ownership Rules
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope
<img
  src="Images\Screenshot 2023-02-08 062557.png"
  alt="Alt text"
  title="Optional title"
  style="display: block; margin: 0 auto; max-width: 500px">

### Variables and Data Interacting with Move
-  When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. If both data pointers s2 and s1 pointing to the same location go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

- To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:
<img
  src="Images\Screenshot 2023-02-08 063253.png"
  alt="Alt text"
  title="Optional title"
  style="display: block; margin: 0 auto; max-width: 500px">

- If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.

<div
    style="display: flex; justify-content: center">
<img
  src="Images\trpl04-04.svg"
  alt="Alt text"
  title="Optional title"
  style="display: block; margin: auto; width: 300px">
</div>

### Ownership and Functions
- The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.

<img
  src="Images\Screenshot 2023-02-08 055445.png"
  alt="Alt text"
  title="Optional title"
  style="display: block; margin: 0 auto; max-width: 500px">

### Return Values and Scope

<img
src="Images\Screenshot 2023-02-08 055529.png"
alt="Alt text"
title="Optional title"
style="display: block; margin: 0 auto; max-width: 500px">

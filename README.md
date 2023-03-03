# RedRust (Redis in Rust)

redrust is a port of the popular Redis database system written in Rust programming language. This port aims to provide all the features of Redis while taking advantage of the Rust language's safety, speed, and modern language features.

Why port Redis to Rust?
-----------------------

Porting Redis to Rust offers several benefits:

### 1\. Performance

Rust is known for its performance and efficiency, thanks to its control over memory allocation and its ability to compile to native code. Redis in Rust takes advantage of these features to provide a fast and efficient database system that can handle large amounts of data with ease.

### 2\. Safety

Rust has a unique ownership model that ensures memory safety and prevents common programming errors such as null pointer dereferences, buffer overflows, and data races. Redis in Rust uses these features to prevent crashes and security vulnerabilities that are common in C programs.

### 3\. Modern language features

Rust is a modern programming language that includes many features that make it easier to write correct and maintainable code. Redis in Rust uses these features to provide a more maintainable and extensible codebase, making it easier for developers to contribute to the project and add new features.

Getting started
---------------

To use RedRust, you will need to have Rust and Cargo installed on your system. Once you have Rust and Cargo installed, you can clone the redrust repository and build the library:

shellCopy code
```
$ git clone https://github.com/mahajpuneet/redrust.git
$ cd redrust
$ cargo build --release
```

This will build the RedRust library in release mode. You can then link against the library in your own Rust project.

Contributing
------------

RedRust is an open-source project, and we welcome contributions from anyone who is interested in helping to improve the project. 

License
-------

RedRust is licensed under the Apache 2.0 license. 

Conclusion
----------

RedRust is a high-performance, safe, and modern port of Redis to the Rust programming language. We believe that this project has the potential to become a popular alternative to the C-based Redis implementation, and we invite you to contribute and help us make it even better.

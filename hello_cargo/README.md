## Rust Tutorial
> Rust Tutorial for dummies
---
**Hello, Cargo!**

- ```println!``` --> indicate a macro function not a normal function cargo is rust's build system and package manager.

---
**Creating a Project with Cargo**

- With ```cargo new hello_cargo``` --> The first command creates a new directory called hello_cargo.

- We’ve named our project ```hello_cargo```, and Cargo creates its files in a directory of the same name.

- ```Cargo.toml``` --> Tom's Obvious Minimal Language --> cargo's configuration format.

- The first line, ```[package]```, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

- The next four lines set the configuration information Cargo needs to compile your program: the name, the version, who wrote it, and the edition of Rust to use.

- Cargo gets your name and email information from your environment, so if that information is not correct, fix the information now and then save the file.  We’ll talk about the edition key in Appendix E.

- The last line, ```[dependencies]```, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates. We won’t need any other crates for this project, but we will in the first project in Chapter 2, so we’ll use this dependencies section then.

- Cargo has generated a “Hello, world!” program for you, just like the one we wrote in Listing 1-1!.
So far, the differences between our previous project and the project Cargo generates are that Cargo placed the code in the src directory, and we have a Cargo.toml configuration file in the top directory.

- Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

- If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the src directory and create an appropriate Cargo.toml file

---
**Building and Running a Cargo Project**

Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:
```shell
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
This command creates an executable file in ```target/debug/hello_cargo``` (or ```target\debug\hello_cargo.exe``` on Windows) rather than in your current directory. You can run the executable with this command:
```shell
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```
If all goes well, Hello, world! should print to the terminal. Running cargo build for the first time also causes Cargo to create a new file at the top level: ```Cargo.lock```. This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

We just built a project with ```cargo build``` and ran it with ```./target/debug/hello_cargo```, but we can also use ```cargo run``` to compile the code and then run the resulting executable all in one command:
```shell
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running `target/debug/hello_cargo`
Hello, world!
```

Notice that this time we didn’t see output indicating that Cargo was compiling ```hello_cargo```. Cargo figured out that the files hadn’t changed, so it just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have seen this output:
```shell
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```
Cargo also provides a command called ```cargo check```. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:
```shell
$ cargo check
Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```
Why would you not want an executable? Often, ```cargo check``` is much faster than ```cargo build```, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using ```cargo check``` will speed up the process! As such, many Rustaceans run ```cargo check``` periodically as they write their program to make sure it compiles. Then they run ```cargo build``` when they’re ready to use the executable.

Let’s recap what we’ve learned so far about Cargo:

- We can build a project using ```cargo build```.
- We can build and run a project in one step using ```cargo run```.
- We can build a project without producing a binary to check for errors using ```cargo check```.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
An additional advantage of using Cargo is that the commands are the same no matter which operating system you’re working on. So, at this point, we’ll no longer provide specific instructions for Linux and macOS versus Windows.

---
**Building for Release**
When your project is finally ready for release, you can use ```cargo build --release``` to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run ```cargo build --release``` and benchmark with the executable in target/release.
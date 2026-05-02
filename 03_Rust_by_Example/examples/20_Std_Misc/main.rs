#![allow(dead_code)]

// # 20 Std Misc

// Many other types are provided by the standard library to support things such as:
// - Threads
// - Channels
// - File I/O

fn main() {
    threads::main();
    map_reduce::main();
    channels::main();
    paths::main();
    files::main();
    processes::main();
    processes::pipes();
    processes::wait();
    arg_parse::main();
    ffi::main();
}

// ## 20.1 Threads

// Rust provides a mechanism for spawning native OS threads via the spwan function, the argument of this function is a moving closure.

mod threads {
    use std::thread;

    const NTHREADS: u32 = 10;

    // This is the main thread
    pub fn main() {
        // Make a vector to hold the children which are spawned
        let mut children = vec![];

        for i in 0..NTHREADS {
            // Spin up another thread
            children.push(thread::spawn(move || {
                println!("this is thread number {}", i);
            }));
        }

        // These threads will be scheduled by the Operating System,

        for child in children {
            // Wait for thread to finish. Return a result
            let _ = child.join();
        }
    }
}

// ### 20.1.1 Testcase: map-reduce

// Rust makes it very easy to parallelize data processing, without many of the headaches traditionally associated with it.

// The standard library provides great threading primitives out of the box. these, combine with Rust's concept of Ownership and aliasing rules, automatically preventing data races.

// The aliasing rules (one writable reference XOR many readable references) automatically prevent you from manipulating state that is visible to other threads.
// (Where synchronization is needed, there are synchronization primitives like Mutex or Channel)

// In the following example, we will calculate the sum of all digits in a block of numbers. We will do this by parcelling out chunks of the block into different threads.
// Each thread will sum its tiny block of digits, and subsequently we will sum the intermediate sum produced by each thread.

// Note that, although we're passing references across thread boundaries, Rust understands that we're only passing read-only references, and that thus
// no unsafety or data races can occur. Also because the references we're passing have 'static lifetimes, Rust understands that our data won't be destroyed
// while these threads are still running. (when you need to share non-static data between threads, you can use a smart pointer like `Arc` to keep the data alive and avoid non-static lifetimes)

mod map_reduce {
    use std::thread;

    // This is the main thread
    pub fn main() {
        // This is our data to process
        // We will calculate the sum of all digits via a threaded map-reduce algorithm.
        // Each whitespace separated chunk will be handled in a different thread.
        let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

        // Make a vector to hold the child-threads which we will spawn
        let mut children = vec![];

        // Map Phase: Divide our data into segments, and apply initial processing

        // split our data into segments for individual calculation each chunk will be a reference (&str) into the actual data
        let chunked_data = data.split_whitespace();

        // Iterate over the data segments.
        // enumerate adds the current loop index to whatever it is iterating, the resulting tuple "(index, element)" is then immediately destructured into two varaibles
        for (i, data_segment) in chunked_data.enumerate() {
            println!("data segment {} is '{}'", i, data_segment);

            // Process each data segment in a separate thread
            //
            // spawn() returns a handle to the new thread, which we MUST keep to access the returned value
            //
            // `move || -> u32` is syntax for a closure that:
            // - takes no arguments "||"
            // - takes ownership of its captured varaibles ("move")
            // - returns an unsigned 32-bit integer "-> u32"
            // Rust is smart enough to infer the "-> u32" from the closure itself, so we could have left that out.
            let t = thread::spawn(move || -> u32 {
                // Calculate the intermediate sum of this segment:
                let result = data_segment
                    // Iterate over the characters in this segment
                    .chars()
                    // convert text-characters to their numeric value
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    // sum the resulting iterator of numbers
                    .sum();

                // println! locks stdout, so no text-interleaving occurs between threads
                println!("intermediate sum for segment {} is {}", i, result);

                // "return" not needed because Rust is an "expression language", the last evaluated expression is automatically returned
                result
            });

            // Save the thread handle, so we can join on it later
            children.push(t);
        }

        // Reduce Phase: Collect our intermediate results, and combine them into a final result

        // Combine each thread's intermediate result into a single final sum
        //
        // we use the turbofish ::<> to provide sum() with a type hint
        let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
        println!("Final sum: {}", final_result);
    }

    // It is not wise to let our number of threads depend on user input data. What if the user decides to insert a lot of spaces.
    // Do we really want to spawn 2000 threads? No, that would be a DoS attack. In a real application, we would want to limit the number of threads we spawn, and have each thread process multiple data segments.
}

// ## Channels

// Rust provides asynchronous channels for communication between threads. Channels allow a unidirectional flow of information between two end-points:
// the Sender
// and the Receiver

mod channels {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    static NTHREADS: u32 = 10;

    pub fn main() {
        // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`
        // where T is the type of the message to be transferred
        // (type annotation is superfluous here, Rust can infer the type from the context)
        let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

        let mut children = Vec::new();

        for id in 0..NTHREADS {
            // The sender endpoint can be copied
            let thread_tx = tx.clone();

            // Each thread will send its id via the channel
            let child = thread::spawn(move || {
                // The thread takes ownership over `thread_tx`
                // Each thread queues a message in the channel
                thread_tx.send(id).unwrap();

                // Sending is a non-blocking operation, the thread will continue immediately after sending its message
                println!("Thread {} sent its id", id);
            });

            // Save the thread handle, so we can join on it later
            children.push(child);
        }

        // Here, all the messages are collected
        let mut ids = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            // The recv method picks a message from the channel
            // this is a blocking operation and will block the current thread until a message is available
            let id = rx.recv().unwrap();
            println!("Main thread received id {}", id);
            ids.push(id);
        }

        // Wait for all threads to finish
        for child in children {
            child.join().expect("oops! the thread panicked");
        }

        // Show the order in which the messages were sent
        println!("{ids:?}");
    }
}

// ## 20.3 Path

// The Path type represents file paths in the underlying filesystem. Across all platforms there is a single `std::path::Path` that abstracts over
// platform specific path semantics and separators.

// A Path can be created from an OsStr, and provides several methods to get information from the file/directory the path points to

// A Path is immutable. The owned version of Path is PathBuf.The relation between Path and PathBuf is similar to that of str and String:
// A PathBuf can be mutated in-place and can be dereferenced to a Path. Note that Path is *not* internally represented as an UTF-8 string, but instead
// stored as an OsString. Therefore, converting a Path to a &str is not free and may fail (an Option is returned).
// However a Path can be freely converted to an OsString, or &OsStr using into_os_string and as_os_str respectively.

mod paths {
    use std::path::Path;

    pub fn main() {
        // Create a Path from an &'static str
        let path = Path::new(".");

        // The `display` method returns a `Display`able structure
        let _display = path.display();

        // `join` merges a path with a byte container using the OS specific separator, returning a PathBuf
        let mut new_path = path.join("a").join("b").join("c");

        // `push` extends the PathBuf with a &Path
        new_path.push("d");
        new_path.push("myfile.tar.gz");

        // set_file_name updates the file name of the PathBuf
        new_path.set_file_name("package.tgz");

        // Convert the PathBuf into a string slice
        match new_path.to_str() {
            Some(s) => println!("The path is '{}'", s),
            None => println!("The path is not valid UTF-8"),
        }
    }
}

// ## 20.4 File I/O

// The File struct represents a file that has been opened (it wraps a file descriptor), and gives read and/or write access to the underlying file.

// Since many things can go wrong when doing file I/O, all the File methods return io::Result<T> type, which is an alias for Result<T, io::Error>
// This makes the failure of all I/O operations explicit. Thanks to this, the programmer can see all the failure paths, and is encouraged to handle them in a proactive manner

mod files {

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    //### 20.4.1 open

    // The open function can be used to open a file in read-only mode

    // A File owns a resource, the file-descriptor and takes care of closing the file when it is dropped.

    pub fn showcase_open() {
        // Create a path to the desired file
        let path = Path::new("hello.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        // file goes out of scope, and the "hello.txt" file is closed
    }

    // ### 20.4.2 create

    // The create function opens a file in write-only mode. If the file already exists, the old content is destroyed. Otherwise, a new file is created.

    static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    ";

    pub fn showcase_create() {
        let path = Path::new("lorem_ipsum.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write LOREM_IPSUM to the file, returns `io::Result<()>`
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // ### 20.4.3 read_lines

    // A naive approach

    // This might be a reasonable first attempt for a beginner's first implementation for reading lines from a file

    use std::fs::read_to_string;

    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string());
        }

        result
    }

    // Since the method `lines()` returns an iterator over the lines in the file, we can also perform a map inline and collect the rsults, yielding a more concise and fluent expression

    fn read_lines_v2(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect() // gather them together into a vector
    }

    // Note that in both examples, we must convert &str reference returned from lines() to the owned type String, using .to_string() or String::from respectively.

    // A more efficient approach

    // Here we pass ownership of the open File to a BufReader struct. BufReader uses an internal buffer to reduce intermediate allocations.

    // We also update read_lines to return an iterator instead of allocating new String objects in memory for each line

    use std::io::{self, BufRead};

    fn read_lines_v3<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn main() {
        // File hosts.txt must exist in the current path
        if let Ok(lines) = read_lines_v3("./hosts.txt") {
            // Consume the iterator, returns an Optional String
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
            }
        }
    }

    // This process is more efficient than creating a String in memory with all of the file's contents.
}

// ## 20.5 Child Processes

// The `process::Output`n struct represents the output of a finished child process, and the `process::Command` struct is a process builder

mod processes {
    use std::process::Command;

    pub fn main() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e);
            });

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            println!("rustc suceeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            println!("rustc failed and stderr was:\n{}", s);
        }
    }

    // ### 20.5.1 Pipes

    // The std::process::Child struct represents a child process, and exposes the stdin, stdout and stderr handles for interaction with the underlying process via pipes

    use std::io::prelude::*;
    use std::process::Stdio;

    static PANGRAM: &'static str = "the quick brown fox jumps over the lazy dog";

    pub fn pipes() {
        // Spawn the `wc` command
        let mut cmd = if cfg!(target_family = "windows") {
            let mut cmd = Command::new("powershell");
            cmd.arg("-Command")
                .arg("$input | Measure-Object -Line -Word -Character");
            cmd
        } else {
            Command::new("wc")
        };

        let process = match cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
            Err(why) => panic!("couldn't spawn wc: {}", why),
            Ok(process) => process,
        };

        // Write a string to the `stdin` of `wc`
        // `stdin` has the type `Option<ChildStdin>` but since we know this instance must have one, we can directly unwrap it
        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}", why),
            Ok(_) => println!("successfully wrote to wc stdin"),
        }

        // Because stdin does not live after the above call, it is droped and the pipe is closed.
        // This is very important, otherwise `wc` wouldn't start processing the input we just sent.

        // The stdout field also has the type `Option<ChildStdout>`, but we can also unwrap it directly
        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read wc stdout: {}", why),
            Ok(_) => println!("wc responded with:\n{}", s),
        }
    }

    // Wait

    // If you'd like to wait for a process::Child to finish, you must call Child::wait, which will return process::ExitStatus

    pub fn wait() {
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        let ecode = child.wait().expect("failed to wait on child");
        println!("child exited with: {}", ecode);
    }

    // Filesystem Operations

    // The std::fs module contains several functions that deal with the filesystem

    mod fs_ops {
        use std::fs;
        use std::fs::{File, OpenOptions};
        use std::io;
        use std::io::prelude::*;
        #[cfg(target_family = "unix")]
        use std::os::windows;
        #[cfg(target_family = "windows")]
        use std::os::windows;
        use std::path::Path;

        // A simple implementation of `% cat path`
        fn cat(path: &Path) -> io::Result<String> {
            let mut f = File::open(path)?;
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Err(why) => Err(why),
                Ok(_) => Ok(s),
            }
        }

        // A simple implementation of `% echo s > path`
        fn echo(s: &str, path: &Path) -> io::Result<()> {
            let mut f = File::create(path)?;
            f.write_all(s.as_bytes())
        }

        // A simple implementation of `% touch path` (ignores existing file)
        fn touch(path: &Path) -> io::Result<()> {
            match OpenOptions::new().create(true).write(true).open(path) {
                Err(why) => Err(why),
                Ok(_) => Ok(()),
            }
        }

        fn main() {
            println!("`mkdir a`");
            // Create a directory, returns `io::Result<()>`
            fs::create_dir("a").expect("failed to create directory");

            println!("`echo hello > a/hello.txt`");
            // Create a file and write to it, returns `io::Result<()>`
            echo("hello", &Path::new("a/hello.txt")).unwrap_or_else(|why| {
                panic!("couldn't write to a/hello.txt: {}", why);
            });

            println!("`mkdir -p a/c/d`");
            // Create a directory and all of its parent components if they are missing, returns `io::Result<()>`
            fs::create_dir_all("a/c/d").unwrap_or_else(|why| println!("! {:?}", why.kind()));

            println!("`touch a/c/e.txt`");
            // Create an empty file, returns `io::Result<()>`
            touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| println!("! {:?}", why.kind()));

            println!("ln -s ../b/txt a/c/b.txt");
            // Create a symbolic link, returns `io::Result<()>`
            #[cfg(target_family = "unix")]
            unix::fs::symlink("../b/txt", "a/c/b.txt")
                .unwrap_or_else(|why| println!("! {:?}", why.kind()));
            #[cfg(target_family = "windows")]
            windows::fs::symlink_file("../b/txt", "a/c/b.txt")
                .unwrap_or_else(|why| println!("! {:?}", why.kind()));

            println!("`cat a/c/b.txt`");
            // Read the contents of a file, returns `io::Result<String>`
            match cat(&Path::new("a/c/b.txt")) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(s) => println!("a/c/b.txt contains:\n{}", s),
            }

            println!("`ls a/c`");
            // Read the contents of a directory, returns `io::Result<Vec<PathBuf>>`
            match fs::read_dir("a/c") {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => {
                    for path in paths {
                        println!("> {:?}", path.unwrap().path());
                    }
                }
            }

            println!("rm a/c/e/txt");
            // Remove a file, returns `io::Result<()>`
            fs::remove_file("a/c/e.txt").unwrap_or_else(|why| println!("! {:?}", why.kind()));

            println!("rmdir a/c/d");
            // Remove an empty directory, returns `io::Result<()>`
            fs::remove_dir("a/c/d").unwrap_or_else(|why| println!("! {:?}", why.kind()));
        }
    }
}

// ## Program Arguments

// ### Standard Library

// The command-line arguments can be accessed using std::env::args, which returns an iterator that yields a String for each argument.

mod args {
    use std::env;

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        // The first argument is the path that was used to call the program
        println!("My path is {}", args[0]);

        // The rest of the arguments are passed command-line parameters
        for arg in &args[1..] {
            println!("I got argument '{}'", arg);
        }
    }

    // Alternatively there are numerous crates that can provide extra functionality when creating command-line applications.
    // One of the more popular command-line argument crates being clap.
}

// ### Argument Parsing

mod arg_parse {
    use std::env;

    fn increase(number: i32) {
        println!("{}", number + 1);
    }

    fn decrease(number: i32) {
        println!("{}", number - 1);
    }

    fn help() {
        println!(
            "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
        );
    }

    pub fn main() {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            // no arguments passed
            1 => {
                println!("My name is match_args. Try passing some arguments!");
            }
            // one argument passed
            2 => match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            },
            // one command and one argument passed
            3 => {
                let cmd = &args[1];
                let num = &args[2];
                // parse the number
                let number = match num.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("error: second argument not an integer");
                        help();
                        return;
                    }
                };
                // parse the command
                match &cmd[..] {
                    "increase" => increase(number),
                    "decrease" => decrease(number),
                    _ => {
                        eprintln!("error: invalid command");
                        help();
                    }
                }
            }
            // all other cases
            _ => {
                eprintln!("error: too many arguments");
                help();
            }
        }
    }
}

// ## 20.8 Foreign Function Interface

// Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign functionsust be declared inside an extern block annotation with
// a `#[link]` attribute containing the name of the foreign library

mod ffi {
    use std::fmt;

    // this extern block links to the libm library
    #[cfg(target_family = "windows")]
    #[link(name = "msvcrt")]
    extern "C" {
        // this is a foreign function
        // that computes the square root of a single precision complex number
        fn csqrtf(z: Complex) -> Complex;

        fn ccosf(z: Complex) -> Complex;
    }
    #[cfg(target_family = "unix")]
    #[link(name = "m")]
    extern "C" {
        // this is a foreign function
        // that computes the square root of a single precision complex number
        fn csqrtf(z: Complex) -> Complex;

        fn ccosf(z: Complex) -> Complex;
    }

    // Since calling foreign functions is considered unsafe, it's common to write safe wrappers around them
    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }

    pub fn main() {
        // z = -1 + 0i
        let z = Complex { re: -1.0, im: 0.0 };

        // calling a foreign function is an unsafe operation
        let z_sqrt = unsafe { csqrtf(z) };

        println!("The square root of {:?} is {:?}", z, z_sqrt);

        // calling safe API wrapped around unsafe operation
        println!("The cosine of {:?} is {:?}", z, cos(z));
    }

    // Minimal implementation of single precision complex numbers
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }

    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.im >= 0.0 {
                write!(f, "{} + {}i", self.re, self.im)
            } else {
                write!(f, "{} - {}i", self.re, -self.im)
            }
        }
    }
}

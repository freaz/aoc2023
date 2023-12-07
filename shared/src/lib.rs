use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

pub fn load_file(filename: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(filename);

    let file = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader
        .read_to_string(&mut data)
        .expect("Cannot read the file");

    data
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-output")]
        {
            println!($($arg)*);
        }
    };
}

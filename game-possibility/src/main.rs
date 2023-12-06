use data_access::file::read_lines;

fn main() {
    let path = std::env::current_dir().unwrap().display().to_string();
    print!("Path: {path}");

    read_lines("game-possibility/data/input.txt")
        .unwrap()
        .for_each(|line| {
            let line_text = line.unwrap();
            println!("{line_text}");
        });
}

use std::io;
use std::io::stdout;
use std::io::Write;

/*
    TODO: Make it so that the application takes flags
    TODO: Wrote todo task
    TODO: Remove todo tasks
    TODO: Check tasks
    TODO: Done tasks from yesterday are removed the next day
    TODO: Auto-complete
    TODO: Stdout should be at the bottom of the terminal window
*/

fn opt1() {
    println!("[OPT] 1")
}

struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }
    fn move_to (&mut self, x: usize, y: usize) {
        todo!()
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush();
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut todo_list = String::new();
    let mut lines: usize = 0;
    let mut cursor: (usize, usize) = (0, 0);
    clear_screen();
    loop {
        io::stdin().read_line(&mut buffer)?;
        match &buffer[0..buffer.len()-1] {
            "todo" | "TODO" | "Todo" | "t" | "T" => {
                buffer = "".to_string();
                todo_list += "[TODO] ";
                io::stdin().read_line(&mut todo_list);
                // TODO: move the cursor back and add space
                cursor.1 += 1; // hight increase
                print!("{todo_list}");
            },
            "quit" | "Quit" | "q" | "Q" => {
                println!("[OPT] 2");
                buffer = "".to_string();
                break;
            },
            "" => { 
                buffer = "".to_string();
            },
            _     => {
                buffer = "".to_string();
                eprintln!("[ERROR]");
            },
        }
    }
    Ok(())
}

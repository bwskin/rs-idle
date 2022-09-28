fn error(error: rs_idle::Error) -> ! {
    println!("{}", error.message());
    std::process::exit(1);
}

fn main() {
    println!(
        "{}",
        rs_idle::get_idle_time().unwrap_or_else(|e| { error(e) })
    );
}

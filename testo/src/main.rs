use chrono;

fn main() {
    // returns DateTime<Local>
    println!("{:?}", chrono::offset::Local::now());

    // returns DateTime<Utc>
    // NOTE: Available on crate feature *clock* only.
    println!("{:?}", chrono::offset::Utc::now());
}
use hashchk::get_hashrecords;

const DBNAME: &str = "actur";

#[tokio::main]
async fn main() {
    let result = get_hashrecords(DBNAME).await;
    match result {
        Ok(_cursor) => {
            println!("Got cursor");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

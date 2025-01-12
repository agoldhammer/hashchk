// use futures::stream::TryStreamExt;
use hashchk::get_hashrecords;

#[tokio::main]
async fn main() {
    let result = get_hashrecords("news").await;
    match result {
        Ok(_cursor) => {
            println!("Got cursor");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

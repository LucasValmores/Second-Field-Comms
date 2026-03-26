// using tokio because we need the multi threading stuffs


use tokio::net::{TcpListener, TcpStream};
use tokio::task;
use tokio::spawn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handleclient(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).await;
    let request = String::from_utf8_lossy((&buffer[..]));
    println!("request received:{}", request);
    stream.write_all("welcome client".as_bytes()).await.expect("uh oh somebody wasn't welcomed");

}
#[tokio::main]
async fn main() {
    let listen = TcpListener::bind("127.0.0.1:8080").await.expect("address not bound");
  //obviously self hosting
    println!("server listening on port 8080");

loop{
    match listen.accept.await{
            Ok((stream, _addr))=>{
                tokio::task::spawn(async move { handleclient(stream).await });

            }
        Err(e)=> eprintln!("no connection: {}", e)
        }
    }
}

use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::env;

fn main() {
    let argv : Vec<String> = env::args().collect();

    let power_addr : &str; //= "127.0.0.1:1000";

    if env::args().count() < 2 {
        power_addr = "127.0.0.1:1000";
    }
    else {
        power_addr = &argv[1];
    }

    match TcpStream::connect(power_addr) {
        Ok(mut stream) => {

            let mut buf = String::new();

            //std::io::stdin().read_line(&mut buf).unwrap();
            
            while match std::io::stdin().read_line(&mut buf) {
                Err(err) => {
                    println!("{}", err);
                    false
                },
                Ok(n) => {
                    if n == 0 {
                        false
                    }
                    else {
                        true
                    }
                }

            }
            {
                let command :&str = buf.trim();
                //print!("{}>>", command);

                match stream.write(command.as_bytes()) {
                    Ok(n) => println!("{}>> {} bytes", command, n),
                    Err(err) => println!("{}>> send err, {}", command, err)
                }

                let mut result = [0 as u8; 16];
                match stream.read(&mut result) {
                    Ok(n) => {
                        //std::io::stdout().write_all(&data[0..size]).unwrap();
                        //std::io::stdout().flush().unwrap();
                        let txt = from_utf8(&result[0..n]).unwrap();
                        println!("{} bytes >>[{}]", n, txt);
                    },
                    Err(e) => {
                        println!("error {}", e);
                    }
                }

                buf.clear();
            }

            stream.shutdown(std::net::Shutdown::Both).unwrap();

        }
        Err(e) => {
            println!("error connect, {}", e);
        }
    }

    
    println!("end");
}

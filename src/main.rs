
use std::{io::{self, Read, Result, Write}, net::TcpStream};
mod ffi;
use crate::ffi::{epoll_event};

mod poll;
use crate::poll::{Events, Poll};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
Host: localhost\r\n\
Connection: close\r\n\
\r\n"
    )
}

// after event occur then handel data
fn handel_events(events: &poll::Events, streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n==0 => {
                    handled_events += 1;
                    break
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }

    }

    Ok(handled_events)
}


fn main() -> Result<()> {
    let mut poll = Poll::new()?;// create epoll_queue
    let n_events = 5;// number of events


    // section to create tcp connection and send GET request

    let mut streams:Vec<TcpStream> = vec![];
    let add = "localhost:8080";// address of local server

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;// delay time to send req one after other
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(add)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;

        poll.registry().register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET);

        streams.push(stream);

    }

    // section to handel events
    let mut handeld_events = 0 ;
    while handeld_events < n_events  {
        let mut events:poll::Events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        handeld_events += (handel_events(&events, &mut streams)? ) ;
    }

    println!("FINISHED");
    Ok(())
}
use std::{io::{self,Result}, net::TcpStream};
use crate::ffi;

//=======handler for event queue
//handel struct
pub struct  Registry {
    raw_fd: i32,
}

impl Registry {
    // to register interest for an event in event queue
    pub fn register(&self, source:&TcpStream, token:usize, interests: i32) -> Result<()> {
        let mut event = ffi::epoll_event {
            events: interests as u32,
            epoll_data:token, // fd
        };
        let event_p = &mut event as *mut ffi::epoll_event;

        let operation = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd,operation, source.as_raw_fd(),event_p)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe {ffi::close(self.raw_fd)};
        if res < 0 {
            let err = io::Error::last_os_error();
            println!("{:?}",err)
        }
    }
}


//=======event queue
pub type Events = Vec<ffi::epoll_event>;
// event queue struct
pub struct Poll {
    registry: Registry, //handler
}

impl Poll {
    // create new event queue and return poll instance
    pub fn new() -> Result<Self> {
        let res = unsafe {
            // create epoll and return filedescriptor
            ffi::epoll_create(1)// argument is ignored but should be > 0
        };

        // check for filedescriptor
        if res < 0 {
            // filedescriptor does not exist
            Result::Err(io::Error::last_os_error())
        }


        Result::Ok(
            Poll {
                registry : Registry {
                    raw_fd: res
                }
            }
        )
    }

    // give reference to handler
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    // to block thread its called on unitl event is ready or time out
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let epfd = self.registry().raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;//no.of.events + extraspace allocated for vec
        let res = unsafe {
            ffi::epoll_wait(epfd, events.as_mut_ptr(), max_events,timeout)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe {events.set_len(res as usize)};// remove extra space allocated for vec

        Ok(())
    }

}


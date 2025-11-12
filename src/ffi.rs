pub const EPOLL_CTL_ADD: i32 = 1;// epoll_ctl operation
pub const EPOLLIN: i32 = 0x1;// event
pub const EPOLLET: i32 = 1 << 31;// event

#[link(name="c")]
unsafe extern "C" {
    pub fn epoll_create(size:i32) -> i32;// to create epoll event queue
    pub fn close(fd: i32) -> i32;// to close fd return by creation of Event queue
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut epoll_event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut epoll_event, maxevents: i32, timeout: i32) -> i32;

}

#[derive(Debug)]
#[repr(C,packed)]
pub struct epoll_event {
    pub(crate) events: u32,
    pub(crate) epoll_data: usize,
}

impl epoll_event {
    pub fn token(&self) ->usize {
        self.epoll_data
    }
}


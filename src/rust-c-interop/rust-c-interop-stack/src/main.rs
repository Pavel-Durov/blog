use libc::{c_uint, c_void, pthread_attr_t, pthread_create, pthread_join, pthread_t, sleep};

#[derive(Debug)]
struct ThreadRoutineArgs {
    sec: c_uint,
}

impl Drop for ThreadRoutineArgs {
    fn drop(&mut self) {
        println!("Drop. {:?}", self);
        self.sec = 0;
    }
}

extern "C" fn thread_function(arg: *mut c_void) -> *mut c_void {
    let args = unsafe { (arg as *mut ThreadRoutineArgs).as_ref().unwrap() };
    println!("Thread sleeps for {} seconds", args.sec);
    unsafe {
        sleep(args.sec as c_uint);
    }
    return std::ptr::null_mut();
}

fn main() {
    let mut handle: pthread_t = 0;
    unsafe {
        let args = &ThreadRoutineArgs { sec: 3 };
        let attr: *const pthread_attr_t = std::ptr::null();
        pthread_create(
            &mut handle,
            attr,
            thread_function,
            args as *const ThreadRoutineArgs as *mut c_void,
        );
        println!("Scheduled thread with {} seconds.", args.sec);
    }

    unsafe { pthread_join(handle, std::ptr::null_mut()) };
}

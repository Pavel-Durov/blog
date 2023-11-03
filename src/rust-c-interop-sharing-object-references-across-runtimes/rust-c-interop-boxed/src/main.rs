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
    unsafe {
        let args = arg as *mut ThreadRoutineArgs;
        println!("Thread sleeps for {} seconds", args.as_ref().unwrap().sec);
        sleep(args.as_ref().unwrap().sec as c_uint);
        Box::from_raw(args)
    };

    return std::ptr::null_mut();
}

fn main() {
    let mut handle: pthread_t = 0;
    unsafe {
        let args = Box::into_raw(Box::new(ThreadRoutineArgs { sec: 3 }));

        let attr: *const pthread_attr_t = std::ptr::null();
        pthread_create(
            &mut handle,
            attr,
            thread_function,
            args as *const ThreadRoutineArgs as *mut c_void,
        );
        println!(
            "Scheduled thread with {} seconds.",
            args.as_ref().unwrap().sec
        );
    }
    unsafe { pthread_join(handle, std::ptr::null_mut()) };
}

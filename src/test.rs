// Just me messing about with Rust to grok how it works

use std::cell::RefCell;
use std::pin::Pin;
use std::borrow::Borrow;

struct SelfRef {
    data: String,
    ptr: RefCell<Vec<*const String>>,
}

fn mk_weird() -> Pin<Box<SelfRef>> {
    let s = Box::pin(SelfRef {
        data: format!("weird flex but ok"),
        ptr: RefCell::new(Vec::new())
    });
    {
        let borrowed: &SelfRef = s.borrow();
        borrowed.ptr.borrow_mut().push( &s.data);
    }
    return s;
}

fn dodgy(s: &String) {
    println!("{}", s);
}

struct Kay {
    damn: Pin<Box<SelfRef>>
}

fn weird() -> Kay {
    let w = mk_weird();
    dodgy(unsafe { &*( *(w.ptr.borrow()).get_unchecked(0)) } );
    let z = w;
    Kay {
        damn: z
    }
}
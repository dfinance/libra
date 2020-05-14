use std::{cell::RefCell, intrinsics::transmute};
thread_local! {
    static POOL: RefCell<Option<Vec<(String, &'static str)>>> = RefCell::new(None);
}

#[derive(Debug)]
pub struct ConstPool();

impl ConstPool {
    pub fn new() -> ConstPool {
        POOL.with(|pool| {
            pool.replace(Some(vec![]));
        });
        ConstPool()
    }

    pub fn push(str: &str) -> &'static str {
        POOL.with(|pool| {
            if let Some(pool) = pool.borrow_mut().as_mut() {
                let buf = str.to_owned();
                let sr: &'static str = unsafe { transmute(buf.as_str()) };
                pool.push((buf, sr));
                sr
            } else {
                panic!("Expected ConstPool in context");
            }
        })
    }
}

impl Default for ConstPool {
    fn default() -> Self {
        ConstPool::new()
    }
}

impl Drop for ConstPool {
    fn drop(&mut self) {
        POOL.with(|pool| {
            if let Some(pool) = pool.replace(None) {
                for (_, sr) in pool {
                    let _: &str = unsafe { transmute(sr) };
                }
            }
        });
    }
}

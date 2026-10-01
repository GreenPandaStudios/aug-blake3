use std::{any::Any, panic::{catch_unwind, AssertUnwindSafe}, ptr, slice};
#[repr(C)]
pub struct NativeError { code: i32, message_length: u32, message: [u8;512] }
unsafe fn fail(error: *mut NativeError, code:i32, message:&str)->i32 {
    if !error.is_null() { let bytes=message.as_bytes(); let length=bytes.len().min(512); unsafe { (*error).code=code; (*error).message_length=length as u32; ptr::copy_nonoverlapping(bytes.as_ptr(),(*error).message.as_mut_ptr(),length); } }
    code
}
// A panic payload can itself have a throwing destructor. Destroy ordinary
// payloads; abort if that destructor panics so nothing unwinds across C.
fn contain_panic(payload: Box<dyn Any + Send>) {
    if let Err(second) = catch_unwind(AssertUnwindSafe(|| drop(payload))) {
        std::mem::forget(second);
        std::process::abort();
    }
}
#[cfg(test)]
std::thread_local! {static FORCE_PANIC:std::cell::Cell<bool>=const {std::cell::Cell::new(false)};}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aug_blake3_hash_v1(input:*const u8,length:u64,out:*mut *mut u8,count:*mut u64,error:*mut NativeError)->i32 {
    if !out.is_null(){unsafe{*out=ptr::null_mut()}} if !count.is_null(){unsafe{*count=0}} if !error.is_null(){unsafe{ptr::write_bytes(error,0,1)}}
    if out.is_null()||count.is_null()||(input.is_null()&&length!=0)||length>isize::MAX as u64 {return unsafe{fail(error,1,"Invalid hash input")}}
    match catch_unwind(AssertUnwindSafe(|| {
        let bytes=if length==0 {&[]} else {unsafe{slice::from_raw_parts(input,length as usize)}};
        let digest=blake3::hash(bytes).to_hex();let mut text=Box::new([0u8;64]);text.copy_from_slice(digest.as_bytes());
        #[cfg(test)]
        FORCE_PANIC.with(|flag|{if flag.replace(false){panic!("injected native hash panic");}});
        unsafe{*out=Box::into_raw(text) as *mut u8;*count=64;}
    })) { Ok(())=>0,Err(payload)=>{contain_panic(payload);unsafe{fail(error,2,"Rust hash operation panicked")}} }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aug_blake3_text_release_v1(text:*mut u8) {if !text.is_null(){unsafe{drop(Box::from_raw(text as *mut [u8;64]));}}}
#[cfg(test)]
mod tests {use super::*;#[test]fn actual_hash_and_cleanup(){let mut out=ptr::null_mut();let mut count=0;let mut error=NativeError{code:0,message_length:0,message:[0;512]};unsafe{assert_eq!(aug_blake3_hash_v1(b"abc".as_ptr(),3,&mut out,&mut count,&mut error),0);assert_eq!(slice::from_raw_parts(out,count as usize),b"6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");aug_blake3_text_release_v1(out);}}}

#[cfg(test)]
mod panic_tests {
    use super::*;
    use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
    struct Payload(Arc<AtomicUsize>);
    impl Drop for Payload {fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}
    #[test] fn panic_payload_is_reclaimed() {
        let counter=Arc::new(AtomicUsize::new(0));
        let payload=catch_unwind(AssertUnwindSafe(||std::panic::panic_any(Payload(counter.clone())))).unwrap_err();
        contain_panic(payload);
        assert_eq!(counter.load(Ordering::SeqCst),1);
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test] fn actual_export_contains_an_injected_panic() {
        let mut out=ptr::null_mut();let mut count=0;let mut error=NativeError{code:0,message_length:0,message:[0;512]};
        FORCE_PANIC.with(|flag|flag.set(true));
        unsafe{assert_eq!(aug_blake3_hash_v1(b"abc".as_ptr(),3,&mut out,&mut count,&mut error),2);}
        assert!(out.is_null());assert_eq!(count,0);assert_eq!(error.code,2);
        assert_eq!(&error.message[..error.message_length as usize],b"Rust hash operation panicked");
    }
    #[test] fn empty_input_matches_the_upstream_vector() {
        let mut out=ptr::null_mut();let mut count=0;let mut error=NativeError{code:0,message_length:0,message:[0;512]};
        unsafe{assert_eq!(aug_blake3_hash_v1(ptr::null(),0,&mut out,&mut count,&mut error),0);
        assert_eq!(slice::from_raw_parts(out,count as usize),b"af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262");
        aug_blake3_text_release_v1(out);}
    }
}

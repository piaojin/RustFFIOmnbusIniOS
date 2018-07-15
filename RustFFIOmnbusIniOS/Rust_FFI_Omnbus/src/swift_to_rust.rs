use libc::{c_void, c_char};
use std::thread;
use std::ops::Deref;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::{Send, Sync};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[repr(C)]
#[derive(Debug)]
pub enum DogType {
    Superdog,
    Normaldog,
}

#[repr(C)]
#[derive(Debug)]
pub struct I_Super_Dog_Delegate {
    user: *mut c_void,
    destroy: extern fn(user: *mut c_void),
    callback_with_int_arg: extern fn(user: *mut c_void, arg: i32),
}

#[repr(C)]
#[derive(Debug)]
pub struct SuperDog {
    age: i32,
    name: String,
    delegate: I_Super_Dog_Delegate,
    dog_type: DogType,
}

unsafe impl Send for I_Super_Dog_Delegate {}

unsafe impl Send for SuperDog {}

unsafe impl Sync for I_Super_Dog_Delegate {}

unsafe impl Sync for SuperDog {}

// ***方式二 start***
struct ISuperDogDelegateWrapper<'a>(&'a I_Super_Dog_Delegate);

impl<'a> Deref for ISuperDogDelegateWrapper<'a> {
    type Target = I_Super_Dog_Delegate;

    fn deref(&self) -> &I_Super_Dog_Delegate {
        &self.0
    }
}

impl<'a> Drop for ISuperDogDelegateWrapper<'a> {
    fn drop(&mut self) {
        (self.destroy)(self.user);
        println!("ISuperDogDelegateWrapper -> drop");
    }
}
// ***方式二 end***

// ***方式一 start***
// struct ISuperDogDelegateWrapper<'a> {
//     delegate: &'a I_Super_Dog_Delegate,
// }

// impl<'a> Deref for ISuperDogDelegateWrapper<'a> {
//     type Target = I_Super_Dog_Delegate;

//     fn deref(&self) -> &I_Super_Dog_Delegate {
//         &self.delegate
//     }
// }

// impl<'a> Drop for ISuperDogDelegateWrapper<'a> {
//     fn drop(&mut self) {
//         (self.delegate.destroy)(self.delegate.user);
//         println!("ISuperDogDelegateWrapper -> drop");
//     }
// }
// ***方式一 end***

impl SuperDog {

    fn new(delegate: I_Super_Dog_Delegate) -> SuperDog {
        SuperDog{
            age: 1,
            name: String::from("哈士奇🐶"),
            delegate: delegate,
            dog_type: DogType::Normaldog,
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_dog_type(&self) -> DogType {
        match self.dog_type {
            DogType::Superdog => DogType::Superdog,
            DogType::Normaldog => DogType::Normaldog,
        }
    }

    fn set_dog_type(&mut self, dog_type: DogType) {
        self.dog_type = dog_type;
    }

    fn super_dog_call_back(&self, delegate: &'static I_Super_Dog_Delegate) {
        println!("super_dog_call_back moving SwiftObject onto a new thread created by Rust");
        let super_dog_delegate_wrapper = ISuperDogDelegateWrapper(delegate);
        
        let super_dog_delegate_wrapper = Arc::new(Mutex::new(super_dog_delegate_wrapper));
        let super_dog_delegate_wrapper = Arc::clone(&super_dog_delegate_wrapper);
        
        thread::spawn(move||{
            thread::sleep(Duration::new(6, 0));
            println!("super_dog_set_delegate_to_rust thread::spawn");
            let super_dog_delegate_wrapper = super_dog_delegate_wrapper.lock().unwrap();
            (super_dog_delegate_wrapper.callback_with_int_arg)(super_dog_delegate_wrapper.user, 10);
        });
    }
}

#[no_mangle]
pub fn super_dog_create_new(delegate: I_Super_Dog_Delegate) -> *mut SuperDog {
    Box::into_raw(Box::new(SuperDog::new(delegate)))
}

#[no_mangle]
pub unsafe extern fn super_dog_get_name(ptr: *const SuperDog) -> *mut c_char {
    assert!(!ptr.is_null());
    let database = &*ptr;//unsafe
    let name = database.get_name();
    let c_name = CString::new(name).unwrap();//unsafe

    c_name.into_raw()
}

#[no_mangle]
pub unsafe extern fn super_dog_set_name(ptr: *mut SuperDog, name: *const c_char) {
    assert!(!ptr.is_null());
    let database = &mut *ptr;//unsafe
    
    let string = CStr::from_ptr(name).to_string_lossy().into_owned();//unsafe

    database.set_name(string);
}

#[no_mangle]
pub unsafe extern fn super_dog_get_dog_type(ptr: *const SuperDog) -> DogType {
    assert!(!ptr.is_null());
    let dog = &*ptr;//unsafe
    let dog_type = dog.get_dog_type();

    dog_type
}

#[no_mangle]
pub unsafe extern fn super_dog_set_dog_type(ptr: *mut SuperDog, dog_type: DogType) {
    assert!(!ptr.is_null());
    let dog = &mut *ptr;//unsafe
    dog.set_dog_type(dog_type);
}

//这种方式放线程中去回调回crash
#[no_mangle]
pub unsafe extern fn super_dog_set_delegate_to_rust(ptr: &'static SuperDog) {
    println!("super_dog_set_delegate_to_rust moving SwiftObject onto a new thread created by Rust");
    // assert!(!ptr.is_null());
    // ***方式一 start***
    // let super_dog_delegate_wrapper = ISuperDogDelegateWrapper {
    //     delegate: &ptr.delegate,
    // };
    
    // let super_dog_delegate_wrapper = Arc::new(Mutex::new(super_dog_delegate_wrapper));
    // let super_dog_delegate_wrapper = Arc::clone(&super_dog_delegate_wrapper);
    
    // thread::spawn(move||{
    //     thread::sleep(Duration::new(1000, 0));
    //     println!("super_dog_set_delegate_to_rust thread::spawn");
    //     let super_dog_delegate_wrapper = super_dog_delegate_wrapper.lock().unwrap();
    //     (super_dog_delegate_wrapper.delegate.callback_with_int_arg)(super_dog_delegate_wrapper.delegate.user, 10);
    // });
    // ***方式一 end***

    // ***方式二 start***
    let super_dog_delegate_wrapper = ISuperDogDelegateWrapper(&ptr.delegate);
    
    let super_dog_delegate_wrapper = Arc::new(Mutex::new(super_dog_delegate_wrapper));
    let super_dog_delegate_wrapper = Arc::clone(&super_dog_delegate_wrapper);
    
    thread::spawn(move||{
        thread::sleep(Duration::new(6, 0));
        println!("super_dog_set_delegate_to_rust thread::spawn");
        let super_dog_delegate_wrapper = super_dog_delegate_wrapper.lock().unwrap();
        (super_dog_delegate_wrapper.callback_with_int_arg)(super_dog_delegate_wrapper.user, 10);
    });
    // ***方式二 end***
}

#[no_mangle]
pub unsafe extern fn super_dog_call_back(ptr: &'static SuperDog) {
    println!("super_dog_call_back moving SwiftObject onto a new thread created by Rust");
    // assert!(!ptr.is_null());
    let ptr = &*ptr;//unsafe
    ptr.super_dog_call_back(&ptr.delegate);
}

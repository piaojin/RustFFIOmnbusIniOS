extern crate libc;

use libc::{c_char, uint32_t};
use std::str;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;

#[warn(non_snake_case)]
#[warn(dead_code)]
pub struct Test {
    zip_name: String,
}

//drop即是C++中的析构函数
#[warn(non_snake_case)]
impl Drop for Test {
    fn drop(&mut self) {
        println!("析构Test对象");
    }
}

impl Test {
    fn new() -> Test {
        Test {
            zip_name: String::from("😎😎😎😎😎😎"),
        }
    }

    fn get_zip_name(&self) -> &str {
        self.zip_name.as_str()
    }

    fn set_zip_name(&mut self, zip_name: String) {
        self.zip_name = zip_name;
    }
}

#[warn(non_snake_case)]
#[warn(dead_code)]
pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
    zip_name: String,
    code: u32,
    test: Test,
}

//drop即是C++中的析构函数
#[warn(non_snake_case)]
impl Drop for ZipCodeDatabase {
    fn drop(&mut self) {
        println!("析构ZipCodeDatabase对象");
    }
}

impl ZipCodeDatabase {

    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
            zip_name: String::from("😘😘😘😘😘😘"),
            code: 100,
            test: Test::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }

    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }

    fn get_code(&self) -> u32 {
        self.code
    }

    fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    fn get_zip_name(&self) -> &str {
        self.zip_name.as_str()
    }

    fn set_zip_name(&mut self, zip_name: String) {
        self.zip_name = zip_name;
    }
}

#[no_mangle]
pub extern fn zip_code_database_new() -> *mut ZipCodeDatabase {
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

#[no_mangle]
pub unsafe extern fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() { return };
    Box::from_raw(ptr);//unsafe
}

#[no_mangle]
pub unsafe extern fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    assert!(!ptr.is_null());
    let database = &mut *ptr;//unsafe
    database.populate();
}

#[no_mangle]
pub unsafe extern fn zip_code_database_population_of(ptr: *const ZipCodeDatabase, zip: *const c_char) -> uint32_t {
    assert!(!ptr.is_null());
    let database = &*ptr;//unsafe

    assert!(!zip.is_null());
    let zip = CStr::from_ptr(zip);//unsafe
    let zip_str = zip.to_str().unwrap();//unsafe
    
    database.population_of(zip_str)
}

#[no_mangle]
pub unsafe extern fn zip_code_database_get_code(ptr: *const ZipCodeDatabase) -> uint32_t {
    assert!(!ptr.is_null());
    let database = &*ptr;//unsafe

    database.get_code()
}

#[no_mangle]
pub unsafe extern fn zip_code_database_set_code(ptr: *mut ZipCodeDatabase, code: uint32_t) {
    assert!(!ptr.is_null());
    let database = &mut *ptr;//unsafe
    database.set_code(code);
}

#[no_mangle]
pub unsafe extern fn zip_code_database_get_zip_name(ptr: *const ZipCodeDatabase) -> *mut c_char {
    assert!(!ptr.is_null());
    let database = &*ptr;//unsafe
    let name = database.get_zip_name();
    let c_name = CString::new(name).unwrap();//unsafe

    c_name.into_raw()
}

#[no_mangle]
pub unsafe extern fn zip_code_database_set_zip_name(ptr: *mut ZipCodeDatabase, zip_name: *const c_char) {
    assert!(!ptr.is_null());
    let database = &mut *ptr;//unsafe
    
    let string = CStr::from_ptr(zip_name).to_string_lossy().into_owned();//unsafe

    database.set_zip_name(string);
}

#[no_mangle]
pub unsafe extern fn test_get_zip_name(ptr: *const ZipCodeDatabase) -> *mut c_char {
    assert!(!ptr.is_null());
    let database = &*ptr;//unsafe
    let name = database.test.get_zip_name();
    let c_name = CString::new(name).unwrap();//unsafe

    c_name.into_raw()
}

#[no_mangle]
pub unsafe extern fn test_set_zip_name(ptr: *mut ZipCodeDatabase, zip_name: *const c_char) {
    assert!(!ptr.is_null());
    let database = &mut *ptr;//unsafe
    
    let string = CStr::from_ptr(zip_name).to_string_lossy().into_owned();//unsafe

    database.test.set_zip_name(string);
}

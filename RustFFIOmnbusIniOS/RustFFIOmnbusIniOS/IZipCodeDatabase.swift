//
//  IZipCodeDatabase.swift
//  RustFFIOmnbus
//
//  Created by Zoey Weng on 2018/6/29.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import Foundation

//获取Rust的信息(Rust -> Swift)
class IZipCodeDatabase {
    private var zipCodeDatabasePtr: OpaquePointer?
    
    deinit {
        self.free()
    }
    
    init() {
        self.zipCodeDatabasePtr = zip_code_database_new()
    }
    
    func getCode() -> UInt32? {
        if let zipCodeDatabasePtr = self.zipCodeDatabasePtr {
            return zip_code_database_get_code(zipCodeDatabasePtr)
        }
        return nil
    }
    
    func setCode(code: UInt32) {
        if let zipCodeDatabasePtr = self.zipCodeDatabasePtr {
            zip_code_database_set_code(zipCodeDatabasePtr, code)
        }
    }
    
    func getName() -> String? {
        if let zipCodeDatabasePtr = self.zipCodeDatabasePtr {
            return String(cString: zip_code_database_get_zip_name(zipCodeDatabasePtr))
        }
        return nil
    }
    
    func setName(name: String) {
        if let zipCodeDatabasePtr = self.zipCodeDatabasePtr {
            zip_code_database_set_zip_name(zipCodeDatabasePtr, name)
        }
    }
    
    private func free() {
        if let zipCodeDatabasePtr = self.zipCodeDatabasePtr {
            zip_code_database_free(zipCodeDatabasePtr)
        }
    }
}

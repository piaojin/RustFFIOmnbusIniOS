//
//  SuperDog.swift
//  RustFFIOmnbusIniOS
//
//  Created by Zoey Weng on 2018/7/1.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

protocol SuperDogDelegate: NSObjectProtocol {
    func callbackWithArg(arg: Int32)
}

enum SuperDogType: Int {
    case superDog = 0, normalDog
    init(raw: Int) {
        self.init(rawValue: raw)!
    }
};

//Swift的信息传递给Rust,包含回调(Swift -> Rust -> callBack -> Swift)
class SuperDogViewModel {
    
    weak var delegate: SuperDogDelegate?
    
    private lazy var i_Super_Dog_Delegate: I_Super_Dog_Delegate = {
        let ownedPointer = UnsafeMutableRawPointer(Unmanaged.passRetained(self).toOpaque())
        let destroyBlock: @convention(c) (UnsafeMutableRawPointer?) -> Void = {(pointer) in
            destroy(user: pointer!)
        }
        
        let callback_with_int_argBlock: (@convention(c) (UnsafeMutableRawPointer?, Int32) -> Void)! = { (pointer, arg) in
            callback_with_int_arg(user: pointer!, arg: arg)
        }
        
        let i_Super_Dog_Delegate = I_Super_Dog_Delegate(user: ownedPointer, destroy: destroyBlock, callback_with_int_arg: callback_with_int_argBlock)
        
        return i_Super_Dog_Delegate
    }()
    
    private lazy var superDogPtr: OpaquePointer? = {
        let superDogPtr = super_dog_create_new(&self.i_Super_Dog_Delegate)
        return superDogPtr
    }()
    
    var name: String {
        get {
            return String(cString: super_dog_get_name(self.superDogPtr))
        }
        
        set {
            super_dog_set_name(self.superDogPtr, newValue)
        }
    }
    
    var age: UInt64 = 0
    
    var dogType: SuperDogType {
        get {
            return SuperDogType(raw: Int(super_dog_get_dog_type(self.superDogPtr).rawValue))
        }
        
        set {
            super_dog_set_dog_type(self.superDogPtr, DogType.init(UInt32(newValue.rawValue)))
        }
    }
    
    deinit {
        print("SuperDogViewModel being deallocated")
    }
    
    init(delegate: SuperDogDelegate) {
        self.delegate = delegate
    }
    
    //Rust回调Swift
    fileprivate func callbackWithArg(arg: Int32) {
        print("SuperDogViewModel: received callback with arg \(arg)")
        self.delegate?.callbackWithArg(arg: arg)
    }
    
    //Swift对象发送给Rust
    func sendToRust() {
//        super_dog_set_delegate_to_rust(self.superDogPtr)
        super_dog_call_back(self.superDogPtr)
    }
}

//Rust回调Swift
fileprivate func callback_with_int_arg(user: UnsafeMutableRawPointer, arg: Int32) {
    let obj: SuperDogViewModel = Unmanaged.fromOpaque(user).takeUnretainedValue()
    obj.callbackWithArg(arg: arg)
}

//Rust回调Swift用以销毁Swift对象
fileprivate func destroy(user: UnsafeMutableRawPointer) {
    let _ = Unmanaged<SuperDogViewModel>.fromOpaque(user).takeRetainedValue()
}

//
//  ViewController.swift
//  RustFFIOmnbusIniOS
//
//  Created by Zoey Weng on 2018/6/30.
//  Copyright © 2018年 piaojin. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

//    lazy var superDog: SuperDogViewModel = {
//        let superDog = SuperDogViewModel(delegate: self)
//        return superDog
//    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        print("***Rust object to Swift begin***")
        let zipCodeDatabase: IZipCodeDatabase = IZipCodeDatabase()
        print("before set name: \(String(describing: zipCodeDatabase.getName()))")
        zipCodeDatabase.setName(name: "This is new name!")
        print("after set name: \(String(describing: zipCodeDatabase.getName()))")
        print("***Rust object to Swift end***")

        print("***Swift object to Rust begin***")
        let superDog = SuperDogViewModel(delegate: self)
        superDog.name = "哈士奇🐶"
        superDog.age = 1
        print("***SuperDog name is \(superDog.name)***)\n")
        print("***SuperDog dogType is \(superDog.dogType)***\n")
        superDog.dogType = .superDog
        print("***SuperDog new dogType is \(superDog.dogType)***\n")
        superDog.sendToRust()
        print("***Swift object to Rust end***")
        
//        DispatchQueue.main.asyncAfter(deadline: DispatchTime.now() + 3) {
//            self.superDog.sendToRust()
//        }
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }
}

extension ViewController: SuperDogDelegate {
    func callbackWithArg(arg: Int32) {
        print("SuperDogDelegate: \(arg)")
    }
}


//
//  DogType.h
//  RustFFIOmnbusIniOS
//
//  Created by Zoey Weng on 2018/7/5.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#ifndef DogType_h
#define DogType_h

#include "SuperDog.h"

typedef enum DogType {
    Superdog = 0, Normaldog
} DogType;

extern DogType super_dog_get_dog_type(const Super_Dog * ptr);

extern void super_dog_set_dog_type(Super_Dog *ptr, DogType dog_type);

#endif /* DogType_h */

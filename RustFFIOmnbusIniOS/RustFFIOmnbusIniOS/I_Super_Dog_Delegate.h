//
//  I_Super_Dog_Delegate.h
//  RustFFIOmnbusIniOS
//
//  Created by Zoey Weng on 2018/7/3.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#ifndef I_Super_Dog_Delegate_h
#define I_Super_Dog_Delegate_h

#include "SuperDog.h"

typedef struct I_Super_Dog_Delegate {
    void *user;
    void (*destroy)(void *user);
    void (*callback_with_int_arg)(void *user, int32_t arg);
} I_Super_Dog_Delegate;

extern void super_dog_set_delegate_to_rust(struct Super_Dog *ptr);

extern void super_dog_set_delegate_to_rust2(I_Super_Dog_Delegate delegate);

extern void super_dog_call_back(const struct Super_Dog *ptr);

#endif /* I_Super_Dog_Delegate_h */

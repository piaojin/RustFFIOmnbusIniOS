//
//  I_Zip_Code_Database.h
//  RustFFIOmnbus
//
//  Created by Zoey Weng on 2018/6/28.
//  Copyright © 2018年 piaojin. All rights reserved.
//

#ifndef Zip_Code_Database_h
#define Zip_Code_Database_h

#ifdef __cplusplus
extern "C" {
#endif
    
#include <stdio.h>
#include <stdint.h>
    
typedef struct I_Zip_Code_Database Zip_Code_Database;

extern Zip_Code_Database * zip_code_database_new(void);

extern void zip_code_database_free(Zip_Code_Database *);

extern void zip_code_database_populate(Zip_Code_Database *);

extern uint32_t zip_code_database_population_of(const Zip_Code_Database *, const char *zip);

extern uint32_t zip_code_database_get_code(const Zip_Code_Database *);

extern void zip_code_database_set_code(Zip_Code_Database *, uint32_t code);

extern char * zip_code_database_get_zip_name(const Zip_Code_Database *);

extern void zip_code_database_set_zip_name(Zip_Code_Database *, const char *);

extern char * test_get_zip_name(const Zip_Code_Database *);

extern void test_set_zip_name(Zip_Code_Database *, const char *);
    
#ifdef __cplusplus
}
#endif

#endif /* Zip_Code_Database_h */

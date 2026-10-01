#include "aug_blake3.h"
#include <assert.h>
#include <string.h>
#include <stdio.h>
int main(void){for(int i=0;i<1000;i++){
 void *digest=NULL;uint64_t length=0;aug_native_error_v1 error={0};
 assert(aug_blake3_hash_v1("abc",3,&digest,&length,&error)==0&&length==64);
 assert(memcmp(digest,"6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",64)==0);aug_blake3_text_release_v1(digest);
 digest=NULL;assert(aug_blake3_hash_v1(NULL,1,&digest,&length,&error)!=0&&digest==NULL);
 }puts("Rust BLAKE3: known vector, invalid input and 1000 cleanup cycles passed");}

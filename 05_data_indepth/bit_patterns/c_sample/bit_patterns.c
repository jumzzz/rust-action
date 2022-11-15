#include <stdio.h>

int main() {
    
    float a = 32.0;

    float* a_ptr = &a;
    unsigned int* a_recast = (unsigned int *)a_ptr;
    unsigned int a_uint = *a_recast;

    printf("a = %.6f\n", a);
    printf("&a = %p\n", &a);
    printf("a = %d\n", a_uint);
}

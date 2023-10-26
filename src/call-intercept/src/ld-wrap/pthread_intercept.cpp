#include <pthread.h>
#include <iostream>

extern "C"
{
    extern int __real_pthread_create(pthread_t *__restrict, const pthread_attr_t *__restrict,
                                     void *(*__start_routine)(void *), void *__restrict);

    int __wrap_pthread_create(pthread_t *__restrict thread, const pthread_attr_t *__restrict attr,
                              void *(*__start_routine)(void *), void *__restrict arg)
    {
        std::cout << "Intercepted pthread_create!" << std::endl;

        // Call the real pthread_create
        int result = __real_pthread_create(thread, attr, __start_routine, arg);
        std::cout << "Running code after calling the real pthread_create..." << std::endl;

        return result;
    }
}

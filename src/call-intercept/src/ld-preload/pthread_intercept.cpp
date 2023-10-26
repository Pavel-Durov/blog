#include <dlfcn.h>
#include <pthread.h>
#include <iostream>

typedef int (*pthread_create_t)(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);

static pthread_create_t original_pthread_create = nullptr;

extern "C" int pthread_create(pthread_t *thread, const pthread_attr_t *attr, void *(*start_routine)(void *), void *arg)
{
    if (!original_pthread_create)
    {
        original_pthread_create = reinterpret_cast<pthread_create_t>(dlsym(RTLD_NEXT, "pthread_create"));
    }

    std::cout << "Intercepted pthread_create!" << std::endl;

    return original_pthread_create(thread, attr, start_routine, arg);
}

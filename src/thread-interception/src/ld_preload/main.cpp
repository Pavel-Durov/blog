#include <iostream>
#include <pthread.h>

void* thread_function(void* arg) {
    std::cout << "Thread is running." << std::endl;
    return nullptr;
}

int main() {
    pthread_t thread;
    pthread_create(&thread, nullptr, thread_function, nullptr);
    pthread_join(thread, nullptr);
    return 0;
}
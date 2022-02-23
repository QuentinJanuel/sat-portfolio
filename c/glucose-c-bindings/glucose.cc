#include <iostream>
#include <pthread.h>

extern "C" {

#include "glucose.h"

void* myThreadFun(void* vargp) {
    std::cout << "Hello from Thread" << std::endl;
    return NULL;
}

void say_hello() {
    std::cout << "Hello from Glucose" << std::endl;
    pthread_t thread_id;
    std::cout << "Before Thread" << std::endl;
    pthread_create(&thread_id, NULL, myThreadFun, NULL);
    pthread_join(thread_id, NULL);
    std::cout << "After Thread" << std::endl;
}

}

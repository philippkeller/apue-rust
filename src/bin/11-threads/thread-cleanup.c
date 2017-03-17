#include <stdio.h>
#include <pthread.h>

void cleanup(void *arg);

void * thr_fn1(void *arg) {
    printf("thread 1 start\n");
    fflush(stdout);
    pthread_cleanup_push(cleanup, "thread 1 first handler");
    pthread_cleanup_push(cleanup, "thread 1 second handler");
    printf("thread 1 push complete\n");
    fflush(stdout);
//    if (arg)
//        return((void *)1);
    pthread_cleanup_pop(0);
    pthread_cleanup_pop(0);
    return((void *)1);
}
void * thr_fn2(void *arg) {
    printf("thread 2 start\n");
    fflush(stdout);
    pthread_cleanup_push(cleanup, "thread 2 first handler");
    pthread_cleanup_push(cleanup, "thread 2 second handler");
    printf("thread 2 push complete\n");
    fflush(stdout);
    if (arg)
        pthread_exit((void *)2);
    pthread_cleanup_pop(0);
    pthread_cleanup_pop(0);
    pthread_exit((void *)2);
}
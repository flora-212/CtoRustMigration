#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t r = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_lock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void f2() {
    int x;

    pthread_mutex_lock(&r);
    n = n + 1;
    pthread_mutex_unlock(&r);

    x = pthread_mutex_trylock(&r);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&r);
    }
}

void f3() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_lock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }

    x = pthread_mutex_trylock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void f4() {
    int x;

    pthread_mutex_lock(&r);
    n = n + 1;
    pthread_mutex_unlock(&r);

    x = pthread_mutex_trylock(&r);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&r);
    }

    x = pthread_mutex_lock(&r);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&r);
    }
}

void *t_fun(void *arg) {
    f1();
    f2();
    f3();
    f4();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    printf("%d\n", n);
}

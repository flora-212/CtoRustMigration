#include <pthread.h>

#define N 3

int n1[N];

pthread_mutex_t num_mutex[N] = {
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER
};

void f1() {
    pthread_mutex_lock(&num_mutex[0]);
    pthread_mutex_lock(&num_mutex[1]);
    n1[0] = n1[0] + 1;
    n1[1] = n1[1] + 1;
    pthread_mutex_unlock(&num_mutex[1]);
    pthread_mutex_unlock(&num_mutex[0]);
}

void f2() {
    pthread_mutex_lock(&num_mutex[1]);
    pthread_mutex_lock(&num_mutex[2]);
    n1[1] = n1[1] + 1;
    n1[2] = n1[2] + 1;
    pthread_mutex_unlock(&num_mutex[2]);
    pthread_mutex_unlock(&num_mutex[1]);
}

void f3() {
    pthread_mutex_lock(&num_mutex[2]);
    pthread_mutex_lock(&num_mutex[0]);
    n1[2] = n1[2] + 1;
    n1[0] = n1[0] + 1;
    pthread_mutex_unlock(&num_mutex[0]);
    pthread_mutex_unlock(&num_mutex[2]);
}

void *t_fun(void *arg) {
    if ((long)arg == 0) {
        f1();
    } else if ((long)arg == 1) {
        f2();
    } else {
        f3();
    }
    return NULL;
}

int main() {
    pthread_t id1, id2, id3;
    pthread_create(&id1, NULL, t_fun, (void *)0);
    pthread_create(&id2, NULL, t_fun, (void *)1);
    pthread_create(&id3, NULL, t_fun, (void *)2);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
    pthread_join(id3, NULL);

    printf("%d %d %d\n", n1[0], n1[1], n1[2]);
}

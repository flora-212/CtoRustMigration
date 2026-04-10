#include <pthread.h>

int n1 = 0;
int n2 = 0;

pthread_mutex_t n1_mutex = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t n2_mutex = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);

    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);

    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_lock(&n2_mutex);
    n1 = n1 + n2;
    pthread_mutex_unlock(&n2_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);

    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_lock(&n1_mutex);
    n2 = n2 + n1;
    pthread_mutex_unlock(&n1_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);
}

void f2() {
    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);

    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);

    pthread_mutex_lock(&n2_mutex);
    n2 = n2 + 1;
    pthread_mutex_lock(&n1_mutex);
    n2 = n2 + n1;
    pthread_mutex_unlock(&n1_mutex);
    n2 = n2 + 1;
    pthread_mutex_unlock(&n2_mutex);

    pthread_mutex_lock(&n1_mutex);
    n1 = n1 + 1;
    pthread_mutex_lock(&n2_mutex);
    n1 = n1 + n2;
    pthread_mutex_unlock(&n2_mutex);
    n1 = n1 + 1;
    pthread_mutex_unlock(&n1_mutex);
}

void *t_fun(void *arg) {
    if ((long)arg == 0) {
        f1();
    } else {
        f2()
    }
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, (void *)0);
    pthread_create(&id2, NULL, t_fun, (void *)1);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    printf("%d %d\n", n1, n2);
}

#include <pthread.h>

typedef struct {
    int n1;
    pthread_mutex_t num_mutex;
} st;

st s1 = { 0, PTHREAD_MUTEX_INITIALIZER };
st s2 = { 1, PTHREAD_MUTEX_INITIALIZER };
st s3 = { 2, PTHREAD_MUTEX_INITIALIZER };

void f(st *s, st *t) {
    pthread_mutex_lock(&s->num_mutex);
    pthread_mutex_lock(&t->num_mutex);
    s->n1 = t->n1 + 1;
    t->n1 = s->n1 + 1;
    pthread_mutex_unlock(&t->num_mutex);
    pthread_mutex_unlock(&s->num_mutex);
}

void f1() {
    f(&s1, &s2);
}

void f2() {
    f(&s2, &s3);
}

void f3() {
    f(&s1, &s3);
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
    pthread_create(&id1, NULL, t_fun, (void *)2);
    pthread_create(&id2, NULL, t_fun, (void *)1);
    pthread_create(&id3, NULL, t_fun, (void *)3)
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
    pthread_join(id3, NULL);
}

#include <stdlib.h>
#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

typedef struct {
    ss *s1;
    ss *s2;
} args;

void inc(ss *s) {
    s->n = s->n + 1;
}

void f1(ss *s1, ss *s2) {
    pthread_mutex_lock(&s1->m);
    pthread_mutex_lock(&s2->m);
    inc(s1);
    inc(s2);
    pthread_mutex_unlock(&s2->m);
    pthread_mutex_unlock(&s1->m);
}

void f2(ss *s1, ss *s2) {
    pthread_mutex_lock(&s2->m);
    pthread_mutex_lock(&s1->m);
    inc(s1);
    inc(s2);
    pthread_mutex_unlock(&s1->m);
    pthread_mutex_unlock(&s2->m);
}

void *t_fun1(void *arg) {
    args *a = (args *)arg;
    f1(a->s1, a->s2);
    return NULL;
}

void *t_fun2(void *arg) {
    args *a = (args *)arg;
    f2(a->s1, a->s2);
    return NULL;
}

int main() {
    ss *s1, *s2;

    s1 = malloc(sizeof(ss));
    s2 = malloc(sizeof(ss));
    s1->n = 0; s2->n = 0;
    args a = {s1, s2};

    pthread_mutex_init(&s1->m, NULL);
    pthread_mutex_init(&s2->m, NULL);

    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun1, &a);
    pthread_create(&id2, NULL, t_fun2, &a);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    printf("%d %d\n", s1->n, s2->n);
}

#ifndef LIBPCI_RS_TEST_H
#define LIBPCI_RS_TEST_H

#include <stdio.h>

#define TEST_ASSERT(expr, msg) { \
    if(!expr) {                  \
        printf(msg);             \
        return 1;                \
    }                            \
}

#endif //LIBPCI_RS_TEST_H

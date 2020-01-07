#include <cstdint>

// this function extern "C" so we don't have to worry about demangling
extern "C" {

// this function never throws, and should always return a positive number
int doesnt_throw(int a) {
    volatile int x = 0;
    volatile bool b = false;
    try {
        x = 10;
        if(b) throw (int32_t)20;
    } catch (...) {
        return -1;
    }
    x++;
    try {
        if(b) throw (int32_t)20;
        if (x + a < 100) {
            return 1;
        } else {
            return 2;
        }
    } catch (...) {
        return -2;
    }
}

// remaining functions are not extern "C", to test demangling
}

// this function either returns 2 or throws 20
int throw_uncaught(volatile int a) {
    if (a % 2) {
        return 2;
    } else {
        throw (int32_t)20;
    }
}

// this function may return 1 or 2, or throw 3 or 4
int throw_multiple_values(volatile int a) {
    switch (a % 4) {
        case 1: return 1;
        case 2: return 2;
        case 3: throw (int32_t)3;
        default: throw (int32_t)4;
    }
}

// this function either returns 2 or throws 20
int throw_uncaught_wrongtype(volatile int a) {
    try {
        if (a % 2) {
            return 2;
        } else {
            throw (int32_t)20;
        }
    } catch (unsigned char c) {
        return 10;
    }
}

// here's a void function that may throw a value
__attribute__((noinline)) void throw_uncaught_void(volatile int* a) {
    if (*a == 0) {
        *a = 1;
    } else {
        throw (int32_t)20;
    }
}

// this function either returns 1 or throws 20
int throw_uncaught_caller(int a) {
    volatile int x = a;
    throw_uncaught_void(&x);
    return 1;
}

// here we can either return 2 or 5
int throw_and_catch_wildcard(bool shouldthrow) {
    try {
        if(shouldthrow) throw (int32_t)20;
        return 2;
    } catch (...) {
        return 5;
    }
}

// here we can either return 2 or 20
int throw_and_catch_val(bool shouldthrow) {
    try {
        if(shouldthrow) throw (int32_t)20;
        return 2;
    } catch (int e) {
        return e;
    }
}

// here we should still return either 2 or 20
int throw_and_catch_in_caller(bool shouldthrow) {
    volatile int x = 2;
    try {
        if(shouldthrow) throw_uncaught_void(&x);
    } catch (int e) {
        return e;
    }
    return 2;
}

// here we should return 2 or throw 20
int throw_and_rethrow_in_caller(bool shouldthrow) {
    volatile int x = 2;
    try {
        if(shouldthrow) throw_uncaught_void(&x);
    }
    catch (int e) {
        throw;
    }
    return 2;
}

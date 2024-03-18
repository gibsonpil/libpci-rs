// Copyright (c) 2024 Gibson Pilconis. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#pragma once

// Some operating systems don't have direct facilities for reading PCI
// information (i.e. Haiku/BeOS). In such cases we have to utilize CPU
// level calls to directly read from I/O ports. These calls are taken from the
// MUSL source code because I'm too lazy to deal with AT&T ASM syntax and annoying
// inline ASM directives. Also, NASM > GAS.

namespace cpu {
    __attribute__((always_inline))
    void outb(unsigned char __val, unsigned short __port) {
        __asm__ volatile ("outb %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((always_inline))
    void outw(unsigned short __val, unsigned short __port) {
        __asm__ volatile ("outw %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((always_inline))
    void outl(unsigned int __val, unsigned short __port) {
        __asm__ volatile ("outl %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((always_inline))
    unsigned char inb(unsigned short __port) {
        unsigned char __val;
        __asm__ volatile ("inb %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }

    __attribute__((always_inline))
    unsigned short inw(unsigned short __port) {
        unsigned short __val;
        __asm__ volatile ("inw %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }

    __attribute__((always_inline))
    unsigned int inl(unsigned short __port) {
        unsigned int __val;
        __asm__ volatile ("inl %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }
}

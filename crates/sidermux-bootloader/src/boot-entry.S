.syntax unified
.arm

.section .boot.vectors, "ax", %progbits
.balign 32

    b boot_entry           /* Reset */
    b boot_exception       /* Undefined instruction */
    b boot_exception       /* Supervisor call */
    b boot_exception       /* Prefetch abort */
    b boot_exception       /* Data abort */
    b boot_exception       /* Reserved slot */
    b boot_exception       /* IRQ */
    b boot_exception       /* FIQ */

.section .text.boot_exception, "ax", %progbits
.balign 4
.type boot_exception, %function

boot_exception:
    b boot_exception

.size boot_exception, . - boot_exception

.section .boot.entry, "ax", %progbits
.balign 4
.global boot_entry
.type boot_entry, %function

boot_entry:
    b {runtime_start}

.size boot_entry, . - boot_entry
/*
 * Cyclone V SoC memory map — physical view.
 *
 * Physical addresses and capacities are a SoC responsibility. The OCRAM
 * region below doubles as the single allocatable boot arena consumed by the
 * boot profile: sections must allocate only through this region. Overlapping
 * physical windows must never be declared as separate allocatable regions.
 *
 * TODO(datasheet): separate physical OCRAM capacity from the boot arena size
 * once the Boot ROM reservation is confirmed against the Cyclone V handbook.
 */
MEMORY
{
    /* DRAM (rwx) : ORIGIN = 0x00000000, LENGTH = 1024M
    H2F (rw) : ORIGIN = 0xC0000000, LENGTH = 960M
    LWH2F (rw) : ORIGIN = 0xFF200000, LENGTH = 2M */
    OCRAM (rwx) : ORIGIN = 0xFFFF0000, LENGTH = 60k
}

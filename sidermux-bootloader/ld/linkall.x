/*
 * Application layout for the bootable image.
 *
 * Generic over staged names: sidermux-target-memory.x comes from the
 * selected SoC (physical view); sidermux-boot.x and
 * sidermux-boot-reserve.x come from the selected boot profile (arena,
 * vectors, header slot, trailer reservation). The REGION_* aliases point at
 * the single allocatable boot arena declared by the SoC's memory script;
 * sections must allocate only through that region — overlapping physical
 * windows must never become separate allocatable MEMORY regions.
 *
 * ENTRY names the selected profile's handoff slot symbol (boot_entry for
 * the Cyclone V profile).
 */
INCLUDE sidermux-target-memory.x

REGION_ALIAS("REGION_TEXT", OCRAM);
REGION_ALIAS("REGION_RODATA", OCRAM);
REGION_ALIAS("REGION_DATA", OCRAM);
REGION_ALIAS("REGION_BSS", OCRAM);
REGION_ALIAS("REGION_STACK", OCRAM);

ENTRY(boot_entry)

__stack_size = 4K;

INCLUDE sidermux-boot.x

INCLUDE sidermux-text.x
INCLUDE sidermux-rodata.x
INCLUDE sidermux-unwind.x
INCLUDE sidermux-data.x
INCLUDE sidermux-boot-reserve.x
INCLUDE sidermux-bss.x
INCLUDE sidermux-stack.x
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
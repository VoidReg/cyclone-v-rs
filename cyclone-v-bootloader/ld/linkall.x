INCLUDE cyclone-v-memory.x

REGION_ALIAS("REGION_TEXT", OCRAM);
REGION_ALIAS("REGION_RODATA", OCRAM);
REGION_ALIAS("REGION_DATA", OCRAM);
REGION_ALIAS("REGION_BSS", OCRAM);
REGION_ALIAS("REGION_STACK", OCRAM);

ENTRY(boot_entry)

__stack_size = 4K;

INCLUDE boot.x

INCLUDE cyclone-v-text.x
INCLUDE cyclone-v-rodata.x
INCLUDE cyclone-v-data.x
INCLUDE cyclone-v-bss.x

INCLUDE stack.x
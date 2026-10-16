ENTRY(boot_entry)

SECTIONS
{
    .boot_vectors ORIGIN(OCRAM) :
    {
        __vectors_start = .;
        KEEP(*(.boot.vectors))
    } > OCRAM

    .boot_header (ORIGIN(OCRAM) + 0x40) :
    {
        LONG(0);
        LONG(0);
        LONG(0);
    } > OCRAM

    .boot_entry (ORIGIN(OCRAM) + 0x4C) :
    {
        KEEP(*(.boot.entry))
    } > OCRAM
}

ASSERT(SIZEOF(.boot_vectors) > 0,
    "Boot vectors are missing");

ASSERT(SIZEOF(.boot_vectors) <= 0x40,
    "Boot vectors overlap the header");

ASSERT(SIZEOF(.boot_entry) == 4,
    "Boot entry must contain one 4-byte ARM branch");

ASSERT(boot_entry == ADDR(.boot_entry),
    "boot_entry must be at the start of the entry section");
SECTIONS
{
    .boot_pack_reserve (NOLOAD) :
    {
        __boot_payload_end = .;
        . = ALIGN(. + 4, 16);
        __boot_packed_end = .;
    } > OCRAM
}

ASSERT(((__boot_packed_end - __boot_image_start) % 16) == 0,
       "Boot image must be a multiple of 16 bytes");

ASSERT((__boot_packed_end - __boot_image_start) <= 60K,
       "Boot image exceeds 60 KiB");
SECTIONS
{
    .text ALIGN(4) :
    {
        *(.text .text.*)
    } > REGION_TEXT
}
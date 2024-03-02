/* Linker script for the STM32L452 */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 160K
}

/* Initial 64K Flash is used to store kernel functions and
 * initial 512 bytes of RAM is used to store kernel data. */
__privileged_functions_region_size__  = 16K;
__privileged_data_region_size__       = 16K;

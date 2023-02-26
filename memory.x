/* Change this as required for your MCU */

MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  CCRAM : ORIGIN = 0x10000000, LENGTH = 32K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
}

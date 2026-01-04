# Booting into Allwinner FEL mode

!!! warning
    Booting into FEL mode requires shorting out a resistor on the board. You can damage or kill your board if you're not careful!

!!! danger
    Making mistakes while in FEL mode CAN break your board! Be careful!

!!! info "Hardware Setup Required"
    Before following this guide, ensure you have properly connected your board for FEL access. See the [FEL & UART Bench Setup Guide](../hardware/CC1/fel-uart-setup.md) for detailed hardware connection instructions including:
    
    - USB connection options (USB-C cable or soldered header)
    - UART setup
    - Power and safety considerations

## Prerequisites

### Install xfel Tool

Clone and build the allwinner-xfel repository:

```bash
git clone https://gitlab.alerv.net/clone/allwinner-xfel.git
cd allwinner-xfel
make
```

### Windows: Install USB Driver

If you're on Windows, you'll need to install the WinUSB driver:

1. Use [Zadig](https://zadig.akeo.ie/) to install the WinUSB driver. 
    - ![img](assets/DriverInstall.jpg){ width="400" }
1. Check the USB ID of the device. It has to be ``1F3A`` ``EFE8``.
1. Rename the unknown device to a name of your choice by enabling "Edit"
1. Make sure the correct driver is selected: ``WinUSB``
1. Click the install button

## Entering FEL Mode

There are two methods to boot into FEL mode:

### Method 1: Hardware Reset (From Power-On)

Press the reset button while shorting R53 with tweezers. The board should boot into FEL mode and connect to your PC.

![FEL_MODE](../assets/FelMode.jpg){ width="400" }

When successful, you should see on the UART console:

```
[25]HELLO! BOOT0 is starting!
[27]BOOT0 commit : 4d16602
```

### Method 2: Software Boot (From u-Boot)

If you already have u-Boot running and UART access, you can boot directly to FEL mode by issuing this command in u-Boot:

```
efex
```

This will reboot the system directly into FEL mode without needing to short the resistor.

## Working with FEL Mode

### Initialize DDR

Once in FEL mode, initialize the DDR memory:

```bash
./xfel ddr r528-s3
```

The UART console should display:

```
[33211]fes begin commit:4d16602
[33214]set pll start
[33220]periph0 has been enabled
[33223]set pll end
[33225][pmu]: bus read error
[33228]board init ok
[33230]beign to init dram
[33232]ZQ value = 0x2f
[33234]get_pmu_exist() = -1
[33237]ddr_efuse_type: 0xf
[33239]trefi:7.8ms
[33242][AUTO DEBUG] single rank and full DQ!
[33246]ddr_efuse_type: 0xf
[33248]trefi:7.8ms
[33251][AUTO DEBUG] rank 0 row = 13
[33254][AUTO DEBUG] rank 0 bank = 8
[33257][AUTO DEBUG] rank 0 page size = 2 KB
[33261]DRAM BOOT DRIVE INFO: V0.33
[33265]DRAM CLK = 792 MHz
[33267]DRAM Type = 3 (2:DDR2,3:DDR3)
[33270]DRAMC read ODT  off.
[33273]DRAM ODT value: 0x42.
[33276]ddr_efuse_type: 0xf
[33279]DRAM SIZE =128 M
[33281]dram_tpr4:0x0
[33283]PLL_DDR_CTRL_REG:0xf8004100
[33286]DRAM_CLK_REG:0xc0000000
[33289][TIMING DEBUG] MR2= 0x18
[33293]DRAM simple test OK.
[33295]rtc standby flag is 0x0, super standby flag is 0x0
[33301]init dram ok
```

### Load and Execute u-Boot

Load a u-Boot image to DRAM via FEL:

```bash
xfel write 0x43000000 uboot239.bin
```

!!! note
    The `uboot239.bin` file can be found in the [cc-fw-tools](https://github.com/OpenCentauri/cc-fw-tools) repository at `/extra-stuff/emmc/uboot239.bin`.

Execute the loaded u-Boot:

```bash
xfel exec 0x43000b50
```

Hold down `s` on the UART console to boot into interactive mode.

## eMMC Recovery

Once you have u-Boot running, you can recover the eMMC from a USB stick.

!!! tip "Complete eMMC Recovery Guide"
    For detailed instructions on eMMC recovery, backup, and restore procedures, see the [EMMC_RESTORE.md](https://github.com/OpenCentauri/cc-fw-tools/blob/main/docs/EMMC_RESTORE.md) documentation in the [cc-fw-tools](https://github.com/OpenCentauri/cc-fw-tools) repository.
    
    This guide covers:
    
    - Loading files from USB stick
    - Flashing to eMMC partitions
    - Creating custom `.scr` scripts for automated recovery
    - Pre-built scripts for full eMMC backup and restore (found in `./extra-stuff/emmc/`)

With u-Boot, you can load files from a USB stick and flash them to any eMMC partition by creating and building custom `.txt` scripts into `.scr` files.
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

## Boot to FEL mode
Press the reset button while shorting R53 with tweezers.
the board should boot into FEL mode and connect to your PC.

![FEL_MODE](../assets/FelMode.jpg){ width="400" }

## Installing driver

1. Use [Zadig](https://zadig.akeo.ie/) to install the WinUSB driver. 
    - ![img](assets/DriverInstall.jpg){ width="400" }
1. Check the USB ID of the device. It has to be ``1F3A`` ``EFE8``.
1. rename the unknown device to a name of your choice by enabling "Edit"
1. Make sure the correct driver is selected: ``WinUSB``
1. Click the install button
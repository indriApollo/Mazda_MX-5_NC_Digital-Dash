# Mazda MX-5 NC Digital Dash

A collection of projects that collect vehicle and gnss metrics to be displayed on a digital dash. 

## DigitalDash

An Avalonia C# project that displays vehicle metrics and lap times.

## MX5MetricsService

A small Linux C service that monitors a Mazda MX-5 NC's canbus and makes supported vehicle metrics
available over shared memory.  
The canbus is accessed via an STN1110 board connected to the car's OBD-2 port.

## UbloxChronoService

A small Linux C service that monitors a Ublox Gnss module to compute lap times and make them available over shared memory.

### References

https://www.scantool.net/scantool/downloads/98/stn11xx21xx_frpm-c.pdf

https://github.com/timurrrr/RaceChronoDiyBleDevice/blob/master/can_db/mazda_mx5_nc.md  
http://www.madox.net/blog/projects/mazda-can-bus/

### Create a fake virtual serial port pair on Linux

`socat -dd pty,rawer,echo=0 pty,rawer,echo=0`

Will create two serial ports at `/dev/pts/*`


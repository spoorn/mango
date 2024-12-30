# DataFixerUpper

Minecraft supports backwards compatibility via DataFixerUpper, which is open
sourced: https://github.com/Mojang/DataFixerUpper

This essentially just runs conversions on data between versions so that players can use the same world save across
different versions of Minecraft (mostly for updating versions).

We have method stubs and very limited implementation here, and I'm not sure if it's worth implementing this in full as
the system is quite complex. Perhaps we will just support versions moving forward, but not backport.

# Minecraft Codecs

Codecs in Minecraft are just the methods that Minecraft uses to encode/decode various data formats to in-game data
structs. For example, the `level.dat` data file is encoded using the NBT format, and after this is parsed via gzip and
the custom NBT logic from Minecraft, a Codec is used to extract the data from that Nbt CompoundTag.

More information can be found here: https://wiki.fabricmc.net/tutorial:codec
and https://docs.minecraftforge.net/en/latest/datastorage/codecs/

Mojang has their own codec system open sourced: https://github.com/Mojang/DataFixerUpper. It is quite complex and the
full capabilities of it may not be necessary for our purposes.

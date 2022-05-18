# Hex-Dump

Mix of Hexdump and Hexeditor formatting

## Currenty it supports

-> 1 byte formatting 

-> ASCII view range (33 - 126), rest of the chars are represented as '.'

-> Proper Alignment with 16 bytes Memory View


## Run

```

cargo run -- <filename>

./hex_dump <filename>

```

## Example Output

[0002c1f0] ba 08 00 00 00 4c 89 e7 48 89 de ff 15 4f 79 02 .....L..H....Oy.
[0002c200] 00 4d 85 f6 0f 84 c5 00 00 00 49 c1 e6 03 4f 8d .M........I...O.
[0002c210] 34 f6 31 db 48 8b 6c 24 10 eb 09 48 83 c3 48 49 4.1.H.l$...H..HI
[0002c220] 39 de 74 47 48 83 7c 1d 08 00 75 ef 48 8b 44 1d 9.tGH.|...u.H.D.
[0002c230] 30 48 c1 e0 03 48 8d 34 80 48 85 f6 74 10 48 8b 0H...H.4.H..t.H.
[0002c240] 7c 1d 28 ba 08 00 00 00 ff 15 02 79 02 00 48 8b |.(........y..H.
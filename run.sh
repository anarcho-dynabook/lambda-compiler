cargo run > output.asm
nasm -f elf64 output.asm
gcc -no-pie output.o -o output -nostartfiles
./output
echo $?

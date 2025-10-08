cargo run > output.asm
nasm -f macho64 output.asm
gcc -no-pie output.o -o output
./output
echo $?

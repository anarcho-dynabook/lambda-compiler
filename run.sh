cargo run > output.asm
nasm -f macho64 output.asm
clang output.o -o output
./output
echo $?

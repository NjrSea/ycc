filepath="${1%.*}"
filename=$(basename $filepath)
rm "$filename.s" &> /dev/null
echo "\n\n\n\n"
clang -fverbose-asm -S "$1" -O2 -o "$filename.s"  
cat tmp_arm.s
echo "\n\n\n\n"
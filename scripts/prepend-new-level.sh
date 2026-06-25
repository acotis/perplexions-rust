
cd ../level-gen
cargo run
cd ../perplexions-rust
cp ../level-gen/levels-experimental.txt /tmp/levels-experimental.txt
echo "——————————" >> /tmp/levels-experimental.txt
cat ./src/levels-experimental.txt >> /tmp/levels-experimental.txt
mv /tmp/levels-experimental.txt ./src/levels-experimental.txt


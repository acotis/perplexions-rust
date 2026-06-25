
echo "step 1"

cd ../level-gen
cargo run
cd ../perplexions-rust

echo "step 2"
pwd

cp ../level-gen/levels-experimental.txt /tmp/levels-experimental.txt
echo "step 3"
echo "——————————" >> /tmp/levels-experimental.txt
echo "step 4"
cat ./src/levels-experimental.txt >> /tmp/levels-experimental.txt
echo "step 5"
mv /tmp/levels-experimental.txt ./src/levels-experimental.txt


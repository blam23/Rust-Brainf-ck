echo -e \\033c
pushd bf-cli
pushd bf-lib
cargo build
cargo test
popd
cargo build
rm ../output.txt
cargo run "test-scripts/helloworld.bf" >> ../output.txt
popd
cat output.txt

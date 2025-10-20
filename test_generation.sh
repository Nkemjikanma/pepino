
#!/bin/bash
set -e  # Exit on error

echo "🧪 Testing pepino generation..."

# Clean up any previous test
rm -rf test-pepino-project

# Build pepino
cargo build

# Generate test project
./target/debug/pepino new test-pepino-project <<EOF
test-pepino-project
1
1
EOF

# Try to build the generated project
cd test-pepino-project

echo "✅ Project generated"

# Check if key files exist
test -f Cargo.toml || (echo "❌ Missing Cargo.toml" && exit 1)
test -f server/Cargo.toml || (echo "❌ Missing server/Cargo.toml" && exit 1)
test -f client/package.json || (echo "❌ Missing client/package.json" && exit 1)

echo "✅ Key files exist"

# Try to compile server
cargo build || (echo "❌ Server failed to compile" && exit 1)

echo "✅ Server compiles"

# Try to install client deps
cd client
npm install || (echo "❌ Client failed to install" && exit 1)

echo "✅ Client installs"

echo "✅ All tests passed!"

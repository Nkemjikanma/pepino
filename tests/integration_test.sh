
#!/bin/bash
set -e

echo "🧪 Running integration tests..."

# Clean up any previous test
rm -rf test-integration-project

# Build pepino
echo "📦 Building pepino..."
cargo build --release

# Generate test project
echo "🏗️  Generating test project..."
./target/release/pepino new test-integration-project <<EOF
test-integration-project
EOF

cd test-integration-project

# Verify files exist
echo "✅ Checking generated files..."
test -f Cargo.toml || (echo "❌ Missing Cargo.toml" && exit 1)
test -f server/Cargo.toml || (echo "❌ Missing server/Cargo.toml" && exit 1)
test -f client/package.json || (echo "❌ Missing client/package.json" && exit 1)
test -f justfile || (echo "❌ Missing justfile" && exit 1)
test -f .env.example || (echo "❌ Missing .env.example" && exit 1)

# Verify variable replacement worked
echo "✅ Checking variable replacement..."
grep -q 'name = "test-integration-project"' server/Cargo.toml || (echo "❌ PROJECT_NAME not replaced in server/Cargo.toml" && exit 1)
grep -q '"name": "test-integration-project-client"' client/package.json || (echo "❌ PROJECT_NAME not replaced in package.json" && exit 1)

# Try to compile server
echo "✅ Compiling server..."
cargo build || (echo "❌ Server failed to compile" && exit 1)

# Try to install client deps
echo "✅ Installing client dependencies..."
cd client
npm install || (echo "❌ Client npm install failed" && exit 1)

# Try to build client
echo "✅ Building client..."
npm run build || (echo "❌ Client build failed" && exit 1)

cd ../..

echo "✅ All integration tests passed!"

# Clean up
rm -rf test-integration-project

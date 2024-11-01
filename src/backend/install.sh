#!/bin/bash

echo "Checking if Rust is installed..."

if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed"
fi

echo "Building the project..."
cargo build --release

echo "Creating run script..."
cat > run.sh << 'EOF'
#!/bin/bash
echo "Starting Sensex Backend..."
./target/release/sensex-backend
EOF

chmod +x run.sh

echo "Installation complete!"
echo "You can now run the application using: ./run.sh"

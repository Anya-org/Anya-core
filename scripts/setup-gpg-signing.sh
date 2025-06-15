#!/bin/bash
# Setup GPG commit signing for enhanced security

set -e

echo "🔐 Setting up GPG commit signing for Anya Core"

# Check if GPG is installed
if ! command -v gpg &> /dev/null; then
    echo "❌ GPG is not installed. Please install GPG first."
    exit 1
fi

# Check if Git is configured
if ! git config user.name &> /dev/null || ! git config user.email &> /dev/null; then
    echo "❌ Git user.name and user.email must be configured first."
    echo "Run: git config --global user.name 'Your Name'"
    echo "Run: git config --global user.email 'your.email@example.com'"
    exit 1
fi

USER_NAME=$(git config user.name)
USER_EMAIL=$(git config user.email)

echo "📧 Configured for: $USER_NAME <$USER_EMAIL>"

# Check if user already has a GPG key
echo "🔍 Checking for existing GPG keys..."
if gpg --list-secret-keys --keyid-format LONG | grep -q "$USER_EMAIL"; then
    echo "✅ Found existing GPG key for $USER_EMAIL"
    KEY_ID=$(gpg --list-secret-keys --keyid-format LONG | grep -A 1 "$USER_EMAIL" | grep "sec" | sed 's/.*\/\([A-F0-9]*\).*/\1/')
    echo "🔑 Key ID: $KEY_ID"
else
    echo "🔧 Generating new GPG key..."
    
    # Generate GPG key with batch mode
    cat > /tmp/gpg-batch << EOF
%echo Generating GPG key for Anya Core
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: $USER_NAME
Name-Email: $USER_EMAIL
Expire-Date: 2y
%no-protection
%commit
%echo Done
EOF

    gpg --batch --generate-key /tmp/gpg-batch
    rm /tmp/gpg-batch
    
    KEY_ID=$(gpg --list-secret-keys --keyid-format LONG | grep -A 1 "$USER_EMAIL" | grep "sec" | sed 's/.*\/\([A-F0-9]*\).*/\1/')
    echo "✅ Generated new GPG key with ID: $KEY_ID"
fi

# Configure Git to use GPG signing
echo "⚙️ Configuring Git for GPG signing..."
git config --global user.signingkey $KEY_ID
git config --global commit.gpgsign true
git config --global tag.forceSignAnnotated true

# Export public key for GitHub
echo "📤 Exporting public key..."
gpg --armor --export $KEY_ID > ~/.anya-core-gpg-public-key.asc

echo ""
echo "✅ GPG commit signing setup complete!"
echo ""
echo "📋 Next steps:"
echo "1. Add the following public key to your GitHub account:"
echo "   GitHub Settings → SSH and GPG keys → New GPG key"
echo ""
echo "2. Copy the public key from: ~/.anya-core-gpg-public-key.asc"
echo "   Or run: cat ~/.anya-core-gpg-public-key.asc"
echo ""
echo "3. Your commits will now be automatically signed with GPG"
echo ""
echo "🔐 Key fingerprint:"
gpg --fingerprint $KEY_ID | grep -A 1 "Key fingerprint"

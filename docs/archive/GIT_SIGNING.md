---
title: "Git_signing"
description: "Documentation for Git_signing"
---

[AIR-3][AIS-3][BPC-3][RES-3]


# Git Commit Signing Guide

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This document explains how to set up and use GPG signing for commits in the Anya Core repository.

## Why Sign Commits?

Signing commits provides verification that the commits were actually created by you. This is important for security and authentication, especially in open-source projects.

## Requirements

- Git (version 2.0 or higher)
- GPG (GNU Privacy Guard)

## Setup Instructions

### Automatic Setup

We provide scripts to automate the setup process:

- **Windows/PowerShell**: Run `.\configure-git-signing.ps1`
- **Linux/Mac**: Run `./configure-git-signing.sh`

These scripts will:
1. Configure Git with the proper user name and email
2. Help you select an existing GPG key or create a new one
3. Configure Git to use the selected key for signing
4. Enable commit signing by default

### Manual Setup

#### 1. Check if you have existing GPG keys

```bash
gpg --list-secret-keys --keyid-format LONG
```

#### 2. Create a new GPG key (if needed)

```bash
gpg --full-generate-key
```

- Select RSA and RSA
- Key size of 4096 bits
- Set an expiration date (or no expiration)
- Enter your information (use "bo_thebig" as name and "botshelomokokoka@gmail.com" as email)
- Set a secure passphrase

#### 3. Configure Git to use your key

Find your key ID from the output of the first command:

```bash
# Example output
# sec   rsa4096/ABC123DEF456GHI7 2023-01-01 [SC]
#       0123456789ABCDEF0123456789ABCDEF01234567
# uid                 [ultimate] Your Name <your.email@example.com>
```

The key ID is the part after "rsa4096/" (e.g., ABC123DEF456GHI7).

Configure Git with your key:

```bash
git config --global user.signingkey YOUR_KEY_ID
git config --global commit.gpgsign true
```

#### 4. Set up GPG in your environment

**Windows**: You might need to tell Git where to find the GPG executable:
```bash
git config --global gpg.program "C:/Program Files (x86)/GnuPG/bin/gpg.exe"
```

**macOS**: You might need to tell Git to use GPG2:
```bash
git config --global gpg.program gpg2
```

## Using Git Signing

### Committing with Signatures

With `commit.gpgsign` set to true, all your commits will be automatically signed. You can also manually sign a commit:

```bash
git commit -S -m "Your commit message"
```

### Adding Your GPG Key to GitHub

1. Export your public key:
```bash
gpg --armor --export YOUR_KEY_ID
```

2. Copy the entire output (including the BEGIN and END lines)

3. Go to GitHub → Settings → SSH and GPG keys → New GPG key

4. Paste your key and save

## Retroactively Signing Previous Commits

If you have existing commits that need to be signed, we provide scripts to help with this process:

- **Windows/PowerShell**: Run `.\scripts\sign-previous-commits.ps1`
- **Linux/Mac**: Run `./scripts/sign-previous-commits.sh`

These scripts will help you identify and sign previous commits in your branch. By default, they examine the last 10 commits and provide a safe way to rewrite your Git history by adding proper GPG signatures.

### Usage Examples

**Windows**:
```powershell
# Show help
.\scripts\sign-previous-commits.ps1 -h

# Sign the last 5 commits
.\scripts\sign-previous-commits.ps1 -CommitCount 5

# Dry run to preview the process without making changes
.\scripts\sign-previous-commits.ps1 -DryRun
```

**Linux/Mac**:
```bash
# Show help
./scripts/sign-previous-commits.sh -h

# Sign the last 5 commits
./scripts/sign-previous-commits.sh -c 5

# Dry run to preview the process without making changes
./scripts/sign-previous-commits.sh -d
```

### Important Notes on Retroactive Signing

- **Force Push Required**: After signing previous commits, you'll need to force push your branch.
- **Caution with Shared Branches**: Only use retroactive signing on branches that haven't been used by other contributors, as it rewrites Git history.
- **Public Repositories**: For public repositories, consider only signing new commits going forward rather than rewriting history.

## Troubleshooting

### "secret key not available"

This usually means the email in your Git config doesn't match the email in your GPG key. Make sure they match exactly.

### "gpg failed to sign the data"

On some systems, you might need to use:
```bash
export GPG_TTY=$(tty)
```

Add this to your `.bashrc` or `.zshrc` file to make it permanent.

### Windows-specific issues

If you're having issues on Windows, try setting the GPG program path:
```bash
git config --global gpg.program "C:/Program Files (x86)/GnuPG/bin/gpg.exe"
```

## Commit Signing in Repository Scripts

The `commit_push.ps1` and `commit_push.sh` scripts in this repository have been updated to automatically detect if GPG signing is configured and will use it when available.

## Further Reading

- [GitHub Documentation on Signing Commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)
- [Git Documentation on Signing](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work) 
## See Also

- [Related Document](#related-document)


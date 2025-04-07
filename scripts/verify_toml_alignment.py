#!/usr/bin/env python3
"""
Script to verify Toml file alignment across the project.
[AIS-3][BPC-3]
"""
import sys
import toml
from pathlib import Path

def check_cargo_alignment():
    """Check alignment of Cargo.toml files."""
    root_toml_path = Path("Cargo.toml")
    privacy_toml_path = Path("packages/privacy/Cargo.toml")
    
    if not root_toml_path.exists() or not privacy_toml_path.exists():
        print("Error: Required Cargo.toml files not found")
        return False
    
    try:
        root_toml = toml.load(root_toml_path)
        privacy_toml = toml.load(privacy_toml_path)
        
        # Check workspace.dependencies
        if "workspace" not in root_toml or "dependencies" not in root_toml["workspace"]:
            print("Error: Missing workspace.dependencies in root Cargo.toml")
            return False
        
        # Check secp256k1 version consistency
        if "secp256k1" in root_toml["workspace"]["dependencies"]:
            root_secp_version = root_toml["workspace"]["dependencies"]["secp256k1"]["version"]
            
            # If not using workspace inheritance in privacy package
            if "secp256k1" in privacy_toml["dependencies"]:
                if isinstance(privacy_toml["dependencies"]["secp256k1"], dict) and "version" in privacy_toml["dependencies"]["secp256k1"]:
                    privacy_secp_version = privacy_toml["dependencies"]["secp256k1"]["version"]
                    if privacy_secp_version != root_secp_version and "workspace" not in privacy_toml["dependencies"]["secp256k1"]:
                        print(f"Version mismatch for secp256k1: {root_secp_version} vs {privacy_secp_version}")
                        return False
        
        # Check workspace.package
        if "workspace" not in root_toml or "package" not in root_toml["workspace"]:
            print("Error: Missing workspace.package in root Cargo.toml")
            return False
        
        print("Cargo.toml files are properly aligned!")
        return True
    
    except Exception as e:
        print(f"Error checking Cargo alignment: {e}")
        return False

def check_other_toml_files():
    """Check other Toml files for basic validity."""
    toml_files = [
        Path("book.toml"), 
        Path("Cross.toml"),
        Path("deny.toml"),
        Path("rust-toolchain.toml")
    ]
    
    valid_files = []
    invalid_files = []
    
    for file_path in toml_files:
        if file_path.exists():
            try:
                toml.load(file_path)
                valid_files.append(file_path)
            except Exception as e:
                invalid_files.append((file_path, str(e)))
    
    if invalid_files:
        for file_path, error in invalid_files:
            print(f"Error in {file_path}: {error}")
        return False
    
    print(f"Verified {len(valid_files)} additional Toml files successfully")
    return True

def check_workspace_alignment(root_toml):
    """Check all Cargo.toml files against workspace standards."""
    violations = []
    
    # 1. Verify workspace inheritance patterns
    for cargo_path in Path('.').rglob('**/Cargo.toml'):
        if 'target/' in str(cargo_path) or cargo_path == Path('Cargo.toml'):
            continue
            
        try:
            pkg_toml = toml.load(cargo_path)
            
            # Check package metadata inheritance
            pkg_meta = pkg_toml.get('package', {})
            for field in ['version', 'edition', 'authors', 'license', 'repository']:
                if f"{field}.workspace" not in pkg_meta:
                    violations.append(f"{cargo_path}: Missing {field}.workspace = true")
            
            # Verify dependency versions match workspace
            deps = pkg_toml.get('dependencies', {})
            workspace_deps = root_toml['workspace']['dependencies']
            
            for dep, config in deps.items():
                if dep in workspace_deps:
                    if isinstance(config, dict) and "workspace" not in config:
                        ws_version = workspace_deps[dep].get('version', '')
                        if 'version' in config and config['version'] != ws_version:
                            violations.append(
                                f"{cargo_path}: {dep} version {config['version']} ≠ workspace {ws_version}"
                            )
                            
        except Exception as e:
            violations.append(f"{cargo_path}: Parse error - {str(e)}")
    
    return violations

def check_bip_compliance():
    """Verify BIP feature flags across packages."""
    required_bips = {
        'bip174': ['packages/core', 'packages/privacy'],
        'bip341': ['packages/core', 'dependencies/anya-bitcoin'],
        'bip353': ['packages/privacy']
    }
    
    missing = []
    for bip, paths in required_bips.items():
        for p in paths:
            cargo_path = Path(p) / "Cargo.toml"
            if not cargo_path.exists():
                continue
                
            pkg_toml = toml.load(cargo_path)
            features = pkg_toml.get('features', {})
            
            if bip not in features.get('default', []):
                missing.append(f"{cargo_path}: Missing {bip} in default features")

    return missing

def check_workspace_inheritance():
    """Verify required fields exist in workspace.package."""
    required_fields = [
        'version', 'edition', 'authors',
        'description', 'repository', 'license'
    ]
    
    root_toml_path = Path("Cargo.toml")
    if not root_toml_path.exists():
        print("Error: Root Cargo.toml file not found")
        return False
        
    try:
        root_toml = toml.load(root_toml_path)
        
        if "workspace" not in root_toml or "package" not in root_toml["workspace"]:
            print("Error: Missing workspace.package in root Cargo.toml")
            return False
            
        workspace_pkg = root_toml["workspace"]["package"]
        missing_fields = []
        for field in required_fields:
            if field not in workspace_pkg:
                missing_fields.append(field)
                
        if missing_fields:
            print(f"Error: Missing fields in workspace.package: {', '.join(missing_fields)}")
            return False
            
        print("Workspace inheritance configuration is valid!")
        return True
    except Exception as e:
        print(f"Error checking workspace inheritance: {e}")
        return False

def validate_workspace_paths():
    """Verify all workspace members exist on filesystem."""
    root_toml_path = Path("Cargo.toml")
    if not root_toml_path.exists():
        print("Error: Root Cargo.toml file not found")
        return False
        
    try:
        root_toml = toml.load(root_toml_path)
        
        if "workspace" not in root_toml or "members" not in root_toml["workspace"]:
            print("Error: Missing workspace.members in root Cargo.toml")
            return False
            
        members = root_toml["workspace"]["members"]
        missing = []
        for member in members:
            if not Path(member).exists():
                missing.append(member)
        
        if missing:
            print(f"Missing workspace members: {', '.join(missing)}")
            return False
            
        # Required critical members
        required_members = [
            "dependencies/anya-bitcoin",
            "packages/core", 
            "packages/privacy"
        ]
        
        missing_critical = []
        for member in required_members:
            if not Path(member).exists():
                missing_critical.append(member)
                
        if missing_critical:
            print(f"Missing critical workspace members: {', '.join(missing_critical)}")
            return False
            
        print("All workspace member paths are valid!")
        return True
    except Exception as e:
        print(f"Error validating workspace paths: {e}")
        return False

def main():
    """Main function to verify all Toml files."""
    cargo_aligned = check_cargo_alignment()
    other_toml_valid = check_other_toml_files()
    workspace_inheritance_valid = check_workspace_inheritance()
    workspace_paths_valid = validate_workspace_paths()
    
    if "--full-scan" in sys.argv:
        # Load root toml for more detailed checks
        try:
            root_toml = toml.load("Cargo.toml")
            violations = check_workspace_alignment(root_toml)
            if violations:
                print("Workspace alignment issues found:")
                for v in violations:
                    print(f"  - {v}")
                return 1
                
            missing_bips = check_bip_compliance()
            if missing_bips:
                print("BIP compliance issues found:")
                for m in missing_bips:
                    print(f"  - {m}")
                return 1
        except Exception as e:
            print(f"Error during full scan: {e}")
            return 1
    
    if (cargo_aligned and other_toml_valid and 
            workspace_inheritance_valid and workspace_paths_valid):
        print("All Toml files are properly aligned with Bitcoin Development Framework v2.5!")
        return 0
    else:
        print("Toml alignment issues detected!")
        return 1

if __name__ == "__main__":
    sys.exit(main())
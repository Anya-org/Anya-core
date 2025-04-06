#!/usr/bin/env python3
"""
Test platform-specific configuration loading
[AIS-3][BPC-3]
"""
import os
import sys
import platform
import yaml
from pathlib import Path

def determine_platform():
    """Determine the current platform"""
    system = platform.system().lower()
    if system == "windows":
        return "windows"
    elif system in ["darwin", "linux"]:
        return "unix"
    else:
        print(f"Warning: Unknown platform {system}, defaulting to unix")
        return "unix"

def load_platform_config(platform_type):
    """Load the platform-specific configuration"""
    config_dir = Path("config/platform")
    config_file = config_dir / f"{platform_type}.yaml"
    
    if not config_file.exists():
        print(f"Error: Platform config file not found: {config_file}")
        return None
    
    try:
        with open(config_file, "r", encoding="utf-8") as f:
            config = yaml.safe_load(f)
            return config
    except Exception as e:
        print(f"Error loading config: {e}")
        return None

def expand_path_variables(path_str):
    """Expand environment variables in paths"""
    # Handle Windows-style variables
    if path_str.startswith("%") and path_str.find("%", 1) > 0:
        var_name = path_str[1:path_str.find("%", 1)]
        var_value = os.environ.get(var_name, "")
        path_str = path_str.replace(f"%{var_name}%", var_value)
    
    # Handle Unix-style variables
    if "$" in path_str:
        for var_name, var_value in os.environ.items():
            path_str = path_str.replace(f"${var_name}", var_value)
            path_str = path_str.replace(f"$HOME", os.path.expanduser("~"))
    
    return path_str

def main():
    """Test platform configuration loading"""
    print(f"System: {platform.system()}")
    print(f"Python version: {sys.version}")
    
    platform_type = determine_platform()
    print(f"Detected platform type: {platform_type}")
    
    config = load_platform_config(platform_type)
    if not config:
        print("Failed to load platform configuration")
        return
    
    print("\nConfiguration loaded successfully:")
    print(f"  Platform: {platform_type}")
    
    if "paths" in config:
        print("\nPaths:")
        for key, path in config["paths"].items():
            expanded_path = expand_path_variables(path)
            print(f"  {key}: {path}")
            print(f"    → {expanded_path}")
    
    if "security" in config:
        print("\nSecurity:")
        for key, value in config["security"].items():
            print(f"  {key}: {value}")
    
    if "network" in config:
        print("\nNetwork:")
        for key, value in config["network"].items():
            print(f"  {key}: {value}")

if __name__ == "__main__":
    main()
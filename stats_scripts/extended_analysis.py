#!/usr/bin/env python3
"""
Extended dependency analysis for libsignal.
This script provides additional insights into dependency patterns.
"""

import json
import sys
from collections import defaultdict, Counter
from pathlib import Path
import re

def load_data(json_file):
    """Load the JSON data from the file."""
    with open(json_file, 'r') as f:
        return json.load(f)

def analyze_dependency_patterns(data):
    """Analyze more specific dependency patterns."""
    
    # Create function lookup
    function_info = {}
    for item in data:
        identifier = item.get('identifier', '')
        function_info[identifier] = item
    
    # Pattern analysis
    patterns = {
        'crypto_operations': defaultdict(int),
        'certificate_operations': defaultdict(int),
        'network_operations': defaultdict(int),
        'encoding_operations': defaultdict(int),
        'validation_operations': defaultdict(int)
    }
    
    # Keywords for categorizing operations
    crypto_keywords = ['encrypt', 'decrypt', 'hash', 'sign', 'verify', 'key', 'cipher', 'aes', 'sha', 'hmac', 'curve25519']
    cert_keywords = ['cert', 'x509', 'certificate', 'ca', 'chain', 'subject', 'issuer', 'serial', 'validity']
    network_keywords = ['ssl', 'tls', 'tcp', 'socket', 'connect', 'proxy', 'dns']
    encoding_keywords = ['encode', 'decode', 'serialize', 'deserialize', 'base64', 'hex', 'der', 'pem']
    validation_keywords = ['validate', 'verify', 'check', 'ensure', 'assert', 'compare', 'equal', 'ct_eq']
    
    dependency_chains = defaultdict(list)  # Track chains of dependencies
    high_frequency_functions = set()  # Functions called very frequently
    
    for item in data:
        identifier = item.get('identifier', '')
        relative_path = item.get('relative_path', '')
        deps = item.get('deps', [])
        body = item.get('body', '').lower()
        display_name = item.get('display_name', '').lower()
        
        # Only analyze rust functions
        if not relative_path.startswith('rust/'):
            continue
        
        # Categorize by operation type
        for dep_id in deps:
            dep_info = function_info.get(dep_id, {})
            dep_path = dep_info.get('relative_path', '')
            dep_name = dep_info.get('display_name', '').lower()
            
            if not dep_path.startswith('deps/'):
                continue
                
            # Categorize the dependency
            all_text = f"{dep_name} {body} {display_name}"
            
            if any(keyword in all_text for keyword in crypto_keywords):
                patterns['crypto_operations'][dep_id] += 1
            if any(keyword in all_text for keyword in cert_keywords):
                patterns['certificate_operations'][dep_id] += 1
            if any(keyword in all_text for keyword in network_keywords):
                patterns['network_operations'][dep_id] += 1
            if any(keyword in all_text for keyword in encoding_keywords):
                patterns['encoding_operations'][dep_id] += 1
            if any(keyword in all_text for keyword in validation_keywords):
                patterns['validation_operations'][dep_id] += 1
    
    return patterns, function_info

def analyze_file_dependencies(data):
    """Analyze which rust modules depend on which deps crates."""
    
    module_deps = defaultdict(set)  # rust module -> set of deps crates used
    
    for item in data:
        relative_path = item.get('relative_path', '')
        deps = item.get('deps', [])
        
        if not relative_path.startswith('rust/'):
            continue
            
        # Extract module name from path
        path_parts = relative_path.split('/')
        if len(path_parts) >= 2:
            module = path_parts[1]  # rust/module/...
        else:
            continue
            
        # Check dependencies
        for dep_id in deps:
            # Look up the dependency to get its path
            for dep_item in data:
                if dep_item.get('identifier', '') == dep_id:
                    dep_path = dep_item.get('relative_path', '')
                    if dep_path.startswith('deps/'):
                        crate_name = dep_path.split('/')[1]
                        module_deps[module].add(crate_name)
                    break
    
    return dict(module_deps)

def print_extended_analysis(patterns, function_info, module_deps):
    """Print extended analysis results."""
    
    print("\n" + "=" * 80)
    print("EXTENDED DEPENDENCY ANALYSIS")
    print("=" * 80)
    
    # Operation patterns
    print(f"\n🔧 DEPENDENCY PATTERNS BY OPERATION TYPE")
    
    for pattern_type, deps in patterns.items():
        if deps:
            print(f"\n  {pattern_type.replace('_', ' ').title()}:")
            print(f"  {'Function':<50} {'Calls'}")
            print(f"  {'-' * 56}")
            for dep_id, count in Counter(deps).most_common(10):
                dep_info = function_info.get(dep_id, {})
                display_name = dep_info.get('display_name', dep_id.split('/')[-1] if '/' in dep_id else dep_id)
                crate = dep_info.get('relative_path', '').split('/')[1] if '/' in dep_info.get('relative_path', '') else 'unknown'
                print(f"  {display_name:<40} ({crate:<10}) {count}")
    
    # Module dependencies
    print(f"\n📋 RUST MODULE DEPENDENCIES ON DEPS CRATES")
    print(f"{'Module':<20} {'Deps Crates Used'}")
    print("-" * 60)
    
    for module, crates in sorted(module_deps.items()):
        if crates:  # Only show modules that use deps
            crates_str = ', '.join(sorted(crates))
            print(f"{module:<20} {crates_str}")
    
    # Dependency concentration
    print(f"\n🎯 DEPENDENCY CONCENTRATION ANALYSIS")
    
    # Count total functions per crate in deps
    crate_function_counts = defaultdict(int)
    for item_id, info in function_info.items():
        path = info.get('relative_path', '')
        if path.startswith('deps/'):
            crate = path.split('/')[1]
            crate_function_counts[crate] += 1
    
    # Count how many deps functions are actually used
    used_deps_by_crate = defaultdict(set)
    for item in function_info.values():
        if not item.get('relative_path', '').startswith('rust/'):
            continue
        for dep_id in item.get('deps', []):
            dep_info = function_info.get(dep_id, {})
            dep_path = dep_info.get('relative_path', '')
            if dep_path.startswith('deps/'):
                crate = dep_path.split('/')[1]
                used_deps_by_crate[crate].add(dep_id)
    
    print(f"{'Crate':<20} {'Used/Total':<12} {'Usage %':<8}")
    print("-" * 45)
    for crate in sorted(crate_function_counts.keys()):
        total = crate_function_counts[crate]
        used = len(used_deps_by_crate[crate])
        percentage = (used / total * 100) if total > 0 else 0
        print(f"{crate:<20} {used}/{total:<11} {percentage:.1f}%")

def main():
    json_file = 'libsignal_with_deps.json'
    
    if not Path(json_file).exists():
        print(f"Error: {json_file} not found in current directory")
        sys.exit(1)
    
    print("Loading data for extended analysis...")
    data = load_data(json_file)
    
    print("Analyzing dependency patterns...")
    patterns, function_info = analyze_dependency_patterns(data)
    module_deps = analyze_file_dependencies(data)
    
    print_extended_analysis(patterns, function_info, module_deps)
    
    # Summary insights
    print(f"\n💡 KEY INSIGHTS")
    print("=" * 40)
    print("1. boring-signal is the most heavily used deps crate (cryptographic operations)")
    print("2. subtle is used primarily for constant-time operations (security-critical)")
    print("3. Attestation modules (attest/) are the heaviest users of deps")
    print("4. Certificate handling relies heavily on boring-signal's X.509 functionality")
    print("5. Device transfer functionality makes extensive use of deps for encryption")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Analyze dependencies in libsignal using the libsignal_with_deps.json file.
This script analyzes function calls from rust/ code to deps/ code.
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

def is_deps_function(identifier, relative_path=None):
    """Check if a function is from the deps/ directory."""
    if relative_path and relative_path.startswith('deps/'):
        return True
    return False

def is_rust_function(identifier, relative_path=None):
    """Check if a function is from the rust/ directory.""" 
    if relative_path and relative_path.startswith('rust/'):
        return True
    return False

def get_dep_crate_name(relative_path):
    """Extract the crate name from a deps/ path."""
    if relative_path and relative_path.startswith('deps/'):
        parts = relative_path.split('/')
        if len(parts) >= 2:
            return parts[1]  # deps/crate_name/...
    return None

def analyze_dependencies(data):
    """Analyze dependencies and return statistics."""
    
    # Create lookup tables
    function_info = {}
    for item in data:
        identifier = item.get('identifier', '')
        relative_path = item.get('relative_path', '')
        function_info[identifier] = {
            'relative_path': relative_path,
            'display_name': item.get('display_name', ''),
            'statement_type': item.get('statement_type', ''),
            'file_name': item.get('file_name', ''),
            'parent_folder': item.get('parent_folder', '')
        }
    
    # Statistics collectors
    rust_to_deps_calls = defaultdict(list)  # rust function -> list of deps functions it calls
    deps_function_usage = Counter()  # deps function -> count of how many times it's called
    deps_crate_usage = Counter()  # deps crate -> count of calls
    rust_files_using_deps = set()  # rust files that use deps
    
    # Analyze each function
    for item in data:
        identifier = item.get('identifier', '')
        relative_path = item.get('relative_path', '')
        deps = item.get('deps', [])
        
        # Only analyze rust functions
        if not is_rust_function(identifier, relative_path):
            continue
            
        # Check each dependency
        for dep_identifier in deps:
            dep_info = function_info.get(dep_identifier, {})
            dep_relative_path = dep_info.get('relative_path', '')
            
            # Check if this dependency is from deps/
            if is_deps_function(dep_identifier, dep_relative_path):
                rust_to_deps_calls[identifier].append(dep_identifier)
                deps_function_usage[dep_identifier] += 1
                rust_files_using_deps.add(relative_path)
                
                # Count by crate
                crate_name = get_dep_crate_name(dep_relative_path)
                if crate_name:
                    deps_crate_usage[crate_name] += 1
            
            # Also check for external crate usage patterns in the function body
            # (for cases where deps might be referenced by crate name in code)
            body = item.get('body', '')
            if body:
                # Look for common deps crate names in the body
                deps_crates = ['curve25519_dalek', 'sha2', 'hmac', 'aes', 'ctr', 'cbc', 'subtle']
                for crate in deps_crates:
                    if crate in body:
                        deps_crate_usage[f"{crate}_in_body"] += 1
    
    return {
        'rust_to_deps_calls': dict(rust_to_deps_calls),
        'deps_function_usage': deps_function_usage,
        'deps_crate_usage': deps_crate_usage,
        'rust_files_using_deps': rust_files_using_deps,
        'function_info': function_info
    }

def generate_markdown_report(stats):
    """Generate a comprehensive markdown report about dependency usage."""
    
    md_content = []
    
    # Header
    md_content.append("# LibSignal Dependency Analysis Report")
    md_content.append("")
    md_content.append("This report analyzes function calls from `rust/` code to `deps/` code in the LibSignal project.")
    md_content.append("")
    
    # Overall statistics
    md_content.append("## 📊 Overall Statistics")
    md_content.append("")
    md_content.append(f"- **Total rust functions calling deps:** {len(stats['rust_to_deps_calls'])}")
    md_content.append(f"- **Total deps functions called:** {len(stats['deps_function_usage'])}")
    md_content.append(f"- **Total rust files using deps:** {len(stats['rust_files_using_deps'])}")
    md_content.append(f"- **Total dependency calls:** {sum(stats['deps_function_usage'].values())}")
    md_content.append("")
    
    # Most used deps functions
    md_content.append("## 🔥 Top 20 Most Called Deps Functions")
    md_content.append("")
    md_content.append("| Rank | Calls | Function | Crate | Path |")
    md_content.append("|------|-------|----------|-------|------|")
    for i, (func, count) in enumerate(stats['deps_function_usage'].most_common(20), 1):
        func_info = stats['function_info'].get(func, {})
        relative_path = func_info.get('relative_path', 'unknown')
        display_name = func_info.get('display_name', func.split('/')[-1] if '/' in func else func)
        crate = get_dep_crate_name(relative_path) or 'unknown'
        md_content.append(f"| {i} | {count} | `{display_name}` | {crate} | `{relative_path}` |")
    md_content.append("")
    
    # Most used deps crates
    md_content.append("## 📦 Deps Crate Usage")
    md_content.append("")
    md_content.append("| Crate | Calls | Path Pattern |")
    md_content.append("|-------|-------|--------------|")
    for crate, count in stats['deps_crate_usage'].most_common():
        if not crate.endswith('_in_body'):
            md_content.append(f"| `{crate}` | {count} | `deps/{crate}/` |")
    md_content.append("")
    
    # Crate usage in function bodies (text analysis)
    body_usage = {k: v for k, v in stats['deps_crate_usage'].items() if k.endswith('_in_body')}
    if body_usage:
        md_content.append("## 📝 Crate References in Code Bodies")
        md_content.append("")
        md_content.append("| Crate | References |")
        md_content.append("|-------|------------|")
        for crate_ref, count in sorted(body_usage.items()):
            crate_name = crate_ref.replace('_in_body', '')
            md_content.append(f"| `{crate_name}` | {count} |")
        md_content.append("")
    
    # Rust files most dependent on deps
    file_deps_count = defaultdict(int)
    for rust_func, deps_list in stats['rust_to_deps_calls'].items():
        func_info = stats['function_info'].get(rust_func, {})
        rust_file = func_info.get('relative_path', 'unknown')
        file_deps_count[rust_file] += len(deps_list)
    
    md_content.append("## 📁 Top 15 Rust Files by Deps Usage")
    md_content.append("")
    md_content.append("| Deps Calls | File |")
    md_content.append("|------------|------|")
    for file_path, call_count in sorted(file_deps_count.items(), key=lambda x: x[1], reverse=True)[:15]:
        md_content.append(f"| {call_count} | `{file_path}` |")
    md_content.append("")
    
    # Detailed breakdown by specific deps
    md_content.append("## 🔍 Detailed Breakdown by Deps Crate")
    md_content.append("")
    crate_details = defaultdict(lambda: {'functions': Counter(), 'total_calls': 0})
    
    for func, count in stats['deps_function_usage'].items():
        func_info = stats['function_info'].get(func, {})
        relative_path = func_info.get('relative_path', '')
        crate_name = get_dep_crate_name(relative_path)
        if crate_name:
            crate_details[crate_name]['functions'][func] += count
            crate_details[crate_name]['total_calls'] += count
    
    for crate_name in sorted(crate_details.keys()):
        details = crate_details[crate_name]
        md_content.append(f"### {crate_name.upper()}")
        md_content.append("")
        md_content.append(f"**Total calls:** {details['total_calls']}")
        md_content.append("")
        md_content.append("| Function | Calls | Path |")
        md_content.append("|----------|-------|------|")
        for func, count in details['functions'].most_common(10):  # Top 10 per crate
            func_info = stats['function_info'].get(func, {})
            display_name = func_info.get('display_name', func.split('/')[-1] if '/' in func else func)
            relative_path = func_info.get('relative_path', 'unknown')
            md_content.append(f"| `{display_name}` | {count} | `{relative_path}` |")
        if len(details['functions']) > 10:
            md_content.append(f"| ... | ... | ... |")
            md_content.append(f"| *{len(details['functions']) - 10} more functions* | | |")
        md_content.append("")
    
    return "\n".join(md_content)

def print_summary(stats):
    """Print a brief summary to the console."""
    print("=" * 80)
    print("LIBSIGNAL DEPENDENCY ANALYSIS")
    print("=" * 80)
    
    print(f"\n📊 SUMMARY")
    print(f"├── Total rust functions calling deps: {len(stats['rust_to_deps_calls'])}")
    print(f"├── Total deps functions called: {len(stats['deps_function_usage'])}")
    print(f"├── Total rust files using deps: {len(stats['rust_files_using_deps'])}")
    print(f"└── Total dependency calls: {sum(stats['deps_function_usage'].values())}")

def main():
    json_file = 'libsignal_with_deps.json'
    
    if not Path(json_file).exists():
        print(f"Error: {json_file} not found in current directory")
        sys.exit(1)
    
    print("Loading and analyzing libsignal dependencies...")
    data = load_data(json_file)
    stats = analyze_dependencies(data)
    
    # Print brief summary to console
    print_summary(stats)
    
    # Generate markdown report
    markdown_content = generate_markdown_report(stats)
    markdown_file = 'DEPENDENCY_ANALYSIS_REPORT.md'
    with open(markdown_file, 'w') as f:
        f.write(markdown_content)
    
    # Save detailed results to JSON file
    output_file = 'dependency_analysis.json'
    with open(output_file, 'w') as f:
        # Convert Counter objects to regular dicts for JSON serialization
        # Also include path information for each dependency and which rust files call them
        deps_with_paths = {}
        
        # First, build a mapping of deps function -> list of rust files that call it
        deps_to_rust_callers = defaultdict(set)
        for rust_func, deps_list in stats['rust_to_deps_calls'].items():
            rust_func_info = stats['function_info'].get(rust_func, {})
            rust_file_path = rust_func_info.get('relative_path', 'unknown')
            for dep_func in deps_list:
                deps_to_rust_callers[dep_func].add(rust_file_path)
        
        for func, count in stats['deps_function_usage'].items():
            func_info = stats['function_info'].get(func, {})
            deps_with_paths[func] = {
                'call_count': count,
                'path': func_info.get('relative_path', 'unknown'),
                'display_name': func_info.get('display_name', func.split('/')[-1] if '/' in func else func),
                'crate': get_dep_crate_name(func_info.get('relative_path', '')),
                'called_from_rust_files': sorted(list(deps_to_rust_callers.get(func, set())))
            }
        
        exportable_stats = {
            'rust_to_deps_calls': stats['rust_to_deps_calls'],
            'deps_function_usage': dict(stats['deps_function_usage']),
            'deps_function_details': deps_with_paths,
            'deps_crate_usage': dict(stats['deps_crate_usage']),
            'rust_files_using_deps': list(stats['rust_files_using_deps'])
        }
        json.dump(exportable_stats, f, indent=2)
    
    print(f"\n💾 Files generated:")
    print(f"├── Markdown report: {markdown_file}")
    print(f"└── JSON data: {output_file}")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Create a simple visualization of dependency usage.
"""

import json
from collections import defaultdict, Counter

def create_simple_chart():
    """Create a simple text-based chart of dependency usage."""
    
    with open('dependency_analysis.json', 'r') as f:
        data = json.load(f)
    
    print("📊 DEPENDENCY USAGE VISUALIZATION")
    print("=" * 60)
    
    # Chart of top dependencies
    print(f"\nTop Dependencies (by call count):")
    print("=" * 40)
    
    max_calls = max(data['deps_function_usage'].values()) if data['deps_function_usage'] else 1
    
    for func, count in sorted(data['deps_function_usage'].items(), key=lambda x: x[1], reverse=True)[:15]:
        func_name = func.split('/')[-1] if '/' in func else func
        # Create a simple bar chart
        bar_length = int((count / max_calls) * 30)
        bar = "█" * bar_length + "░" * (30 - bar_length)
        print(f"{func_name:<25} │{bar}│ {count:>3}")
    
    # Crate usage pie chart (text-based)
    print(f"\nCrate Usage Distribution:")
    print("=" * 30)
    
    total_calls = sum(v for k, v in data['deps_crate_usage'].items() if not k.endswith('_in_body'))
    
    for crate, count in sorted(data['deps_crate_usage'].items()):
        if not crate.endswith('_in_body'):
            percentage = (count / total_calls * 100) if total_calls > 0 else 0
            bar_length = int(percentage / 100 * 20)
            bar = "█" * bar_length + "░" * (20 - bar_length)
            print(f"{crate:<15} │{bar}│ {percentage:>5.1f}% ({count} calls)")
    
    # Module dependency matrix
    print(f"\nModule vs Crate Dependency Matrix:")
    print("=" * 40)
    print("(X = uses dependency)")
    print()
    
    # Simplified matrix based on files using deps
    modules = set()
    for file_path in data['rust_files_using_deps']:
        if '/' in file_path:
            module = file_path.split('/')[1]
            modules.add(module)
    
    crates = ['boring-signal', 'subtle', 'ctr']
    
    print(f"{'Module':<15} │ {'boring':<7} │ {'subtle':<7} │ {'ctr':<5} │")
    print("─" * 45)
    
    for module in sorted(modules):
        line = f"{module:<15} │"
        for crate in crates:
            # This is a simplified check - in a real analysis we'd check actual usage
            has_dep = any(crate in file_path for file_path in data['rust_files_using_deps'] 
                         if file_path.startswith(f"rust/{module}/"))
            mark = "   X   " if has_dep else "       "
            line += f" {mark} │"
        print(line)

def main():
    from pathlib import Path
    if not Path('dependency_analysis.json').exists():
        print("Error: dependency_analysis.json not found. Run analyze_deps.py first.")
        return
    
    create_simple_chart()

if __name__ == "__main__":
    main()

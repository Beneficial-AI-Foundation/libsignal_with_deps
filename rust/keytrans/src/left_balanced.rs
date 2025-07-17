//
// Copyright 2024 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

//! Contains functions that are shared between the two left-balanced search tree
//! implementations (the Log Tree and the Implicit Binary Search Tree).
//!
//! This module works on a flat array representation, where the nodes of the
//! tree are numbered from left to right. Leaf nodes are stored in even-numbered
//! indices, while intermediate nodes are stored in odd-numbered indices:
//!
//! ```text
//!                              X
//!                              |
//!                    .---------+---------.
//!                   /                     \
//!                  X                       X
//!                  |                       |
//!              .---+---.               .---+---.
//!             /         \             /         \
//!            X           X           X           X
//!           / \         / \         / \         /
//!          /   \       /   \       /   \       /
//!         X     X     X     X     X     X     X
//!
//! Index:  0  1  2  3  4  5  6  7  8  9 10 11 12 13
//! ```
//!
//! The bit twiddling functions in this file are all taken from RFC 9420,
//! although you will not find more insight on how/why they work there.

pub fn log2(n: u64) -> u32 {
    n.checked_ilog2().unwrap_or(0)
}

/// Returns true if x is the position of a leaf node.
pub fn is_leaf(x: u64) -> bool {
    (x & 1) == 0
}

/// Returns the level of a node in the tree. Leaves are level 0, their parents
/// are level 1, and so on.
pub fn level(x: u64) -> usize {
    x.trailing_ones() as usize
}

pub fn left_step(x: u64) -> u64 {
    match level(x) {
        0 => panic!("leaf node has no children"),
        k => x ^ (1 << (k - 1)),
    }
}

pub fn right_step(x: u64) -> u64 {
    match level(x) {
        0 => panic!("leaf node has no children"),
        k => x ^ (3 << (k - 1)),
    }
}

pub fn parent_step(x: u64) -> u64 {
    let k = level(x);
    let b = (x >> (k + 1)) & 1;
    (x | (1 << k)) ^ (b << (k + 1))
}


#[cfg(kani)]
#[kani::proof]
    fn verify_tree_navigation_properties() {
        let x: u64 = kani::any();

        // Bound the input to reasonable tree sizes (e.g., 16 bits)
        kani::assume(x < (1 << 16));

        // Property 1: Level consistency
        let lvl = level(x);

        // Property 2: If not a leaf, left and right steps should return valid children
        if !is_leaf(x) && lvl > 0 {
            let left_child = left_step(x);
            let right_child = right_step(x);

            // Children should be at one level lower
            assert_eq!(level(left_child), lvl - 1);
            assert_eq!(level(right_child), lvl - 1);

            // Children should have the current node as parent
            assert_eq!(parent_step(left_child), x);
            assert_eq!(parent_step(right_child), x);

            // Left child should be smaller than right child
            assert!(left_child < right_child);
        }

        // Property 3: Parent-child relationship consistency
        if lvl < 63 { // Avoid overflow
            let parent = parent_step(x);
            let parent_level = level(parent);

            // Parent should be at one level higher
            assert_eq!(parent_level, lvl + 1);

            // Current node should be either left or right child of parent
            if !is_leaf(parent) {
                let left = left_step(parent);
                let right = right_step(parent);
                assert!(x == left || x == right);
            }
        }
    }

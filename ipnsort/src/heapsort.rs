use core::ptr;

/// Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.
///
/// Never inline this, it sits the main hot-loop in `recurse` and is meant as unlikely algorithmic
/// fallback.
///
/// SAFETY: The caller has to guarantee that `v.len()` >= 2.
#[inline(never)]
pub(crate) unsafe fn heapsort<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if v.len() < 2 {
        // This helps prove things to the compiler. That we checked earlier.
        // SAFETY: This function is only called if len >= 2.
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }

    let len = v.len();

    // Build the heap in linear time.
    for i in (0..len / 2).rev() {
        sift_down(v, i, is_less);
    }

    // Pop maximal elements from the heap.
    for i in (1..len).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0, is_less);
    }
}

// This binary heap respects the invariant `parent >= child`.
//
// SAFETY: The caller has to guarantee that node < `v.len()`.
#[inline(never)]
unsafe fn sift_down<T, F>(v: &mut [T], mut node: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if node >= v.len() {
        // This helps prove things to the compiler. That we checked earlier.
        // SAFETY: This function is only called if node < `v.len()`.
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }

    let len = v.len();

    let v_base = v.as_mut_ptr();

    loop {
        // Children of `node`.
        let mut child = 2 * node + 1;
        if child >= len {
            break;
        }

        // SAFETY: The invariants and checks guarantee that both node and child are in-bounds.
        unsafe {
            // Choose the greater child.
            if child + 1 < len {
                // We need a branch to be sure not to out-of-bounds index,
                // but it's highly predictable.  The comparison, however,
                // is better done branchless, especially for primitives.
                child += is_less(&*v_base.add(child), &*v_base.add(child + 1)) as usize;
            }

            // Stop if the invariant holds at `node`.
            if !is_less(&*v_base.add(node), &*v_base.add(child)) {
                break;
            }

            // Swap `node` with the greater child, move one step down, and continue sifting.
            // Same as v.swap_unchecked(node, child); which is unstable.
            ptr::swap(v_base.add(node), v_base.add(child))
        }

        node = child;
    }
}

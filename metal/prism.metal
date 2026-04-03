// prism.metal — the root crystal.
//
// Hand-wired. The seed. The fixed point that bootstraps everything else.
// Five operations encoded as GPU compute kernels.
// This file exists because the compiler that would produce it
// doesn't exist yet.
//
// The parser loads the eigenvalues from this kernel's output.
// The eigenvalues ARE the parser's vocabulary.

#include <metal_stdlib>
using namespace metal;

// ---------------------------------------------------------------------------
// fold — decompose structure into eigenvalues
// Laplacian: L = D - A. Eigenvalues of L.
// Input: adjacency matrix (n×n, symmetric)
// Output: diagonal of Laplacian (eigenvalue estimates via Gershgorin)
// ---------------------------------------------------------------------------

kernel void fold(
    device const float* adjacency [[buffer(0)]],
    device float* eigenvalues [[buffer(1)]],
    constant uint& n [[buffer(2)]],
    uint id [[thread_position_in_grid]]
) {
    if (id >= n) return;

    // Compute degree (row sum) and diagonal of Laplacian
    float degree = 0.0;
    for (uint j = 0; j < n; j++) {
        degree += adjacency[id * n + j];
    }
    // Gershgorin estimate: eigenvalue ≈ degree
    // (exact for regular graphs, good estimate for most)
    eigenvalues[id] = degree;
}

// ---------------------------------------------------------------------------
// prism — project eigenvalues through precision cut
// Keep eigenvalues above threshold. Zero the rest.
// Output: count of surviving eigenvalues in output[0]
// ---------------------------------------------------------------------------

kernel void prism(
    device const float* eigenvalues [[buffer(0)]],
    device float* projection [[buffer(1)]],
    device atomic_uint* count [[buffer(2)]],
    constant float& precision [[buffer(3)]],
    constant uint& n [[buffer(4)]],
    uint id [[thread_position_in_grid]]
) {
    if (id >= n) return;

    if (eigenvalues[id] > precision) {
        projection[id] = eigenvalues[id];
        atomic_fetch_add_explicit(count, 1, memory_order_relaxed);
    } else {
        projection[id] = 0.0;
    }
}

// ---------------------------------------------------------------------------
// traversal — walk nodes, compute spectral distance between pairs
// For each node i, compute distance to all other nodes j
// Distance = |λ_i - λ_j| (eigenvalue distance)
// Output: distance matrix (n×n)
// ---------------------------------------------------------------------------

kernel void traversal(
    device const float* eigenvalues [[buffer(0)]],
    device float* distances [[buffer(1)]],
    constant uint& n [[buffer(2)]],
    uint2 id [[thread_position_in_grid]]
) {
    uint i = id.x;
    uint j = id.y;
    if (i >= n || j >= n) return;

    distances[i * n + j] = abs(eigenvalues[i] - eigenvalues[j]);
}

// ---------------------------------------------------------------------------
// lens — focus, transform, put back
// Apply element-wise transform: output[i] = input[i] * scale + bias
// The simplest lens: affine transformation
// ---------------------------------------------------------------------------

kernel void lens(
    device const float* input [[buffer(0)]],
    device float* output [[buffer(1)]],
    constant float& scale [[buffer(2)]],
    constant float& bias [[buffer(3)]],
    constant uint& n [[buffer(4)]],
    uint id [[thread_position_in_grid]]
) {
    if (id >= n) return;
    output[id] = input[id] * scale + bias;
}

// ---------------------------------------------------------------------------
// iso — convergence check
// Compare two eigenvalue vectors. If max difference < epsilon, settled.
// Output: max_diff in result[0], settled flag in result[1]
// ---------------------------------------------------------------------------

kernel void iso(
    device const float* current [[buffer(0)]],
    device const float* previous [[buffer(1)]],
    device float* result [[buffer(2)]],
    constant float& epsilon [[buffer(3)]],
    constant uint& n [[buffer(4)]],
    uint id [[thread_position_in_grid]]
) {
    if (id >= n) return;

    float diff = abs(current[id] - previous[id]);

    // Thread 0 aggregates (simple — for small n)
    // For large n, use parallel reduction
    if (id == 0) {
        float max_diff = 0.0;
        for (uint i = 0; i < n; i++) {
            float d = abs(current[i] - previous[i]);
            if (d > max_diff) max_diff = d;
        }
        result[0] = max_diff;
        result[1] = (max_diff < epsilon) ? 1.0 : 0.0;
    }
}

// ---------------------------------------------------------------------------
// matmul — the classifier's forward pass
// C = A × B, where A is (m×k), B is (k×n)
// Used for the 2,892-parameter tension classifier
// ---------------------------------------------------------------------------

kernel void matmul(
    device const float* A [[buffer(0)]],
    device const float* B [[buffer(1)]],
    device float* C [[buffer(2)]],
    constant uint& M [[buffer(3)]],
    constant uint& K [[buffer(4)]],
    constant uint& N [[buffer(5)]],
    uint2 id [[thread_position_in_grid]]
) {
    uint row = id.x;
    uint col = id.y;
    if (row >= M || col >= N) return;

    float sum = 0.0;
    for (uint i = 0; i < K; i++) {
        sum += A[row * K + i] * B[i * N + col];
    }
    C[row * N + col] = sum;
}

// ---------------------------------------------------------------------------
// sigmoid — element-wise activation for hidden layer
// ---------------------------------------------------------------------------

kernel void sigmoid(
    device float* data [[buffer(0)]],
    constant uint& n [[buffer(1)]],
    uint id [[thread_position_in_grid]]
) {
    if (id >= n) return;
    data[id] = 1.0 / (1.0 + exp(-data[id]));
}

// ---------------------------------------------------------------------------
// softmax — classifier output
// Two-pass: find max, then exp and normalize
// ---------------------------------------------------------------------------

kernel void softmax(
    device float* data [[buffer(0)]],
    constant uint& n [[buffer(1)]],
    uint id [[thread_position_in_grid]]
) {
    // Single-threaded for small n (12 optic categories)
    if (id != 0) return;

    float max_val = data[0];
    for (uint i = 1; i < n; i++) {
        if (data[i] > max_val) max_val = data[i];
    }

    float sum = 0.0;
    for (uint i = 0; i < n; i++) {
        data[i] = exp(data[i] - max_val);
        sum += data[i];
    }

    for (uint i = 0; i < n; i++) {
        data[i] /= sum;
    }
}

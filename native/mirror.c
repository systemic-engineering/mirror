/*
 * mirror.c — the native binary. One C file. No deps beyond libc.
 *
 * Implements: SHA-256, CoincidenceHash<3>, tokenizer, content-addressing,
 * git crystal cache, CLI dispatch.
 *
 * Same OIDs as the Rust binary. ~50KB stripped.
 *
 * Build:
 *   clang -O2 -ffp-contract=off -o mirror native/mirror.c -lm
 *
 * The -ffp-contract=off flag is REQUIRED for OID compatibility with the Rust
 * binary. Without it, the compiler may fuse multiply-add operations (FMA),
 * producing different floating-point intermediate values in the CoincidenceHash
 * computation. Even a 1 ULP difference cascades through SHA-256 compression
 * to produce completely different content addresses.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>

/* =========================================================================
 * SHA-256
 * ========================================================================= */

static const unsigned int sha256_k[64] = {
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
};

#define ROTR(x,n) (((x)>>(n))|((x)<<(32-(n))))
#define CH(x,y,z) (((x)&(y))^((~(x))&(z)))
#define MAJ(x,y,z) (((x)&(y))^((x)&(z))^((y)&(z)))
#define EP0(x) (ROTR(x,2)^ROTR(x,13)^ROTR(x,22))
#define EP1(x) (ROTR(x,6)^ROTR(x,11)^ROTR(x,25))
#define SIG0(x) (ROTR(x,7)^ROTR(x,18)^((x)>>3))
#define SIG1(x) (ROTR(x,17)^ROTR(x,19)^((x)>>10))

typedef struct {
    unsigned int state[8];
    unsigned char data[64];
    unsigned int datalen;
    unsigned long long bitlen;
} sha256_ctx;

static void sha256_transform(sha256_ctx *ctx, const unsigned char data[64]) {
    unsigned int a, b, c, d, e, f, g, h, t1, t2, w[64];
    int i;
    for (i = 0; i < 16; i++) {
        w[i] = ((unsigned int)data[i*4] << 24) | ((unsigned int)data[i*4+1] << 16) |
               ((unsigned int)data[i*4+2] << 8) | (unsigned int)data[i*4+3];
    }
    for (i = 16; i < 64; i++)
        w[i] = SIG1(w[i-2]) + w[i-7] + SIG0(w[i-15]) + w[i-16];
    a = ctx->state[0]; b = ctx->state[1]; c = ctx->state[2]; d = ctx->state[3];
    e = ctx->state[4]; f = ctx->state[5]; g = ctx->state[6]; h = ctx->state[7];
    for (i = 0; i < 64; i++) {
        t1 = h + EP1(e) + CH(e,f,g) + sha256_k[i] + w[i];
        t2 = EP0(a) + MAJ(a,b,c);
        h = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;
    }
    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c; ctx->state[3] += d;
    ctx->state[4] += e; ctx->state[5] += f; ctx->state[6] += g; ctx->state[7] += h;
}

static void sha256_init(sha256_ctx *ctx) {
    ctx->datalen = 0;
    ctx->bitlen = 0;
    ctx->state[0] = 0x6a09e667; ctx->state[1] = 0xbb67ae85;
    ctx->state[2] = 0x3c6ef372; ctx->state[3] = 0xa54ff53a;
    ctx->state[4] = 0x510e527f; ctx->state[5] = 0x9b05688c;
    ctx->state[6] = 0x1f83d9ab; ctx->state[7] = 0x5be0cd19;
}

static void sha256_update(sha256_ctx *ctx, const unsigned char *data, size_t len) {
    size_t i;
    for (i = 0; i < len; i++) {
        ctx->data[ctx->datalen] = data[i];
        ctx->datalen++;
        if (ctx->datalen == 64) {
            sha256_transform(ctx, ctx->data);
            ctx->bitlen += 512;
            ctx->datalen = 0;
        }
    }
}

static void sha256_final(sha256_ctx *ctx, unsigned char hash[32]) {
    unsigned int i = ctx->datalen;
    if (ctx->datalen < 56) {
        ctx->data[i++] = 0x80;
        while (i < 56) ctx->data[i++] = 0;
    } else {
        ctx->data[i++] = 0x80;
        while (i < 64) ctx->data[i++] = 0;
        sha256_transform(ctx, ctx->data);
        memset(ctx->data, 0, 56);
    }
    ctx->bitlen += ctx->datalen * 8;
    ctx->data[63] = (unsigned char)(ctx->bitlen);
    ctx->data[62] = (unsigned char)(ctx->bitlen >> 8);
    ctx->data[61] = (unsigned char)(ctx->bitlen >> 16);
    ctx->data[60] = (unsigned char)(ctx->bitlen >> 24);
    ctx->data[59] = (unsigned char)(ctx->bitlen >> 32);
    ctx->data[58] = (unsigned char)(ctx->bitlen >> 40);
    ctx->data[57] = (unsigned char)(ctx->bitlen >> 48);
    ctx->data[56] = (unsigned char)(ctx->bitlen >> 56);
    sha256_transform(ctx, ctx->data);
    for (i = 0; i < 8; i++) {
        hash[i*4]   = (ctx->state[i] >> 24) & 0xff;
        hash[i*4+1] = (ctx->state[i] >> 16) & 0xff;
        hash[i*4+2] = (ctx->state[i] >> 8)  & 0xff;
        hash[i*4+3] = (ctx->state[i])       & 0xff;
    }
}

/* SHA-256 convenience: hash data and write hex string (64 chars + null) */
static void sha256_hex(const unsigned char *data, size_t len, char out[65]) {
    sha256_ctx ctx;
    unsigned char hash[32];
    sha256_init(&ctx);
    sha256_update(&ctx, data, len);
    sha256_final(&ctx, hash);
    for (int i = 0; i < 32; i++)
        sprintf(out + i*2, "%02x", hash[i]);
    out[64] = '\0';
}

/* =========================================================================
 * CoincidenceHash<3> — eigenvalue-based content addressing
 *
 * Ports the Rust prism-core coincidence module exactly:
 * 1. encode_into_basis: bytes -> 16-dimensional coefficients via SHA-256 seeds
 * 2. Projection::from_seed: deterministic rank-1 projections from SHA-256
 * 3. detect: apply 3 projections, build eigenvalue record
 * 4. review: SHA-256 compress the eigenvalue record -> 64 hex chars
 * ========================================================================= */

#define DIM 16
#define NUM_PROJECTIONS 3
#define EPSILON 2.2204460492503131e-16   /* f64::EPSILON */
#define FRAGILE_THRESHOLD 1e-6

/* --- Projection matrix: rank-1, stored as dense DIMxDIM --- */

/* BTreeMap lexicographic order of "d0".."d15":
 * d0, d1, d10, d11, d12, d13, d14, d15, d2, d3, d4, d5, d6, d7, d8, d9
 * This maps to numeric indices: */
static const int LEX_ORDER[DIM] = {0, 1, 10, 11, 12, 13, 14, 15, 2, 3, 4, 5, 6, 7, 8, 9};

typedef struct {
    double entries[DIM][DIM];
    /* Which entries are "present" (|v[i]*v[j]| > EPSILON in rank1) */
    int present[DIM][DIM];
} projection_t;

/* Derive a rank-1 projection from a seed string.
 * Matches Rust's Projection::from_seed exactly. */
static void projection_from_seed(const char *seed, projection_t *proj) {
    size_t seed_len = strlen(seed);
    double components[DIM];

    /* Generate components: for each dimension j, hash seed + ":" + j_le_bytes */
    for (int j = 0; j < DIM; j++) {
        sha256_ctx hasher;
        unsigned char hash[32];
        sha256_init(&hasher);
        sha256_update(&hasher, (const unsigned char *)seed, seed_len);
        sha256_update(&hasher, (const unsigned char *)":", 1);
        /* j as 8 little-endian bytes (usize on 64-bit) */
        unsigned char j_bytes[8];
        unsigned long long jj = (unsigned long long)j;
        for (int k = 0; k < 8; k++) j_bytes[k] = (unsigned char)((jj >> (k*8)) & 0xff);
        sha256_update(&hasher, j_bytes, 8);
        sha256_final(&hasher, hash);
        /* First 8 bytes as u64 little-endian */
        unsigned long long raw = 0;
        for (int k = 0; k < 8; k++) raw |= ((unsigned long long)hash[k]) << (k*8);
        double val = ((double)raw / (double)18446744073709551615ULL) * 2.0 - 1.0;
        components[j] = val;
    }

    /* Normalize: compute norm in BTreeMap key (lex) order.
     * StateVector::norm() iterates self.entries.values() which is BTreeMap order
     * of string keys "d0".."d15" = lex: d0,d1,d10,...,d15,d2,...,d9 */
    double norm = 0.0;
    for (int li = 0; li < DIM; li++) {
        int j = LEX_ORDER[li];
        norm += components[j] * components[j];
    }
    norm = sqrt(norm);
    if (norm < EPSILON) norm = 1.0;
    for (int j = 0; j < DIM; j++) components[j] /= norm;

    /* Build rank-1 matrix: P[i][j] = v[i] * v[j]
     * Mark as present only where |val| > EPSILON (matching Rust rank1) */
    for (int i = 0; i < DIM; i++) {
        if (fabs(components[i]) < EPSILON) {
            for (int j = 0; j < DIM; j++) {
                proj->entries[i][j] = 0.0;
                proj->present[i][j] = 0;
            }
            continue;
        }
        for (int j = 0; j < DIM; j++) {
            double val = components[i] * components[j];
            if (fabs(val) > EPSILON) {
                proj->entries[i][j] = val;
                proj->present[i][j] = 1;
            } else {
                proj->entries[i][j] = 0.0;
                proj->present[i][j] = 0;
            }
        }
    }
}

/* Apply projection to a vector. Result stored in out[DIM].
 * Matches Rust's Projection::apply exactly:
 * - Iterates entries in BTreeMap<(String,String), f64> order
 * - BTreeMap key order for (row,col) tuples is lex order on the tuple
 *   (first by row lex, then by col lex within same row)
 * - Filters input coefficients by EPSILON
 * - Filters result by EPSILON */
static void projection_apply(const projection_t *proj, const double *v, double *out) {
    /* Initialize result to zero */
    for (int i = 0; i < DIM; i++) out[i] = 0.0;

    /* Iterate in BTreeMap<(String,String), f64> lex order.
     * Row keys in lex order: d0,d1,d10,...,d15,d2,...,d9
     * Col keys in lex order within each row: same pattern.
     * This matches how Rust's BTreeMap iterates the entries. */
    for (int ri = 0; ri < DIM; ri++) {
        int row = LEX_ORDER[ri];
        for (int ci = 0; ci < DIM; ci++) {
            int col = LEX_ORDER[ci];
            if (!proj->present[row][col]) continue;
            double coeff = v[col];
            if (fabs(coeff) > EPSILON) {
                out[row] += proj->entries[row][col] * coeff;
            }
        }
    }

    /* Filter result: set near-zero entries to 0.0 (matching Rust result.retain) */
    for (int i = 0; i < DIM; i++) {
        if (fabs(out[i]) <= EPSILON) out[i] = 0.0;
    }
}

/* Check if vector is zero (all entries below EPSILON). */
static int vec_is_zero(const double *v) {
    for (int i = 0; i < DIM; i++)
        if (fabs(v[i]) > EPSILON) return 0;
    return 1;
}

/* --- Encode bytes into basis (matches Rust encode_into_basis exactly) --- */
static void encode_into_basis(const unsigned char *data, size_t len, double *coefficients) {
    memset(coefficients, 0, DIM * sizeof(double));
    if (len == 0) return;

    for (size_t i = 0; i < len; i++) {
        /* seed_hasher: SHA-256 of "encode:" + i_le_bytes + byte */
        sha256_ctx seed_hasher;
        unsigned char seed[32];
        sha256_init(&seed_hasher);
        sha256_update(&seed_hasher, (const unsigned char *)"encode:", 7);
        unsigned char i_bytes[8];
        unsigned long long ii = (unsigned long long)i;
        for (int k = 0; k < 8; k++) i_bytes[k] = (unsigned char)((ii >> (k*8)) & 0xff);
        sha256_update(&seed_hasher, i_bytes, 8);
        sha256_update(&seed_hasher, &data[i], 1);
        sha256_final(&seed_hasher, seed);

        for (int j = 0; j < DIM; j++) {
            sha256_ctx dim_hasher;
            unsigned char dim_hash[32];
            sha256_init(&dim_hasher);
            sha256_update(&dim_hasher, seed, 32);
            sha256_update(&dim_hasher, (const unsigned char *)":", 1);
            unsigned char j_bytes[8];
            unsigned long long jj = (unsigned long long)j;
            for (int k = 0; k < 8; k++) j_bytes[k] = (unsigned char)((jj >> (k*8)) & 0xff);
            sha256_update(&dim_hasher, j_bytes, 8);
            sha256_final(&dim_hasher, dim_hash);
            unsigned long long raw = 0;
            for (int k = 0; k < 8; k++) raw |= ((unsigned long long)dim_hash[k]) << (k*8);
            double val = ((double)raw / (double)18446744073709551615ULL) * 2.0 - 1.0;
            coefficients[j] += val;
        }
    }

    /* Filter: entries with |coeff| <= EPSILON are treated as absent (0.0)
     * Matches Rust's encode_into_basis filter + StateVector::from_entries */
    for (int j = 0; j < DIM; j++) {
        if (fabs(coefficients[j]) <= EPSILON) coefficients[j] = 0.0;
    }
}

/* --- Canonical projections (lazy-init, matches Rust's LazyLock) --- */
static projection_t canonical_projections[NUM_PROJECTIONS];
static int projections_initialized = 0;

static void init_projections(void) {
    if (projections_initialized) return;
    for (int i = 0; i < NUM_PROJECTIONS; i++) {
        char seed[64];
        snprintf(seed, sizeof(seed), "coincidence:projection:%d:%d", i, NUM_PROJECTIONS);
        projection_from_seed(seed, &canonical_projections[i]);
    }
    projections_initialized = 1;
}

/* --- Hex encode helper --- */
static void hex_encode(const unsigned char *data, size_t len, char *out) {
    for (size_t i = 0; i < len; i++)
        sprintf(out + i*2, "%02x", data[i]);
    out[len*2] = '\0';
}

/* --- canonical_hash: CoincidenceHash<3> then SHA-256 compression ---
 * Matches Rust's canonical_hash() exactly. */
static void canonical_hash(const unsigned char *data, size_t len, char out[65]) {
    init_projections();

    /* Encode into basis */
    double coefficients[DIM];
    encode_into_basis(data, len, coefficients);

    if (vec_is_zero(coefficients)) {
        /* Fallback: SHA-256 with domain separation "prism-core:dark:" + input */
        sha256_ctx hasher;
        unsigned char hash[32];
        sha256_init(&hasher);
        sha256_update(&hasher, (const unsigned char *)"prism-core:dark:", 16);
        sha256_update(&hasher, data, len);
        sha256_final(&hasher, hash);
        for (int i = 0; i < 32; i++) sprintf(out + i*2, "%02x", hash[i]);
        out[64] = '\0';
        return;
    }

    /* Build eigenvalue record: "coincidence:" + N_le_bytes + projection results */
    /* First, compute all projection results and check for zero/disagree */
    double focus_results[NUM_PROJECTIONS][DIM];
    int any_zero = 0;
    for (int p = 0; p < NUM_PROJECTIONS; p++) {
        projection_apply(&canonical_projections[p], coefficients, focus_results[p]);
        if (vec_is_zero(focus_results[p])) {
            any_zero = 1;
            break;
        }
    }

    if (any_zero) {
        /* Disagree fallback */
        sha256_ctx hasher;
        unsigned char hash[32];
        sha256_init(&hasher);
        sha256_update(&hasher, (const unsigned char *)"prism-core:dark:", 16);
        sha256_update(&hasher, data, len);
        sha256_final(&hasher, hash);
        for (int i = 0; i < 32; i++) sprintf(out + i*2, "%02x", hash[i]);
        out[64] = '\0';
        return;
    }

    /* Build eigenvalue bytes: "coincidence:" + N as u64 LE + dense f64 vectors */
    /* eigenvalue_bytes capacity: 12 + 8 + NUM_PROJECTIONS * DIM * 8 */
    size_t ev_len = 12 + 8 + NUM_PROJECTIONS * DIM * 8;
    unsigned char *eigenvalue_bytes = (unsigned char *)malloc(ev_len);
    size_t pos = 0;

    memcpy(eigenvalue_bytes, "coincidence:", 12);
    pos = 12;

    /* N as u64 little-endian */
    unsigned long long n = NUM_PROJECTIONS;
    for (int k = 0; k < 8; k++) eigenvalue_bytes[pos++] = (unsigned char)((n >> (k*8)) & 0xff);

    /* Dense f64 vectors for each projection result */
    for (int p = 0; p < NUM_PROJECTIONS; p++) {
        for (int j = 0; j < DIM; j++) {
            /* f64 to little-endian bytes */
            unsigned long long bits;
            memcpy(&bits, &focus_results[p][j], 8);
            for (int k = 0; k < 8; k++)
                eigenvalue_bytes[pos++] = (unsigned char)((bits >> (k*8)) & 0xff);
        }
    }

    /* Hex encode the eigenvalue bytes */
    size_t hex_len = pos * 2 + 1;
    char *eigenvalue_hex = (char *)malloc(hex_len);
    hex_encode(eigenvalue_bytes, pos, eigenvalue_hex);

    /* Decode hex back to bytes (for SHA-256 input) -- but actually, the Rust
     * code hex-encodes eigenvalue_bytes, then hex-decodes it back. The round
     * trip means the SHA-256 input is just eigenvalue_bytes. We skip the
     * roundtrip and hash eigenvalue_bytes directly. */

    /* SHA-256 compress: "prism-core:coincidence:" + eigenvalue_bytes */
    sha256_ctx hasher;
    unsigned char hash[32];
    sha256_init(&hasher);
    sha256_update(&hasher, (const unsigned char *)"prism-core:coincidence:", 23);
    sha256_update(&hasher, eigenvalue_bytes, pos);
    sha256_final(&hasher, hash);

    for (int i = 0; i < 32; i++) sprintf(out + i*2, "%02x", hash[i]);
    out[64] = '\0';

    free(eigenvalue_bytes);
    free(eigenvalue_hex);
}

/* =========================================================================
 * Tokenizer — grammar loading + source scanning
 * ========================================================================= */

/* AST node kinds (five operations + In + Out) */
typedef enum {
    AST_FOCUS,
    AST_PROJECT,
    AST_SPLIT,
    AST_ZOOM,
    AST_REFRACT,
    AST_IN,
    AST_OUT
} ast_kind_t;

/* Forward declaration */
typedef struct ast_node ast_node_t;

struct ast_node {
    ast_kind_t kind;
    char name[256];          /* node name */
    ast_node_t **children;   /* dynamic array of children */
    int num_children;
    int cap_children;
};

static ast_node_t *ast_new(ast_kind_t kind, const char *name) {
    ast_node_t *node = (ast_node_t *)calloc(1, sizeof(ast_node_t));
    node->kind = kind;
    strncpy(node->name, name, 255);
    node->name[255] = '\0';
    node->children = NULL;
    node->num_children = 0;
    node->cap_children = 0;
    return node;
}

static void ast_add_child(ast_node_t *parent, ast_node_t *child) {
    if (parent->num_children >= parent->cap_children) {
        parent->cap_children = parent->cap_children ? parent->cap_children * 2 : 4;
        parent->children = (ast_node_t **)realloc(parent->children,
            parent->cap_children * sizeof(ast_node_t *));
    }
    parent->children[parent->num_children++] = child;
}

static void ast_free(ast_node_t *node) {
    if (!node) return;
    for (int i = 0; i < node->num_children; i++)
        ast_free(node->children[i]);
    free(node->children);
    free(node);
}

/* Grammar: keyword -> operation mappings */
typedef struct {
    char keyword[64];
    ast_kind_t kind;
} grammar_mapping_t;

typedef struct {
    grammar_mapping_t *mappings;
    int num_mappings;
    int cap_mappings;
} grammar_t;

static void grammar_init(grammar_t *g) {
    g->mappings = NULL;
    g->num_mappings = 0;
    g->cap_mappings = 0;
}

static void grammar_add(grammar_t *g, const char *keyword, ast_kind_t kind) {
    if (g->num_mappings >= g->cap_mappings) {
        g->cap_mappings = g->cap_mappings ? g->cap_mappings * 2 : 8;
        g->mappings = (grammar_mapping_t *)realloc(g->mappings,
            g->cap_mappings * sizeof(grammar_mapping_t));
    }
    strncpy(g->mappings[g->num_mappings].keyword, keyword, 63);
    g->mappings[g->num_mappings].keyword[63] = '\0';
    g->mappings[g->num_mappings].kind = kind;
    g->num_mappings++;
}

static void grammar_free(grammar_t *g) {
    free(g->mappings);
    g->mappings = NULL;
    g->num_mappings = 0;
}

static int grammar_lookup(const grammar_t *g, const char *word, ast_kind_t *kind) {
    for (int i = 0; i < g->num_mappings; i++) {
        if (strcmp(g->mappings[i].keyword, word) == 0) {
            *kind = g->mappings[i].kind;
            return 1;
        }
    }
    return 0;
}

/* Read entire file into malloc'd buffer. Caller must free. */
static char *read_file(const char *path, size_t *len_out) {
    FILE *f = fopen(path, "r");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buf = (char *)malloc(sz + 1);
    size_t n = fread(buf, 1, sz, f);
    buf[n] = '\0';
    if (len_out) *len_out = n;
    fclose(f);
    return buf;
}

/* Parse a grammar from source text.
 * Matches Rust parse_grammar exactly. */
static void parse_grammar(const char *source, grammar_t *g) {
    grammar_init(g);
    int in_grammar_block = 0;
    int brace_depth = 0;

    const char *line = source;
    while (*line) {
        /* Find end of line */
        const char *eol = line;
        while (*eol && *eol != '\n') eol++;

        /* Copy trimmed line */
        const char *start = line;
        while (start < eol && (*start == ' ' || *start == '\t')) start++;
        const char *end = eol;
        while (end > start && (*(end-1) == ' ' || *(end-1) == '\t' || *(end-1) == '\r')) end--;

        size_t trimmed_len = end - start;
        char trimmed[1024];
        if (trimmed_len >= sizeof(trimmed)) trimmed_len = sizeof(trimmed) - 1;
        memcpy(trimmed, start, trimmed_len);
        trimmed[trimmed_len] = '\0';

        /* Skip comments */
        if (trimmed[0] == '-' && trimmed[1] == '-') goto next_line;
        if (trimmed[0] == '#') goto next_line;

        /* Track grammar block entry */
        if (strncmp(trimmed, "grammar ", 8) == 0 && strchr(trimmed, '{')) {
            in_grammar_block = 1;
            brace_depth = 1;
            goto next_line;
        }

        if (!in_grammar_block) goto next_line;

        /* Track braces */
        for (size_t k = 0; k < trimmed_len; k++) {
            if (trimmed[k] == '{') brace_depth++;
            else if (trimmed[k] == '}') {
                brace_depth--;
                if (brace_depth == 0) in_grammar_block = 0;
            }
        }

        if (!in_grammar_block && brace_depth <= 0) goto next_line;

        /* Skip lines with parens, arrows, braces */
        if (strchr(trimmed, '(') || strchr(trimmed, '>') ||
            strchr(trimmed, '{') || strchr(trimmed, '}'))
            goto next_line;

        /* Strip inline comments */
        char content[1024];
        strncpy(content, trimmed, sizeof(content)-1);
        content[sizeof(content)-1] = '\0';
        char *hash_pos = strchr(content, '#');
        if (hash_pos) *hash_pos = '\0';
        char *dash_pos = strstr(content, " --");
        if (dash_pos) *dash_pos = '\0';
        /* Trim trailing whitespace from content */
        size_t clen = strlen(content);
        while (clen > 0 && (content[clen-1] == ' ' || content[clen-1] == '\t')) clen--;
        content[clen] = '\0';

        /* Extract exactly two words */
        char w1[64] = {0}, w2[64] = {0};
        int nw = sscanf(content, "%63s %63s", w1, w2);
        if (nw == 2) {
            /* Check for extra words */
            const char *p = content;
            int words = 0;
            while (*p) {
                while (*p == ' ' || *p == '\t') p++;
                if (*p && *p != ' ' && *p != '\t') {
                    words++;
                    while (*p && *p != ' ' && *p != '\t') p++;
                }
            }
            if (words == 2) {
                ast_kind_t kind;
                int found = 0;
                if (strcmp(w1, "focus") == 0) { kind = AST_FOCUS; found = 1; }
                else if (strcmp(w1, "split") == 0) { kind = AST_SPLIT; found = 1; }
                else if (strcmp(w1, "zoom") == 0) { kind = AST_ZOOM; found = 1; }
                else if (strcmp(w1, "project") == 0) { kind = AST_PROJECT; found = 1; }
                else if (strcmp(w1, "refract") == 0) { kind = AST_REFRACT; found = 1; }
                if (found) grammar_add(g, w2, kind);
            }
        }

next_line:
        line = *eol ? eol + 1 : eol;
    }
}

/* Load grammar from a file. Returns 0 on success. */
static int load_grammar(const char *path, grammar_t *g) {
    char *source = read_file(path, NULL);
    if (!source) {
        fprintf(stderr, "cannot read grammar %s\n", path);
        return -1;
    }
    parse_grammar(source, g);
    free(source);
    return 0;
}

/* Return the grammar file path for a given source file. */
static const char *grammar_for_file(const char *path) {
    size_t len = strlen(path);
    if (len >= 3 && strcmp(path + len - 3, ".rs") == 0)
        return "boot/std/code/rust.mirror";
    if ((len >= 7 && strcmp(path + len - 7, ".mirror") == 0) ||
        (len >= 5 && strcmp(path + len - 5, ".spec") == 0) ||
        (len >= 8 && strcmp(path + len - 8, ".shatter") == 0))
        return "boot/std/mirror/grammar.mirror";
    return "boot/std/code/rust.mirror";
}

/* --- Skip list: Rust modifiers that are NOT grammar keywords --- */
static int is_skip_word(const char *word) {
    static const char *skip[] = {
        "async","unsafe","const","extern","crate","super","self","where",
        "mut","ref","static","let","if","else","for","while","loop","match",
        "return","break","continue","as","in","type","dyn","move",NULL
    };
    for (int i = 0; skip[i]; i++)
        if (strcmp(word, skip[i]) == 0) return 1;
    return 0;
}

/* --- Tokenizer: scan source using grammar keywords (matches Rust scan_items) --- */
static void scan_items(const char *source, size_t len, const grammar_t *grammar,
                       ast_node_t *parent);

static void scan_items(const char *source, size_t slen, const grammar_t *grammar,
                       ast_node_t *parent) {
    const unsigned char *bytes = (const unsigned char *)source;
    size_t len = slen;
    size_t pos = 0;

    while (pos < len) {
        /* Skip whitespace */
        while (pos < len && (bytes[pos] == ' ' || bytes[pos] == '\t' ||
               bytes[pos] == '\n' || bytes[pos] == '\r'))
            pos++;
        if (pos >= len) break;

        /* Line comments: // or -- */
        if (pos + 1 < len && bytes[pos] == '/' && bytes[pos+1] == '/') {
            while (pos < len && bytes[pos] != '\n') pos++;
            continue;
        }
        if (pos + 1 < len && bytes[pos] == '-' && bytes[pos+1] == '-') {
            while (pos < len && bytes[pos] != '\n') pos++;
            continue;
        }

        /* Block comments */
        if (pos + 1 < len && bytes[pos] == '/' && bytes[pos+1] == '*') {
            pos += 2;
            while (pos + 1 < len && !(bytes[pos] == '*' && bytes[pos+1] == '/')) pos++;
            if (pos + 1 < len) pos += 2;
            continue;
        }

        /* String literals */
        if (bytes[pos] == '"') {
            pos++;
            while (pos < len && bytes[pos] != '"') {
                if (bytes[pos] == '\\') pos++;
                pos++;
            }
            if (pos < len) pos++;
            continue;
        }

        /* # -- attributes or line comments */
        if (bytes[pos] == '#') {
            pos++;
            if (pos < len && bytes[pos] == '!') pos++;
            if (pos < len && bytes[pos] == '[') {
                pos++;
                int bracket_depth = 1;
                while (pos < len && bracket_depth > 0) {
                    if (bytes[pos] == '[') bracket_depth++;
                    else if (bytes[pos] == ']') bracket_depth--;
                    pos++;
                }
            } else {
                while (pos < len && bytes[pos] != '\n') pos++;
            }
            continue;
        }

        /* Stray closing punctuation */
        if (bytes[pos] == ')' || bytes[pos] == ']' || bytes[pos] == '}') {
            pos++;
            continue;
        }

        /* Try to read a word */
        size_t word_start = pos;
        while (pos < len && ((bytes[pos] >= 'a' && bytes[pos] <= 'z') ||
               (bytes[pos] >= 'A' && bytes[pos] <= 'Z') ||
               (bytes[pos] >= '0' && bytes[pos] <= '9') || bytes[pos] == '_'))
            pos++;

        if (pos == word_start) {
            pos++;
            continue;
        }

        size_t word_len = pos - word_start;
        char word[256];
        if (word_len >= sizeof(word)) word_len = sizeof(word) - 1;
        memcpy(word, source + word_start, word_len);
        word[word_len] = '\0';

        /* Skip "pub" modifier */
        if (strcmp(word, "pub") == 0) {
            size_t saved = pos;
            while (pos < len && (bytes[pos] == ' ' || bytes[pos] == '\t' ||
                   bytes[pos] == '\n' || bytes[pos] == '\r'))
                pos++;
            if (pos < len && bytes[pos] == '(') {
                pos++;
                int paren_depth = 1;
                while (pos < len && paren_depth > 0) {
                    if (bytes[pos] == '(') paren_depth++;
                    else if (bytes[pos] == ')') paren_depth--;
                    pos++;
                }
            } else {
                pos = saved;
            }
            continue;
        }

        /* Check if word is a grammar keyword */
        ast_kind_t kind;
        if (grammar_lookup(grammar, word, &kind)) {
            /* Skip whitespace after keyword */
            while (pos < len && (bytes[pos] == ' ' || bytes[pos] == '\t' ||
                   bytes[pos] == '\n' || bytes[pos] == '\r'))
                pos++;

            /* Extract name (allowing @ prefix and / for grammar refs) */
            size_t name_start = pos;
            if (pos < len && bytes[pos] == '@') pos++;
            while (pos < len && ((bytes[pos] >= 'a' && bytes[pos] <= 'z') ||
                   (bytes[pos] >= 'A' && bytes[pos] <= 'Z') ||
                   (bytes[pos] >= '0' && bytes[pos] <= '9') ||
                   bytes[pos] == '_' || bytes[pos] == '/'))
                pos++;

            char name[256];
            size_t name_len = pos - name_start;
            if (name_len >= sizeof(name)) name_len = sizeof(name) - 1;
            if (name_len > 0) {
                memcpy(name, source + name_start, name_len);
                name[name_len] = '\0';
            } else {
                strcpy(name, "_");
            }

            /* Project (use/in/out) ends at semicolon or newline */
            if (kind == AST_PROJECT) {
                while (pos < len && bytes[pos] != ';' && bytes[pos] != '\n') pos++;
                if (pos < len && bytes[pos] == ';') pos++;

                /* Determine In/Out/Project based on keyword */
                if (strcmp(word, "in") == 0 && name[0] == '@') {
                    ast_add_child(parent, ast_new(AST_IN, name));
                } else if (strcmp(word, "out") == 0) {
                    ast_add_child(parent, ast_new(AST_OUT, name));
                } else {
                    ast_add_child(parent, ast_new(AST_PROJECT, name));
                }
                continue;
            }

            /* Split: check for newline-terminated (no braces on this line) */
            if (kind == AST_SPLIT) {
                size_t peek = pos;
                int has_brace = 0;
                while (peek < len && bytes[peek] != '\n') {
                    if (bytes[peek] == '{') { has_brace = 1; break; }
                    peek++;
                }
                if (!has_brace) {
                    /* newline-terminated */
                    if (peek < len) pos = peek; else pos = len;
                    ast_add_child(parent, ast_new(kind, name));
                    continue;
                }
            }

            /* Skip to opening brace or semicolon */
            while (pos < len && bytes[pos] != '{' && bytes[pos] != ';' && bytes[pos] != '\n')
                pos++;

            if (pos >= len || bytes[pos] == ';' || bytes[pos] == '\n') {
                if (pos < len && bytes[pos] == ';') pos++;
                ast_add_child(parent, ast_new(kind, name));
                continue;
            }

            /* At opening brace — extract body */
            size_t body_start = pos + 1;
            pos++;
            int depth = 1;
            while (pos < len && depth > 0) {
                if (bytes[pos] == '{') depth++;
                else if (bytes[pos] == '}') depth--;
                pos++;
            }
            size_t body_end = pos > 0 ? pos - 1 : pos;

            /* For containers (Focus, Refract), scan body for children */
            ast_node_t *node = ast_new(kind, name);
            if (kind == AST_FOCUS || kind == AST_REFRACT) {
                size_t body_len = body_end - body_start;
                char *body = (char *)malloc(body_len + 1);
                memcpy(body, source + body_start, body_len);
                body[body_len] = '\0';
                scan_items(body, body_len, grammar, node);
                free(body);
            }
            ast_add_child(parent, node);

        } else if (is_skip_word(word)) {
            /* Skip word already consumed, continue */
            continue;
        } else {
            /* Not a grammar keyword. Skip brace block if present. */
            while (pos < len && (bytes[pos] == ' ' || bytes[pos] == '\t' ||
                   bytes[pos] == '\n' || bytes[pos] == '\r'))
                pos++;
            if (pos < len && bytes[pos] == '{') {
                pos++;
                int depth = 1;
                while (pos < len && depth > 0) {
                    if (bytes[pos] == '{') depth++;
                    else if (bytes[pos] == '}') depth--;
                    pos++;
                }
            }
        }
    }
}

static ast_node_t *tokenize(const char *source, size_t len, const grammar_t *grammar) {
    ast_node_t *root = ast_new(AST_FOCUS, "root");
    scan_items(source, len, grammar, root);
    return root;
}

/* =========================================================================
 * Content addressing — hash_tagged and content_oid for each variant
 *
 * Matches the Rust MirrorAST::content_oid() exactly.
 * ========================================================================= */

/* hash_tagged: canonical_hash("tag:" + content) */
static void hash_tagged(const char *tag, const unsigned char *content, size_t content_len,
                        char out[65]) {
    size_t tag_len = strlen(tag);
    size_t total = tag_len + 1 + content_len;
    unsigned char *buf = (unsigned char *)malloc(total);
    memcpy(buf, tag, tag_len);
    buf[tag_len] = ':';
    memcpy(buf + tag_len + 1, content, content_len);
    canonical_hash(buf, total, out);
    free(buf);
}

/* Dynamic buffer for building content_oid input */
typedef struct {
    unsigned char *data;
    size_t len;
    size_t cap;
} dynbuf_t;

static void dynbuf_init(dynbuf_t *b) {
    b->data = NULL; b->len = 0; b->cap = 0;
}

static void dynbuf_append(dynbuf_t *b, const unsigned char *data, size_t len) {
    if (b->len + len > b->cap) {
        b->cap = (b->len + len) * 2;
        if (b->cap < 64) b->cap = 64;
        b->data = (unsigned char *)realloc(b->data, b->cap);
    }
    memcpy(b->data + b->len, data, len);
    b->len += len;
}

static void dynbuf_free(dynbuf_t *b) {
    free(b->data);
    b->data = NULL; b->len = 0; b->cap = 0;
}

/* Compute content_oid for an AST node. Writes 64-char hex + null into out. */
static void content_oid(const ast_node_t *node, char out[65]) {
    dynbuf_t buf;
    dynbuf_init(&buf);

    switch (node->kind) {
    case AST_FOCUS: {
        /* buf = name + children OIDs */
        dynbuf_append(&buf, (const unsigned char *)node->name, strlen(node->name));
        for (int i = 0; i < node->num_children; i++) {
            char child_oid[65];
            content_oid(node->children[i], child_oid);
            dynbuf_append(&buf, (const unsigned char *)":", 1);
            dynbuf_append(&buf, (const unsigned char *)child_oid, 64);
        }
        hash_tagged("focus", buf.data, buf.len, out);
        break;
    }
    case AST_PROJECT: {
        dynbuf_append(&buf, (const unsigned char *)node->name, strlen(node->name));
        for (int i = 0; i < node->num_children; i++) {
            char child_oid[65];
            content_oid(node->children[i], child_oid);
            dynbuf_append(&buf, (const unsigned char *)":", 1);
            dynbuf_append(&buf, (const unsigned char *)child_oid, 64);
        }
        hash_tagged("project", buf.data, buf.len, out);
        break;
    }
    case AST_SPLIT: {
        dynbuf_append(&buf, (const unsigned char *)node->name, strlen(node->name));
        for (int i = 0; i < node->num_children; i++) {
            char child_oid[65];
            content_oid(node->children[i], child_oid);
            dynbuf_append(&buf, (const unsigned char *)":", 1);
            dynbuf_append(&buf, (const unsigned char *)child_oid, 64);
        }
        hash_tagged("split", buf.data, buf.len, out);
        break;
    }
    case AST_ZOOM: {
        dynbuf_append(&buf, (const unsigned char *)node->name, strlen(node->name));
        for (int i = 0; i < node->num_children; i++) {
            char child_oid[65];
            content_oid(node->children[i], child_oid);
            dynbuf_append(&buf, (const unsigned char *)":", 1);
            dynbuf_append(&buf, (const unsigned char *)child_oid, 64);
        }
        hash_tagged("zoom", buf.data, buf.len, out);
        break;
    }
    case AST_REFRACT: {
        dynbuf_append(&buf, (const unsigned char *)node->name, strlen(node->name));
        for (int i = 0; i < node->num_children; i++) {
            char child_oid[65];
            content_oid(node->children[i], child_oid);
            dynbuf_append(&buf, (const unsigned char *)":", 1);
            dynbuf_append(&buf, (const unsigned char *)child_oid, 64);
        }
        hash_tagged("refract", buf.data, buf.len, out);
        break;
    }
    case AST_IN: {
        /* In: hash_tagged("in", GrammarRef.to_string())
         * GrammarRef.to_string() for "@prism" = "@prism"
         * The name field stores the full "@grammar/ident" form. */
        const char *name = node->name;
        /* Reconstruct GrammarRef.to_string():
         * If name starts with @, it's already in the right form.
         * GrammarRef::parse strips @, then Display adds it back. */
        char ref_str[256];
        if (name[0] == '@') {
            strncpy(ref_str, name, 255);
        } else {
            snprintf(ref_str, sizeof(ref_str), "@%s", name);
        }
        ref_str[255] = '\0';
        hash_tagged("in", (const unsigned char *)ref_str, strlen(ref_str), out);
        break;
    }
    case AST_OUT: {
        const char *name = node->name;
        char ref_str[256];
        if (name[0] == '@') {
            /* Display format for GrammarRef:
             * parse("@code/rust") -> grammar="code", identifier="rust" -> display "@code/rust"
             * parse("@prism") -> grammar="prism", identifier=None -> display "@prism"
             * So name is already correct. */
            strncpy(ref_str, name, 255);
        } else {
            snprintf(ref_str, sizeof(ref_str), "@%s", name);
        }
        ref_str[255] = '\0';
        hash_tagged("out", (const unsigned char *)ref_str, strlen(ref_str), out);
        break;
    }
    }

    dynbuf_free(&buf);
}

/* =========================================================================
 * Kintsugi — canonical mirror form
 * ========================================================================= */

static void render_ast(const ast_node_t *node, int depth, dynbuf_t *out);

static void append_indent(dynbuf_t *out, int depth) {
    for (int i = 0; i < depth; i++)
        dynbuf_append(out, (const unsigned char *)"  ", 2);
}

static void render_ast(const ast_node_t *node, int depth, dynbuf_t *out) {
    switch (node->kind) {
    case AST_FOCUS:
        if (strcmp(node->name, "root") == 0 && depth == 0) {
            /* Root focus: just render children */
            for (int i = 0; i < node->num_children; i++)
                render_ast(node->children[i], depth, out);
            return;
        }
        append_indent(out, depth);
        if (node->name[0] == '@') {
            dynbuf_append(out, (const unsigned char *)"grammar ", 8);
        } else {
            dynbuf_append(out, (const unsigned char *)"focus ", 6);
        }
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        if (node->num_children > 0) {
            dynbuf_append(out, (const unsigned char *)" {\n", 3);
            for (int i = 0; i < node->num_children; i++)
                render_ast(node->children[i], depth + 1, out);
            append_indent(out, depth);
            dynbuf_append(out, (const unsigned char *)"}\n", 2);
        } else {
            dynbuf_append(out, (const unsigned char *)"\n", 1);
        }
        break;

    case AST_IN:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"in ", 3);
        if (node->name[0] != '@') dynbuf_append(out, (const unsigned char *)"@", 1);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        break;

    case AST_OUT:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"out ", 4);
        if (node->name[0] != '@') dynbuf_append(out, (const unsigned char *)"@", 1);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        break;

    case AST_PROJECT:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"project ", 8);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        break;

    case AST_SPLIT:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"type ", 5);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        for (int i = 0; i < node->num_children; i++)
            render_ast(node->children[i], depth + 1, out);
        break;

    case AST_ZOOM:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"zoom ", 5);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        for (int i = 0; i < node->num_children; i++)
            render_ast(node->children[i], depth + 1, out);
        break;

    case AST_REFRACT:
        append_indent(out, depth);
        dynbuf_append(out, (const unsigned char *)"refract ", 8);
        dynbuf_append(out, (const unsigned char *)node->name, strlen(node->name));
        dynbuf_append(out, (const unsigned char *)"\n", 1);
        for (int i = 0; i < node->num_children; i++)
            render_ast(node->children[i], depth + 1, out);
        break;
    }
}

/* =========================================================================
 * File discovery — find .mirror and .rs files recursively
 * ========================================================================= */

typedef struct {
    char **paths;
    int num;
    int cap;
} file_list_t;

static void file_list_init(file_list_t *fl) {
    fl->paths = NULL; fl->num = 0; fl->cap = 0;
}

static void file_list_add(file_list_t *fl, const char *path) {
    if (fl->num >= fl->cap) {
        fl->cap = fl->cap ? fl->cap * 2 : 16;
        fl->paths = (char **)realloc(fl->paths, fl->cap * sizeof(char *));
    }
    fl->paths[fl->num++] = strdup(path);
}

static void file_list_free(file_list_t *fl) {
    for (int i = 0; i < fl->num; i++) free(fl->paths[i]);
    free(fl->paths);
    fl->paths = NULL; fl->num = 0; fl->cap = 0;
}

static int cmp_strings(const void *a, const void *b) {
    return strcmp(*(const char **)a, *(const char **)b);
}

static void file_list_sort(file_list_t *fl) {
    qsort(fl->paths, fl->num, sizeof(char *), cmp_strings);
}

static void collect_files(const char *dir, const char *ext, file_list_t *fl) {
    DIR *d = opendir(dir);
    if (!d) return;
    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (ent->d_name[0] == '.') continue;
        char path[4096];
        snprintf(path, sizeof(path), "%s/%s", dir, ent->d_name);

        struct stat st;
        if (stat(path, &st) != 0) continue;

        if (S_ISDIR(st.st_mode)) {
            collect_files(path, ext, fl);
        } else {
            size_t nlen = strlen(ent->d_name);
            size_t elen = strlen(ext);
            if (nlen >= elen && strcmp(ent->d_name + nlen - elen, ext) == 0) {
                file_list_add(fl, path);
            }
        }
    }
    closedir(d);
}

/* =========================================================================
 * Git crystal cache
 * ========================================================================= */

/* Execute command and capture stdout. Returns malloc'd string. */
static char *exec_capture(const char *cmd) {
    FILE *f = popen(cmd, "r");
    if (!f) return NULL;
    char buf[4096];
    size_t n = fread(buf, 1, sizeof(buf)-1, f);
    pclose(f);
    buf[n] = '\0';
    /* Trim trailing whitespace */
    while (n > 0 && (buf[n-1] == '\n' || buf[n-1] == '\r' || buf[n-1] == ' ')) buf[--n] = '\0';
    return strdup(buf);
}

/* Store content in git, return blob OID */
static char *git_store(const char *content) {
    FILE *f = popen("git hash-object -w --stdin", "w");
    if (!f) return NULL;
    fwrite(content, 1, strlen(content), f);
    pclose(f);
    /* Read back by running the command in read mode */
    FILE *f2 = popen("git hash-object --stdin", "r");
    if (!f2) return NULL;
    /* Hmm, we need to feed content again. Use a temp approach. */
    pclose(f2);

    /* Better: use a pipe with both read and write */
    /* Actually, let's use the simple approach: write to temp, hash */
    /* Even simpler: exec with stdin piped */
    char cmd[4096];
    /* Write to a temp file */
    char tmpfile[] = "/tmp/mirror_git_XXXXXX";
    int fd = mkstemp(tmpfile);
    if (fd < 0) return NULL;
    write(fd, content, strlen(content));
    close(fd);
    snprintf(cmd, sizeof(cmd), "git hash-object -w %s", tmpfile);
    char *result = exec_capture(cmd);
    unlink(tmpfile);
    return result;
}

/* Store crystal OID in git refs */
static void git_store_crystal(const char *source_hash, const char *crystal_oid) {
    char *blob = git_store(crystal_oid);
    if (!blob || !*blob) { free(blob); return; }
    char cmd[4096];
    snprintf(cmd, sizeof(cmd), "git update-ref refs/crystals/%s %s", source_hash, blob);
    char *out = exec_capture(cmd);
    free(out);
    free(blob);
}

/* Check if crystal exists */
static char *git_crystal_exists(const char *source_hash) {
    char cmd[4096];
    snprintf(cmd, sizeof(cmd), "git cat-file -p refs/crystals/%s 2>/dev/null", source_hash);
    char *result = exec_capture(cmd);
    if (result && *result) return result;
    free(result);
    return NULL;
}

/* =========================================================================
 * Commands
 * ========================================================================= */

/* --- compile --- */
static int cmd_compile(const char *file, int no_cache) {
    size_t source_len;
    char *source = read_file(file, &source_len);
    if (!source) {
        fprintf(stderr, "cannot read file %s\n", file);
        return 1;
    }

    const char *grammar_path = grammar_for_file(file);
    grammar_t grammar;
    if (load_grammar(grammar_path, &grammar) != 0) {
        free(source);
        return 1;
    }

    if (!no_cache) {
        /* Check cache: hash source, look up crystal */
        char source_oid[65];
        canonical_hash((const unsigned char *)source, source_len, source_oid);
        char *cached = git_crystal_exists(source_oid);
        if (cached) {
            fprintf(stderr, "(cached)\n");
            printf("%s\n", cached);
            free(cached);
            free(source);
            grammar_free(&grammar);
            return 0;
        }
    }

    ast_node_t *ast = tokenize(source, source_len, &grammar);
    char oid[65];
    content_oid(ast, oid);

    if (!no_cache) {
        char source_oid[65];
        canonical_hash((const unsigned char *)source, source_len, source_oid);
        git_store_crystal(source_oid, oid);
    }

    printf("%s\n", oid);

    ast_free(ast);
    free(source);
    grammar_free(&grammar);
    return 0;
}

/* --- craft --- */
static int cmd_craft(const char *target, int no_cache) {
    file_list_t files;
    file_list_init(&files);

    if (strcmp(target, "boot") == 0 || strcmp(target, "std") == 0) {
        collect_files("boot", ".mirror", &files);
    } else if (strcmp(target, "cargo") == 0) {
        collect_files("src", ".rs", &files);
    } else {
        fprintf(stderr, "unknown target: %s\n", target);
        return 1;
    }
    file_list_sort(&files);

    /* Accumulate all OIDs into a hasher buffer */
    dynbuf_t hasher_buf;
    dynbuf_init(&hasher_buf);
    int hits = 0;
    int total = files.num;

    for (int i = 0; i < files.num; i++) {
        const char *file = files.paths[i];
        const char *grammar_path = grammar_for_file(file);
        grammar_t grammar;
        if (load_grammar(grammar_path, &grammar) != 0) {
            fprintf(stderr, "  skip %s (grammar error)\n", file);
            continue;
        }

        size_t source_len;
        char *source = read_file(file, &source_len);
        if (!source) {
            fprintf(stderr, "  skip %s (read error)\n", file);
            grammar_free(&grammar);
            continue;
        }

        char oid[65];
        int cached = 0;

        if (!no_cache) {
            char source_oid[65];
            canonical_hash((const unsigned char *)source, source_len, source_oid);
            char *cached_oid = git_crystal_exists(source_oid);
            if (cached_oid) {
                strncpy(oid, cached_oid, 64);
                oid[64] = '\0';
                free(cached_oid);
                cached = 1;
                hits++;
            }
        }

        if (!cached) {
            ast_node_t *ast = tokenize(source, source_len, &grammar);
            content_oid(ast, oid);
            ast_free(ast);

            if (!no_cache) {
                char source_oid[65];
                canonical_hash((const unsigned char *)source, source_len, source_oid);
                git_store_crystal(source_oid, oid);
            }
        }

        if (cached) {
            fprintf(stderr, "  %s -> %s (cached)\n", file, oid);
        } else {
            fprintf(stderr, "  %s -> %s\n", file, oid);
        }

        dynbuf_append(&hasher_buf, (const unsigned char *)oid, 64);

        free(source);
        grammar_free(&grammar);
    }

    /* Crystal OID = canonical_hash of all OIDs concatenated */
    char crystal[65];
    canonical_hash(hasher_buf.data, hasher_buf.len, crystal);

    if (hits > 0) {
        fprintf(stderr, "cache: %d/%d hits\n", hits, total);
    }

    printf("%s\n", crystal);

    dynbuf_free(&hasher_buf);
    file_list_free(&files);
    return 0;
}

/* --- kintsugi --- */
static int cmd_kintsugi(const char *file) {
    size_t source_len;
    char *source = read_file(file, &source_len);
    if (!source) {
        fprintf(stderr, "cannot read file %s\n", file);
        return 1;
    }

    const char *grammar_path = grammar_for_file(file);
    grammar_t grammar;
    if (load_grammar(grammar_path, &grammar) != 0) {
        free(source);
        return 1;
    }

    ast_node_t *ast = tokenize(source, source_len, &grammar);
    dynbuf_t output;
    dynbuf_init(&output);
    render_ast(ast, 0, &output);
    /* Write output */
    fwrite(output.data, 1, output.len, stdout);

    dynbuf_free(&output);
    ast_free(ast);
    free(source);
    grammar_free(&grammar);
    return 0;
}

/* =========================================================================
 * CLI dispatch
 * ========================================================================= */

static void usage(void) {
    fprintf(stderr, "usage: mirror <command> [args...]\n");
    fprintf(stderr, "commands: compile <file>, craft <target>, kintsugi <file>\n");
}

int main(int argc, char **argv) {
    if (argc < 3) {
        usage();
        return 1;
    }

    /* Parse --no-cache flag */
    int no_cache = 0;
    for (int i = 2; i < argc; i++) {
        if (strcmp(argv[i], "--no-cache") == 0) no_cache = 1;
    }

    /* Find first positional argument (not a flag) */
    const char *positional = NULL;
    for (int i = 2; i < argc; i++) {
        if (strncmp(argv[i], "--", 2) != 0) {
            positional = argv[i];
            break;
        }
    }

    if (strcmp(argv[1], "compile") == 0) {
        if (!positional) { fprintf(stderr, "usage: mirror compile <file>\n"); return 1; }
        return cmd_compile(positional, no_cache);
    }
    if (strcmp(argv[1], "craft") == 0) {
        if (!positional) { fprintf(stderr, "usage: mirror craft <target>\n"); return 1; }
        return cmd_craft(positional, no_cache);
    }
    if (strcmp(argv[1], "kintsugi") == 0) {
        if (!positional) { fprintf(stderr, "usage: mirror kintsugi <file>\n"); return 1; }
        return cmd_kintsugi(positional);
    }

    fprintf(stderr, "unknown: %s\n", argv[1]);
    return 1;
}

# Issue 23 inline review evidence, 2026-10-13

## Scope and source

Twoliter PR 5 inline comments 4330, 4365 and 4366. Starting revision:
`515d9ae99a8800eb0373d7f6e33a41183435c506`. This change documents existing
parameter alias handling and strengthens its regression coverage. It does not
change production behavior or the grammar.

## Grammar comparison

The original `tools/pcrsys/src/parsers/bootconfig.pest` at
`9c1b67c864c27ee88c7c5f0cf2beeb51f26a9218` and current
`tools/bootconfig/src/parsers/bootconfig.pest` at
`515d9ae99a8800eb0373d7f6e33a41183435c506` are byte-for-byte equal.
Both SHA-256: `603f097e44bb009130f2e66fc074854a2d17778bb131d7f0652ade289c6c779c`.
Verified using `git show d98480c^:tools/pcrsys/src/parsers/bootconfig.pest`
and direct file-byte comparison.

The earlier version at `342b00152ef99b93c730569913c7342c68d90519`
changed more than the shared crate extraction needed:

| Rule | Original, now restored | Earlier shared version |
| --- | --- | --- |
| Statement | `pair`, `bool_key`, `block` | `block`, `pair`, `flag` |
| Block prefix | Single identifier | Dotted key permitted |
| Block end | Closing brace | Optional trailing semicolon |
| Pair | One value node contains array items; optional semicolon | One value node per array item; required statement boundary |
| Double quotes | Backslash consumes next character in grammar | Backslash literal, next quote closes string |
| Unquoted value | Nonempty; stops at space, tab and opening brace | Empty and embedded spaces, tabs and opening braces permitted |
| Whitespace | Space, tab | Also bare carriage return |

The rewrite attempted to broaden Linux syntax support and simplify the Rust
parse-tree traversal. It was not required to share parsing or construct UKI
command lines. `ef50daeb9762b581d705d78d05d1fc6c27090728` restored the
original grammar and adapted `tools/bootconfig/src/lib.rs` to its `bool_key`
and array-node structure. `515d9ae99a8800eb0373d7f6e33a41183435c506`
aligned the nesting guard with the restored quote/comment rules.
Neither grammar implements the `+=` or `:=` assignment operators.

## Parameter spelling

Linux v6.12 registers `module_blacklist` as a core parameter. The matching
path calls `parameq`, which treats hyphens and underscores as equivalent.
The kernel documentation explicitly describes that equivalence. Thus
`module-blacklist` is an accepted spelling, not an invented kernel parameter.
The shared merger recognizes both input spellings and emits only
`kernel.module_blacklist`, preventing contributions under one spelling from
being lost when the other spelling sets the same kernel parameter.

The existing mixed-spelling fixture repeated modules already supplied under
the canonical spelling, so it could miss a lost contribution. The changed
canonical package fixture uses underscores throughout. A dedicated regression
now supplies disjoint modules under the two spellings and checks the single
canonical bootconfig assignment and resulting kernel argument.

This is not a general kernel-parameter allowlist. Parameter availability can
vary by kernel configuration; the present feedback concerns this known alias.

## Validation status

Local `cargo fmt --all -- --check` and `git diff --check` pass after formatting
the new assertion. Workspace tests, strict all-target Clippy and `make integ`
are requested through the build queue for the committed revision returned
with this evidence. They have not run in this turn. No existing build receipts
were present in this stage for the October 13 grammar revisions. Historical
issue 8 results do not validate these revisions.

No new product build, AMI registration or EC2 test was performed. No claim is
made about current instance state. The inherited completed final adversarial
review is recorded in `.symphony/evidence/workflow-recovery.md`; this feedback
does not request another critic round.

## Primary source excerpts

Retrieved read-only on 2026-10-13. These establish upstream Linux v6.12
behavior; this turn did not boot a Bottlerocket image to retest the alias.

### Linux v6.12: Documentation/admin-guide/kernel-parameters.rst

Source: `https://raw.githubusercontent.com/torvalds/linux/v6.12/Documentation/admin-guide/kernel-parameters.rst`

SHA-256 of retrieved file: `2a604b6f5290d8df49e13b6b12fda323a6b5f030d9d4f205d9217f89681ce112`

```text
30: Hyphens (dashes) and underscores are equivalent in parameter names, so::
31: 
32: 	log_buf_len=1M print-fatal-signals=1
33: 
34: can also be entered as::
35: 
36: 	log-buf-len=1M print_fatal_signals=1
```

### Linux v6.12: kernel/params.c

Source: `https://raw.githubusercontent.com/torvalds/linux/v6.12/kernel/params.c`

SHA-256 of retrieved file: `901757da5f0b485d0a37c28b135ab05fe3894c21af7f9b5dba7e9fb2a5c0328f`

```text
79: static char dash2underscore(char c)
80: {
81: 	if (c == '-')
82: 		return '_';
83: 	return c;
84: }
85: 
86: bool parameqn(const char *a, const char *b, size_t n)
87: {
88: 	size_t i;
89: 
90: 	for (i = 0; i < n; i++) {
91: 		if (dash2underscore(a[i]) != dash2underscore(b[i]))
92: 			return false;
93: 	}
94: 	return true;
95: }
96: 
97: bool parameq(const char *a, const char *b)
98: {
99: 	return parameqn(a, b, strlen(a)+1);
100: }
129: 	/* Find parameter */
130: 	for (i = 0; i < num_params; i++) {
131: 		if (parameq(param, params[i].name)) {
132: 			if (params[i].level < min_level
133: 			    || params[i].level > max_level)
134: 				return 0;
135: 			/* No one handled NULL, so do it here. */
136: 			if (!val &&
137: 			    !(params[i].ops->flags & KERNEL_PARAM_OPS_FL_NOARG))
138: 				return -EINVAL;
139: 			pr_debug("handling %s with %p\n", param,
140: 				params[i].ops->set);
141: 			kernel_param_lock(params[i].mod);
142: 			if (param_check_unsafe(&params[i]))
143: 				err = params[i].ops->set(val, &params[i]);
144: 			else
```

### Linux v6.12: kernel/module/main.c

Source: `https://raw.githubusercontent.com/torvalds/linux/v6.12/kernel/module/main.c`

SHA-256 of retrieved file: `837c2387c7aa0a109cdedb66a1d96ff9386209951ab6449ff5e73e5d8d55770b`

```text
2350: /* module_blacklist is a comma-separated list of module names */
2351: static char *module_blacklist;
2352: static bool blacklisted(const char *module_name)
2353: {
2354: 	const char *p;
2355: 	size_t len;
2356: 
2357: 	if (!module_blacklist)
2358: 		return false;
2359: 
2360: 	for (p = module_blacklist; *p; p += len) {
2361: 		len = strcspn(p, ",");
2362: 		if (strlen(module_name) == len && !memcmp(module_name, p, len))
2363: 			return true;
2364: 		if (p[len] == ',')
2365: 			len++;
2366: 	}
2367: 	return false;
2368: }
2369: core_param(module_blacklist, module_blacklist, charp, 0400);
```

# Q5: Typed error boundaries

bootconfig and ukisys use typed Snafu variants with concrete sources. pcrsys uses whatever_context at the legacy boundary, retaining the typed source; new tests downcast it. The standalone missing ResultExt import was fixed by 01c1e3f. Errors are still runtime validation of bytes/strings; changing the public data model to newtypes is outside the accepted restructuring and no uncovered type invariant was identified.

Evidence: 09380d356886842b8987b602ed2ef7daaefec4f3:tools/bootconfig/src/error.rs:1; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/ukisys/src/error.rs:25; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/pcrsys/src/pcrs/pcr9.rs:19; 09380d356886842b8987b602ed2ef7daaefec4f3:tools/pcrsys/src/pe.rs:557.

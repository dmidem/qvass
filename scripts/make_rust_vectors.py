#!/usr/bin/env python3

"""
Rust Test Vector Generator (Domain-Based Generic Structs)

This script analyzes test vectors from 'vectors.py' and automatically
generates unique Rust structs for each test group (e.g., 'PHASE_TESTS' -> 'PhaseArgs').
It then creates corresponding generic TestVector<T> static variables.
"""
import math
import sys
import subprocess
from collections import defaultdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union

import vectors

class RustConstants:
    COMPLEX64, FRAC_1_SQRT_2 = "Complex64", "FRAC_1_SQRT_2"
    ZERO, ONE, I = "ZERO", "ONE", "I"
    FROM, NEW = "from", "new"

class RustVectorGenerator:
    TOLERANCE, TESTS_SUFFIX, INDENT = 1e-10, '_TESTS', "    "
    
    def __init__(self, vectors_module):
        self.vectors_module = vectors_module
        self.frac_1_sqrt_2 = getattr(vectors_module, RustConstants.FRAC_1_SQRT_2, 1/math.sqrt(2))
        self.c = RustConstants
        # This will store mappings like:
        # 'PHASE_TESTS' -> ('PhaseArgs', frozenset({'fraction'}))
        self.group_to_arg_struct_info = {}

    # --- Formatting helpers (unchanged) ---
    def _is_close_to_zero(self, value: float) -> bool: return abs(value) < self.TOLERANCE
    def _is_close_to_frac(self, value: float) -> bool: return math.isclose(abs(value), self.frac_1_sqrt_2, abs_tol=self.TOLERANCE)
    def _is_integer(self, value: float) -> bool: return abs(value - round(value)) < self.TOLERANCE
    def _format_number_as_rust(self, num: Union[int, float, complex]) -> str:
        c = complex(num)
        match c:
            case 0+0j: return f"{self.c.COMPLEX64}::{self.c.ZERO}"
            case 1+0j: return f"{self.c.COMPLEX64}::{self.c.ONE}"
            case -1+0j: return f"-{self.c.COMPLEX64}::{self.c.ONE}"
            case 0+1j: return f"{self.c.COMPLEX64}::{self.c.I}"
            case 0-1j: return f"-{self.c.COMPLEX64}::{self.c.I}"
        if self._is_close_to_zero(c.imag): return self._format_real_number(c.real)
        return self._format_complex_number(c)
    def _format_real_number(self, real: float) -> str:
        if self._is_close_to_frac(real):
            sign = "" if real > 0 else "-"
            return f"{self.c.COMPLEX64}::{self.c.FROM}({sign}{self.c.FRAC_1_SQRT_2})"
        if self._is_integer(real): return f"{self.c.COMPLEX64}::{self.c.FROM}({int(round(real))}.0)"
        return f"{self.c.COMPLEX64}::{self.c.FROM}({real})"
    def _format_complex_number(self, c: complex) -> str:
        real_str = self._format_component(c.real)
        imag_str = self._format_component(c.imag)
        return f"{self.c.COMPLEX64}::{self.c.NEW}({real_str}, {imag_str})"
    def _format_component(self, value: float) -> str:
        if self._is_close_to_frac(value):
            sign = "" if value > 0 else "-"
            return f"{sign}{self.c.FRAC_1_SQRT_2}"
        if self._is_integer(value): return f"{int(round(value))}.0"
        return str(value)
    def _format_state_vector(self, state_vector: List[complex], indent_level: int) -> str:
        if not state_vector: return "vec![]"
        if len(state_vector) & (len(state_vector) - 1) != 0: raise ValueError(f"State vector length {len(state_vector)} is not a power of 2")
        base_indent = self.INDENT * indent_level
        n_qubits = int(math.log2(len(state_vector)))
        lines = [f"{base_indent}{self.INDENT}{self._format_number_as_rust(amp)}, // |{i:0{n_qubits}b}⟩" for i, amp in enumerate(state_vector)]
        return "vec![\n" + "\n".join(lines) + f"\n{base_indent}]"

    # --- Pass 1: Analyze and Prepare ---
    def _analyze_and_prepare_arg_structs(self):
        """
        First pass: Scan all test groups to identify their argument structure and
        assign a domain-based Rust struct name.
        """
        all_test_sets = self._discover_test_sets()
        for group_name, test_data in all_test_sets:
            # Find the first test case with args to define the structure for the whole group
            arg_keys = None
            for test_case in test_data:
                args_dict = test_case.get('kwargs') or test_case.get('args')
                if args_dict:
                    current_keys = frozenset(k for k, v in args_dict.items() if isinstance(v, (int, float)))
                    if current_keys:
                        # Assumption: all arg structures within a group are the same.
                        # We only need to find the first one.
                        arg_keys = current_keys
                        break
            
            if arg_keys:
                # Generate a struct name from the group name, e.g., "PHASE_TESTS" -> "PhaseArgs"
                base_name = group_name.replace(self.TESTS_SUFFIX, '').capitalize()
                struct_name = f"{base_name}Args"
                self.group_to_arg_struct_info[group_name] = (struct_name, arg_keys)
        
        print(f"Discovered {len(self.group_to_arg_struct_info)} test groups with custom arguments.")

    # --- Pass 2: Generation ---
    def _get_arg_type_and_value(self, group_name: str, args_dict: Optional[Dict]) -> Tuple[str, str]:
        """Determines the Rust type and value literal for a given test case."""
        if not args_dict:
            return "()", "()"

        struct_info = self.group_to_arg_struct_info.get(group_name)
        if not struct_info:
            return "()", "()" # This group has no defined argument struct

        struct_name, expected_keys = struct_info
        
        # Build the struct literal, e.g., "PhaseArgs { fraction: 0.5 }"
        fields = ", ".join(f"{k}: {args_dict[k]}" for k in sorted(list(expected_keys)))
        value_literal = f"{struct_name} {{ {fields} }}"
        
        return struct_name, value_literal

    def _generate_rust_variable(self, group_name: str, test_cases: List[Dict]) -> str:
        """Generates a Rust variable for a single group of tests."""
        first_case = test_cases[0]
        args_dict = first_case.get('kwargs') or first_case.get('args')
        arg_type, _ = self._get_arg_type_and_value(group_name, args_dict)
    
        lines = [
            f"pub(crate) static {group_name}: LazyLock<Vec<TestVector<{arg_type}>>> = LazyLock::new(|| {{",
            f"{self.INDENT}vec!["
        ]
    
        for test_case in test_cases:
            description = test_case['description'].replace('"', '\\"')
            current_args_dict = test_case.get('kwargs') or test_case.get('args')
            _, rust_arg_value = self._get_arg_type_and_value(group_name, current_args_dict)

            lines.append(f"{self.INDENT*2}// {description}")
            lines.append(f"{self.INDENT*2}TestVector {{")
        
            indent_level = 3
            indent_str = self.INDENT * indent_level
        
            # This list of extend calls now contains the corrected line for 'qubits'
            lines.extend([
                f'{indent_str}description: "{description}",',
                # --- THIS IS THE CORRECTED LINE ---
                f"{indent_str}qubits: vec!{str(test_case['qubits'])},",
                f"{indent_str}num_controls: {'Some({})'.format(test_case.get('num_controls')) if test_case.get('num_controls') is not None else 'None'},",
                f"{indent_str}args: {rust_arg_value},",
                f"{indent_str}initial_state: {self._format_state_vector(test_case['initial_state'], indent_level)},",
                f"{indent_str}expected_state: {self._format_state_vector(test_case['expected_state'], indent_level)},"
            ])
            lines.append(f"{self.INDENT*2}}},")

        lines.extend([f"{self.INDENT}]", "});"])
        return "\n".join(lines)

    def _generate_file_header(self) -> List[str]:
        """Generates the file header, including the dynamically created arg structs."""
        header = [
            "// This file is auto-generated by a Python script. Do not edit manually.",
            "// It uses a generic TestVector<T> struct and domain-based arg structs.",
            "",
            "use std::{f64::consts::FRAC_1_SQRT_2, sync::LazyLock, vec, vec::Vec};",
            "use num_complex::Complex64;",
            "",
            "// --- Argument Structs (Auto-generated from test groups) ---",
        ]

        if not self.group_to_arg_struct_info:
            header.append("// No custom argument structs needed.")
        else:
            # Sort by struct name for consistent output
            sorted_structs = sorted(self.group_to_arg_struct_info.values(), key=lambda item: item[0])
            # Use a set to avoid duplicate struct definitions
            defined_structs = set()
            for struct_name, keys in sorted_structs:
                if struct_name in defined_structs:
                    continue
                header.append("#[derive(Debug, PartialEq, Clone, Copy)]")
                header.append(f"pub(crate) struct {struct_name} {{")
                for key in sorted(list(keys)):
                    header.append(f"{self.INDENT}pub(crate) {key}: f64,")
                header.append("}")
                header.append("")
                defined_structs.add(struct_name)

        header.extend([
            "// --- Generic TestVector ---",
            "#[derive(Debug, PartialEq)]",
            "pub(crate) struct TestVector<T> {",
            "    pub(crate) description: &'static str,",
            "    pub(crate) qubits: Vec<u8>,",
            "    pub(crate) initial_state: Vec<Complex64>,",
            "    pub(crate) expected_state: Vec<Complex64>,",
            "    pub(crate) num_controls: Option<usize>,",
            "    pub(crate) args: T,",
            "}",
            "",
        ])
        return header

    def _discover_test_sets(self) -> List[Tuple[str, List[Dict]]]:
        """Discovers all test data sets from the vectors module."""
        test_sets = []
        for attr_name in sorted(dir(self.vectors_module)):
            if attr_name.endswith(self.TESTS_SUFFIX) and not attr_name.startswith('__'):
                test_data = getattr(self.vectors_module, attr_name)
                if isinstance(test_data, list) and test_data:
                    test_sets.append((attr_name, test_data))
        return test_sets

    def generate_rust_code(self) -> str:
        """Main generation orchestrator."""
        self._analyze_and_prepare_arg_structs()
        
        rust_file_content = self._generate_file_header()
        
        all_test_sets = self._discover_test_sets()
        print(f"Generating {len(all_test_sets)} Rust static variables.")

        for group_name, test_data in all_test_sets:
            try:
                # Unlike the previous version, we don't need to group. Each Python
                # list corresponds to exactly one Rust static variable.
                rust_code_block = self._generate_rust_variable(group_name, test_data)
                rust_file_content.append(rust_code_block)
                rust_file_content.append("")
                
                arg_type, _ = self._get_arg_type_and_value(group_name, (test_data[0].get('kwargs') or test_data[0].get('args')))
                print(f"  ✓ Generated {group_name}<{arg_type}> with {len(test_data)} cases")
            except Exception as e:
                print(f"  ✗ Error generating {group_name}: {e}", file=sys.stderr)
                raise
        
        return "\n".join(rust_file_content)


def main():
    print("Generating Rust test vectors from vectors.py (domain-based generic struct mode)")
    try:
        generator = RustVectorGenerator(vectors)
        generated_rust_code = generator.generate_rust_code()
        
        script_dir = Path(__file__).parent
        output_path = script_dir.parent / "src" / "tests" / "vectors.rs"
        if output_path.exists():
            backup_path = output_path.with_suffix('.rs.bak')
            output_path.rename(backup_path)
        
        output_path.write_text(generated_rust_code, encoding='utf-8')
        print(f"✓ Successfully saved to {output_path}")

        print("Formatting the generated file with 'cargo fmt'...")
        subprocess.run(
            ["cargo", "fmt", "--", str(output_path)],
            check=True,
            capture_output=True, # Captures stdout/stderr for better error messages
            text=True
        )
        print("✓ Formatting successful.")
        
    except ImportError as e:
        print(f"✗ Error: Could not import 'vectors' module. Ensure 'vectors.py' is in the same directory.", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"✗ An error occurred during Rust code generation: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()

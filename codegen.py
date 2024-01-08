#!/usr/bin/env python3

import yaml
import requests
from time import sleep
import tempfile
import subprocess


def pascal_to_snake(s):
    return "".join(["_" + c.lower() if c.isupper() else c for c in s]).lstrip("_")


rust_lib = """//! Kubernetes CRDs for cert-manager 1.13.3
//!
//! This library provides automatically generated types for the [cert-manager 1.13.3 definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [cert-manager 1.13.3 definitions]: https://github.com/cert-manager/cert-manager/releases/download/v1.13.3/cert-manager.crds.yaml
//! [Kube-rs]: https://kube.rs/

pub mod acme;
pub use acme::*;
"""

rust_acme_mod = ""

crds = yaml.safe_load_all(
    requests.get(
        "https://github.com/cert-manager/cert-manager/releases/download/v1.13.3/cert-manager.crds.yaml"
    ).text
)
for crd in crds:
    file_name = crd["metadata"]["name"].removesuffix(".cert-manager.io")
    rust_code = ""
    # Save the CRD as a tmp yaml file
    with tempfile.NamedTemporaryFile(mode="w") as f:
        yaml.dump(crd, f)
        tmp_file = f.name
        rust_code = subprocess.run(
            ["kopium", "-f", tmp_file, "--schema=derived", "--docs", "-b"],
            capture_output=True,
        )
        if rust_code.returncode != 0:
            print(rust_code.stderr.decode("utf-8"))
            exit(1)
        rust_code = rust_code.stdout.decode("utf-8")

    rust_code = rust_code.replace(
        f"// kopium command: kopium -f {tmp_file} --schema=derived --docs -b",
        f"// kopium command: kopium -f {file_name}.yml --schema=derived --docs -b",
    )
    rust_code = "\n".join(
        [
            line.replace("#[builder(", '#[cfg_attr(feature = "builder", builder(')
            .strip()
            .removesuffix("]")
            + ")]"
            if "#[builder(" in line
            else line
            for line in rust_code.split("\n")
        ]
    )
    # We're not setting PartialEq, Hash, Default with kopium because then rustfmt would insert a line break, which would make this script more complicated
    rust_code = (
        rust_code.replace(
            ", TypedBuilder, JsonSchema)]\npub struct",
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct",
        )
        .replace(
            ", TypedBuilder, JsonSchema)]\npub enum",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub enum",
        )
        .replace(
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct CertificateAdditionalOutputFormats",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub struct CertificateAdditionalOutputFormats",
        )
        .replace(
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct CertificateStatusConditions",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub struct CertificateStatusConditions",
        )
        .replace(
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct CertificateRequestStatusConditions",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub struct CertificateRequestStatusConditions",
        )
        .replace(
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct ClusterIssuerStatusConditions",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub struct ClusterIssuerStatusConditions",
        )
        .replace(
            ", PartialEq, Default, TypedBuilder, JsonSchema)]\npub struct IssuerStatusConditions",
            ", PartialEq, TypedBuilder, JsonSchema)]\npub struct IssuerStatusConditions",
        )
    )
    rust_code = "\n".join(
        [
            line.replace(
                ", TypedBuilder, JsonSchema)]",
                ')]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\n#[cfg_attr(not(feature = "schemars"), kube(schema="disabled"))]',
            )
            if line.startswith("#[derive(") and "CustomResource" in line
            else line.replace(
                ", TypedBuilder, JsonSchema)]",
                ')]\n#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]',
            )
            if line.startswith("#[derive(")
            else line
            for line in rust_code.split("\n")
        ]
    )
    rust_code = (
        rust_code.replace(
            "use typed_builder::TypedBuilder;",
            '#[cfg(feature = "builder")]\nuse typed_builder::TypedBuilder;',
        )
        .replace(
            "use schemars::JsonSchema;",
            '#[cfg(feature = "schemars")]\nuse schemars::JsonSchema;',
        )
        .replace("use kube::CustomResource;", "use kube_derive::CustomResource;")
        .replace(
            '#[cfg_attr(feature = "builder", derive(TypedBuilder))]\n#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
            '#[cfg_attr(feature = "schemars", derive(JsonSchema))]\npub enum',
        )
    )
    rust_file = f"./src/{file_name}.rs"
    if file_name.endswith(".acme"):
        file_name = file_name.removesuffix(".acme")
        rust_file = f"./src/acme/{file_name}.rs"
        rust_acme_mod += f"pub mod {file_name};\npub use {file_name}::*;\n"
    else:
        rust_lib += f"pub mod {file_name};\npub use {file_name}::*;\n"
    with open(rust_file, "w") as f:
        f.write(rust_code)
    # Format the code
    subprocess.run(["rustfmt", rust_file])

with open("./src/lib.rs", "w") as f:
    f.write(rust_lib)

with open("./src/acme/mod.rs", "w") as f:
    f.write(rust_acme_mod)

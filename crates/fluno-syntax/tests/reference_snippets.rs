use fluno_syntax::parse_module;

const MARKDOWN_DOCS: &[(&str, &str)] = &[
    ("README.md", include_str!("../../../README.md")),
    (
        "docs/syntax-overview.md",
        include_str!("../../../docs/syntax-overview.md"),
    ),
    (
        "docs/reference/language.md",
        include_str!("../../../docs/reference/language.md"),
    ),
    (
        "docs/reference/domain-syntax.md",
        include_str!("../../../docs/reference/domain-syntax.md"),
    ),
    (
        "docs/reference/stdlib.md",
        include_str!("../../../docs/reference/stdlib.md"),
    ),
    (
        "examples/hello/README.md",
        include_str!("../../../examples/hello/README.md"),
    ),
];

const PUBLIC_FLN_SOURCES: &[(&str, &str)] = &[
    (
        "stdlib/stable/core.fln",
        include_str!("../../../stdlib/stable/core.fln"),
    ),
    (
        "stdlib/stable/math.fln",
        include_str!("../../../stdlib/stable/math.fln"),
    ),
    (
        "stdlib/stable/prob.fln",
        include_str!("../../../stdlib/stable/prob.fln"),
    ),
    (
        "examples/hello/hello.fln",
        include_str!("../../../examples/hello/hello.fln"),
    ),
    (
        "examples/representative-corpus/arithmetic.fln",
        include_str!("../../../examples/representative-corpus/arithmetic.fln"),
    ),
    (
        "examples/representative-corpus/struct_enum.fln",
        include_str!("../../../examples/representative-corpus/struct_enum.fln"),
    ),
    (
        "examples/representative-corpus/probability_minimal.fln",
        include_str!("../../../examples/representative-corpus/probability_minimal.fln"),
    ),
    (
        "examples/representative-corpus/reactive_minimal.fln",
        include_str!("../../../examples/representative-corpus/reactive_minimal.fln"),
    ),
];

#[test]
fn standalone_reference_fluno_blocks_parse() {
    for (doc_name, doc) in MARKDOWN_DOCS {
        for block in fluno_blocks(doc) {
            if !looks_like_standalone_module(block.source.as_str()) {
                continue;
            }

            parse_module(block.source.as_str()).unwrap_or_else(|err| {
                panic!(
                    "{doc_name}: fluno block starting at line {} should parse: {err}\n{}",
                    block.start_line, block.source
                )
            });
        }
    }
}

#[test]
fn public_fln_sources_parse() {
    for (path, source) in PUBLIC_FLN_SOURCES {
        parse_module(source)
            .unwrap_or_else(|err| panic!("{path} should parse as a public .fln source: {err}"));
    }
}

struct CodeBlock {
    start_line: usize,
    source: String,
}

fn fluno_blocks(markdown: &str) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let mut in_fluno = false;
    let mut start_line = 0;
    let mut source = String::new();

    for (index, line) in markdown.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();

        if in_fluno {
            if trimmed.starts_with("```") {
                blocks.push(CodeBlock {
                    start_line,
                    source: source.trim().to_string(),
                });
                source.clear();
                in_fluno = false;
            } else {
                source.push_str(line);
                source.push('\n');
            }
            continue;
        }

        if trimmed == "```fluno" {
            in_fluno = true;
            start_line = line_number + 1;
            source.clear();
        }
    }

    blocks
}

fn looks_like_standalone_module(source: &str) -> bool {
    let trimmed = source.trim_start();
    ["fn ", "async fn ", "struct ", "enum ", "extern ", "mod "]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
}

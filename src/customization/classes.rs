pub fn normalize_output_language(language: &str) -> &str {
    match language {
        "shellscript" => "bash",
        "plain" => "text",
        "js" => "javascript",
        "cs" => "csharp",
        "yml" => "yaml",
        "regexp" => "regex",
        "rs" => "rust",
        l => l,
    }
}

pub fn normalize_input_language(language: &str) -> &str {
    match language {
        "plain" => "text",
        "htm" => "html",
        "cshtml" => "razor",
        "js" | "cjs" | "mjs" => "javascript",
        "cs" => "csharp",
        "yml" => "yaml",
        "py" => "python",
        "shellscript" | "sh" | "shell" | "zsh" => "bash",
        "ps1" => "powershell",
        "ts" | "cts" | "mts" => "typescript",
        "regexp" => "regex",
        "rs" => "rust",
        l => l,
    }
}

pub fn shorten_identifier(identifier: &str, prefix: &str) -> String {
    let mut shortened = true;

    let identifier = match identifier {
        // Uppercase single-letter shortcuts
        "attribute-name" => "A",
        "builtin" => "B",
        "constant" => "C",
        "delimiter" => "D",
        "error" => "E",
        "function-call" => "F",
        // G
        // H
        "invalid" | "illegal" => "I", // alias for invalid
        "property-name" => "J",
        "key-value" => "K",
        "link" => "L",
        "modifier" => "M",
        "numeric" => "N",
        "other" => "O",
        "property" => "P",
        "quarto" => "Q",
        "storage" => "R",
        "support" => "S",
        "type" => "T",
        "using" => "U",
        "value" => "V",
        "unrecognized" | "unimplemented" => "W",
        "source" => "X",
        "accessor" => "Y",
        "separator" => "Z",

        // Lowercase single-letter shortcuts
        "attribute" => "a",
        "boolean" => "b",
        "comment" => "c",
        "definition" => "d",
        "entity" => "e",
        "function" => "f",
        "group" => "g",
        "header" | "heading" => "h",
        "identifier" => "i",
        // j
        "keyword" => "k",
        "language" => "l",
        "meta" => "m",
        "name" => "n",
        "operator" => "o",
        "punctuation" => "p",
        "quote" | "quoted" => "q",
        "regexp" => "r",
        "string" | "text" => "s",
        "tag" => "t",
        "unit" => "u",
        "variable" => "v",
        "readwrite" => "w",
        // x
        "directive" => "y",
        // z    /// RESERVED

        // Uppercase underscore shortcuts
        "assignment" => "_A",
        "begin" => "_B",
        "control" => "_C",
        "diff" => "_D",
        "end" => "_E",
        "primitive" => "_P",
        "url" => "_U",

        // Lowercase underscore shortcuts
        "class" => "_c",
        "deleted" => "_d",
        "inserted" => "_i",
        "markup" => "_m",
        "component" => "_p",
        "object" => "_o",
        "range" => "_r",
        "typeparameters" => "_t",

        // Uppercase suffix underscore shortcuts
        "deprecated" => "D_",
        "embedded" => "E_",
        "percentage" => "P_",
        "section" => "S_",

        // Lowercase suffix underscore shortcuts
        "arrow" => "a_",
        "color" => "c_",
        "double" => "d_",
        "expression" => "e_",
        "inherited-class" => "i_",
        "object-literal" => "o_",
        "parameter" => "p_",
        "rgb-value" => "r_",
        "single" => "s_",
        "template-expression" => "t_",

        // Languages uppercase
        "js" | "javascript" => "J-",

        // Languages lowercase
        "bash" | "shell" => "b-",
        "css" => "c-",
        "graphql" => "g-",
        "html" => "h-",
        "ini" => "i-",
        "json" => "j-",
        "tsx" => "t-",
        "yml" | "yaml" => "y-",
        "vue" => "v-",

        // Others
        i => {
            shortened = false;
            i
        }
    };

    if shortened {
        identifier.to_string()
    } else {
        format!("{}{}", prefix, identifier)
    }
}

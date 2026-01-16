pub type IdentifierShortener = fn(&str, &str) -> String;
pub type LanguageNormalizer = fn(&str) -> &str;

pub fn normalize_language(language: &str) -> &str {
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
        "anchor" => "A",
        "builtin" => "B",
        "constant" => "C",
        "delimiter" => "D",
        "embedded" => "E",
        "function-call" => "F",
        "control" => "G",
        "html" => "H",
        "invalid" | "illegal" => "I", // alias for invalid
        "property-name" => "J",
        "key-value" => "K",
        "link" => "L",
        "modifier" => "M",
        "numeric" => "N",
        "other" => "O",
        "parenthesis" => "P",
        "quantifier" => "Q",
        "storage" => "R",
        "support" => "S",
        "type" => "T",
        "using" => "U",
        "value" => "V",
        "double" => "W",
        "class" => "X",
        "accessor" => "Y",
        "separator" => "Z",

        // Lowercase single-letter shortcuts
        "attribute-name" => "a",
        "boolean" => "b",
        "comment" => "c",
        "definition" => "d",
        "entity" => "e",
        "function" => "f",
        "group" => "g",
        "header" | "heading" => "h",
        "identifier" => "i",
        "json" => "j",
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
        "xml" => "x",
        "directive" => "y",
        // z    /// RESERVED

        // Uppercase underscore shortcuts
        "assignment" => "_A",
        "begin" => "_B",
        "character-class" => "_C",
        "diff" => "_D",
        "end" => "_E",
        "id" => "_I",
        "or" => "_O",
        "primitive" => "_P",
        "url" => "_U",
        "source" => "_S",

        // Lowercase underscore shortcuts
        "attribute" => "_a",
        "deleted" => "_d",
        "escape" => "_e",
        "font-name" => "_f",
        "inserted" => "_i",
        "key" => "_k",
        "logical" => "_l",
        "markup" => "_m",
        "negation" => "_n",
        "component" => "_p",
        "object" => "_o",
        "range" => "_r",
        "set" => "_s",
        "typeparameters" => "_t",

        // Uppercase suffix underscore shortcuts
        "character" => "C_",
        "deprecated" => "D_",
        "error" => "E_",
        "member" => "M_",
        "new" => "N_",
        "pseudo-element" => "O_",
        "option" => "O_",
        "percentage" => "P_",
        "preprocessor" => "R_",
        "section" => "S_",
        "unrecognized" | "unimplemented" => "U_", // alias for unrecognized
        "property-value" => "V_",
        "pseudo-class" => "X_",

        // Lowercase suffix underscore shortcuts
        "arrow" => "a_",
        "color" => "c_",
        "expression" => "e_",
        "inherited-class" => "i_",
        "media" => "m_",
        "namespace" => "n_",
        "object-literal" => "o_",
        "parameter" => "p_",
        "rgb-value" => "r_",
        "single" => "s_",
        "template-expression" => "t_",
        "vendored" => "v_",

        // Languages uppercase
        "js" | "javascript" => "J-",

        // Languages lowercase
        "bash" | "shell" => "b-",
        "css" => "c-",
        "graphql" => "g-",
        "ini" => "i-",
        "python" => "p-",
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

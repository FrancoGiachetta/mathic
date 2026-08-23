; Identifier

(IDENT) @variable
(field_identifier) @property
(type_identifier) @type
(native_type) @type.builtin

; Assume that all names in an import path are types
(import_path
  (IDENT) @type)

; Assume that all names of the head of a path are types and the name of the
; tail a function
(path (IDENT) @type)
(path
  (IDENT)+
  (path
  tail: (IDENT) @function))

; Assume that the name of the tail of a path that is upparcase is a constructor
(((IDENT) @type
  (#match? @type "^[A-Z]")))

; Functions

(func_decl
  name: (IDENT) @function)
(func_decl
  params: (param_list
    (IDENT) @variable.parameter))

(call_expression
  (primary
    (path
      tail: (IDENT) @function)))

(substitution_args
  sym: (IDENT) @function)

; Keywords

[
  "imp"
  "df"
  "struct"
  "let"
  "sym"
  "for"
  "in"
  "while"
  "if"
  "else"
  "return"
  "or"
  "and"
] @keyword

; Literals

(STRING) @string
(NUM) @constant.numeric

[
  "true"
  "false"
] @constant.builtin

; Operators

[
  "+"
  "-"
  "*"
  "/"
  "="
  "=="
  "!"
  "!="
  ">"
  "<"
  ">="
  "<="
] @operator

(comment) @comment

[
  ","
  "."
  ";"
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

package tree_sitter_mathic_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_mathic "github.com/francogiachetta/mathic/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_mathic.Language())
	if language == nil {
		t.Errorf("Error loading Mathic grammar")
	}
}

import XCTest
import SwiftTreeSitter
import TreeSitterMathic

final class TreeSitterMathicTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_mathic())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Mathic grammar")
    }
}

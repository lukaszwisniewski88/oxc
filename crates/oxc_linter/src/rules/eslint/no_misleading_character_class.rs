use crate::{context::LintContext, rule::Rule, AstNode};
use oxc_ast::AstKind;
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;
use regex::Regex;

#[derive(Debug, Error, Diagnostic)]
#[error("eslint(no-misleading-character-class):")]
#[diagnostic(severity(advice), help(""))]
struct NoMisleadingCharacterClassDiagnostic(#[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct NoMisleadingCharacterClass;

declare_oxc_lint!(
    /// ### What it does
    /// Disallow characters which are made with multiple code points in character class syntax
    ///
    /// ### Why is this bad?
    /// This can be misleading because the character class syntax is designed to match a single character.
    ///
    /// ### Example
    /// ```javascript
    /// var r = /[👶🏻]/";
    /// ```
    NoMisleadingCharacterClass,
    correctness
);
fn is_unicode_code_point_escape(character: char) -> bool {
    let is = Regex::new(r"^\\u\{[\da-f]+\}$").unwrap();
    is.is_match(&character.to_string())
}
impl Rule for NoMisleadingCharacterClass {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        if let AstKind::RegExpLiteral(lit) = node.kind() {
            println!("{} is a regex", lit.regex.pattern);
            lit.regex.pattern.chars().for_each(|c| {
                if c.is_ascii() {
                    println!("{} is ascii", c);
                    return;
                }
                debug_assert!(c.len_utf16() == 2);
            });
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r#"var r = /[👍]/u"#,
        r#"var r = /[\uD83D\uDC4D]/u"#,
        r#"var r = /[\u{1F44D}]/u"#,
        r#"var r = /❇️/"#,
        r#"var r = /Á/"#,
        r#"var r = /[❇]/"#,
        r#"var r = /👶🏻/"#,
        r#"var r = /[👶]/u"#,
        r#"var r = /🇯🇵/"#,
        r#"var r = /[JP]/"#,
        r#"var r = /👨‍👩‍👦/"#,
        r#"var r = /[\uD83D]/"#,
        r#"var r = /[\uDC4D]/"#,
        r#"var r = /[\uD83D]/u"#,
        r#"var r = /[\uDC4D]/u"#,
        r#"var r = /[\u0301]/"#,
        r#"var r = /[\uFE0F]/"#,
        r#"var r = /[\u0301]/u"#,
        r#"var r = /[\uFE0F]/u"#,
        r#"var r = /[\u{1F3FB}]/u"#,
        r#"var r = /[🏻]/u"#,
        r#"var r = /[🇯]/u"#,
        r#"var r = /[🇵]/u"#,
        r#"var r = /[\u200D]/"#,
        r#"var r = /[\u200D]/u"#,
        r#"var r = new RegExp('[Á] [ ');"#,
        r#"var r = RegExp('{ [Á]', 'u');"#,
        r#"var r = new globalThis.RegExp('[Á] [ ');"#,
        r#"var r = globalThis.RegExp('{ [Á]', 'u');"#,
        r#"var r = /[👍]/v"#,
        r#"var r = /[🇯[A]🇵]/v"#,
        r#"var r = /[🇯[A--B]🇵]/v"#,
    ];

    let fail = vec![
        r#"var r = /[👍]/"#,
        r#"var r = /[\uD83D\uDC4D]/"#,
        r#"var r = /[👍]/"#,
        r#"var r = /[👍]/"#,
        r#"var r = /[👍]\a/"#,
        r#"var r = /(?<=[👍])/"#,
        r#"var r = /(?<=[👍])/"#,
        r#"var r = /[Á]/"#,
        r#"var r = /[Á]/u"#,
        r#"var r = /[\u0041\u0301]/"#,
        r#"var r = /[\u0041\u0301]/u"#,
        r#"var r = /[\u{41}\u{301}]/u"#,
        r#"var r = /[❇️]/"#,
        r#"var r = /[❇️]/u"#,
        r#"var r = /[\u2747\uFE0F]/"#,
        r#"var r = /[\u2747\uFE0F]/u"#,
        r#"var r = /[\u{2747}\u{FE0F}]/u"#,
        r#"var r = /[👶🏻]/"#,
        r#"var r = /[👶🏻]/u"#,
        r#"var r = /[\uD83D\uDC76\uD83C\uDFFB]/u"#,
        r#"var r = /[\u{1F476}\u{1F3FB}]/u"#,
        r#"var r = /[🇯🇵]/"#,
        r#"var r = /[🇯🇵]/i"#,
        r#"var r = /[🇯🇵]/u"#,
        r#"var r = /[\uD83C\uDDEF\uD83C\uDDF5]/u"#,
        r#"var r = /[\u{1F1EF}\u{1F1F5}]/u"#,
        r#"var r = /[👨‍👩‍👦]/"#,
        r#"var r = /[👨‍👩‍👦]/u"#,
        r#"var r = /[\uD83D\uDC68\u200D\uD83D\uDC69\u200D\uD83D\uDC66]/u"#,
        r#"var r = /[\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466}]/u"#,
        r#"var r = new RegExp('[👍]', ``)"#,
        r#"var r = new RegExp('[🇯🇵]', `i`)"#,
        r#"var r = new RegExp('[🇯🇵]', `${foo}`)"#,
        r#"var r = /[[👶🏻]]/v"#,
        r#"var r = /[👍]/"#,
        r#"var r = /[👍]/"#,
    ];

    Tester::new_without_config(NoMisleadingCharacterClass::NAME, pass, fail).test_and_snapshot();
}

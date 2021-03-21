use std::error;
use crate::renderer::Renderer;

type ParseResult<'a, T> = Result<(T, &'a str), Box<dyn error::Error>>;

fn parse_ok<T>(ret: T, rest: &str) -> ParseResult<T> {
    Ok((ret, rest))
}

fn parse_error<T>(err: Box<dyn error::Error>) -> ParseResult<'static, T> {
    Err(err)
}

//-----------------------------------------------------------------------------

pub fn parse(s: &str, renderer: &impl Renderer) -> Result<String, Box<dyn error::Error>> {
    let mut ret = String::new();
    let mut rest = s;

    loop {
        let (paragraph, r) = parse_paragraph(rest, renderer)?;

        rest = r;
        ret.push_str(&paragraph);

        if rest.is_empty() {
            return Ok(ret);
        }
    }
}

fn parse_paragraph<'a>(s: &'a str, renderer: &impl Renderer) -> ParseResult<'a, String> {
    let mut paragraph = String::new();
    let mut rest = s;

    loop {
        let (line, r) = split_line(rest);

        rest = r;
        if !is_blank(line) {
            paragraph.push_str(line);
        }

        let done =
            rest.is_empty() ||
            (is_blank(line) && !paragraph.is_empty());
        if done {
            let ret = renderer.render_paragraph(&paragraph);
            return parse_ok(ret, rest);
        }
    }
}

// s から最初の行を切り出して (最初の行, 残りの文字列) を返す。
// 「最初の行」は行末の改行文字を含む。
fn split_line(s: &str) -> (&str, &str) {
    s.find('\n')
        .map(|index| s.split_at(index + 1))
        .unwrap_or((s, ""))
}

#[test]
fn test_split_line() {
    // 典型的なケース
    assert_eq!(split_line("line1\nline2\nline3"), ("line1\n", "line2\nline3"));
    // 空行
    assert_eq!(split_line("\nline2\nline3"), ("\n", "line2\nline3"));
    // 最後の行
    assert_eq!(split_line("last line"), ("last line", ""));
    // 空文字列
    assert_eq!(split_line(""), ("", ""));
}

// s が空白文字のみを含むならば true を返し、そうでないならば false を返す。
fn is_blank(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

#[test]
fn test_is_blank() {
    assert_eq!(is_blank(""), true);
    assert_eq!(is_blank("\n"), true);
    assert_eq!(is_blank("  \n"), true);
    assert_eq!(is_blank("\t\n"), true);
    assert_eq!(is_blank("　\n"), true);
    assert_eq!(is_blank("a"), false);
    assert_eq!(is_blank("Hello, World!\n"), false);
}

use crate::matcher::Matcher;
use crate::mp::Mp;

/// Extracts the code block (component or template) from the input string based on the matcher.
/// Returns an Mp struct with the extracted scope and position info.
pub fn collect_scope(toks: &str, matcher: &Matcher, i_s: bool) -> Option<Mp> {
    let matcher_str = matcher.as_str();
    let start_idx = toks.find(matcher_str)?;

    match matcher {
        Matcher::Component(_) => {
            let after_match = &toks[start_idx + matcher_str.len()..];
            let open_brace_idx = after_match.find('{')?;
            let before_brace = &after_match[..open_brace_idx];

            if before_brace.trim().is_empty() {
                // Find the matching closing brace for the component block
                if let Some(close_brace_idx) = after_match.rfind('}') {
                    let content_start = open_brace_idx + 1;
                    let content_end = close_brace_idx;
                    let content = after_match
                        .get(content_start..content_end)
                        .unwrap_or("")
                        .to_string();

                    let start = if i_s { start_idx } else { start_idx + matcher_str.len() + open_brace_idx + 1 };
                    let end = if i_s { Some(start_idx + matcher_str.len() + close_brace_idx) } else { None };

                    return Some(Mp::new(content, start, end));
                }
                None
            } else {
                // If not a valid match, try to find the next occurrence recursively
                let next = &toks[start_idx + matcher_str.len()..];
                collect_scope(next, &Matcher::Component(matcher_str), i_s)
            }
        }
        Matcher::Template => {
            // Find the opening <temp> and closing </temp> tags
            let remain = &toks[start_idx..];
            if let Some(close_tag_idx) = remain.find("</temp>") {
                let content = remain
                    .get(..close_tag_idx+7)
                    .unwrap_or("")
                    .to_string();
                return Some(Mp::new(content, start_idx, None));
            } else {
                panic!("</temp> expected to end the template scope");
            }
        }
    }
}

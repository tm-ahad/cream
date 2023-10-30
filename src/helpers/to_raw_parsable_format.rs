use crate::parsable_format::ParsableFormat;

pub fn to_raw_parsable_format(script: &String, html: &String) -> ParsableFormat {
    let raw = format!("
{script}
<temp>
{html}
</temp>
    ");

    let temp_starts = script.len() + 8;
    ParsableFormat::new(raw, temp_starts)
}

use std::io;

fn indent<W>(wr: &mut W, n: usize) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        wr.write_all(b"  ")?;
    }
    Ok(())
}

fn format(
    value: &serde_json::Value,
    mut out: &mut impl io::Write,
    current_indent: usize,
) -> io::Result<()> {
    match value {
        serde_json::Value::Null => {
            out.write_all(b"null")?;
        }
        serde_json::Value::Bool(true) => {
            out.write_all(b"true")?;
        }
        serde_json::Value::Bool(false) => {
            out.write_all(b"false")?;
        }
        serde_json::Value::Number(v) => {
            serde_json::to_writer(out, v)?;
        }
        serde_json::Value::String(v) => {
            serde_json::to_writer(out, v)?;
        }
        serde_json::Value::Array(v) => {
            let unflattened = v.iter().any(|v| v.is_object() || v.is_array());
            let mut first = true;

            out.write_all(b"[")?;
            if unflattened {
                for v in v {
                    let sep = if first {
                        first = false;
                        b"\n".as_ref()
                    } else {
                        b",\n".as_ref()
                    };
                    out.write_all(sep)?;

                    indent(out, current_indent + 1)?;

                    format(v, out, current_indent + 1)?;
                }

                if !v.is_empty() {
                    out.write_all(b"\n")?;
                    indent(out, current_indent)?;
                }
            } else {
                for v in v {
                    if first {
                        first = false;
                    } else {
                        out.write_all(b", ")?;
                    }

                    serde_json::to_writer(&mut out, v)?;
                }
            }
            out.write_all(b"]")?;
        }
        serde_json::Value::Object(obj) => {
            let unflattened = obj.iter().any(|(_, v)| v.is_object() || v.is_array());
            let mut first = true;

            out.write_all(b"{")?;
            if unflattened {
                for (key, value) in obj {
                    if first {
                        first = false;
                        out.write_all(b"\n")?;
                    } else {
                        out.write_all(b",\n")?;
                    }
                    indent(out, current_indent + 1)?;

                    if true {
                        serde_json::to_writer(&mut out, key)?;
                    } else {
                        out.write_all(key.as_bytes())?;
                    }

                    out.write_all(b": ")?;
                    format(value, out, current_indent + 1)?;
                }

                if !obj.is_empty() {
                    out.write_all(b"\n")?;
                    indent(out, current_indent)?;
                }
            } else {
                out.write_all(b" ")?;
                for (key, value) in obj {
                    if first {
                        first = false;
                        out.write_all(b"")?;
                    } else {
                        out.write_all(b", ")?;
                    }

                    if true {
                        serde_json::to_writer(&mut out, key)?;
                    } else {
                        out.write_all(key.as_bytes())?;
                    }

                    out.write_all(b": ")?;
                    format(value, out, current_indent + 1)?;
                }

                if !obj.is_empty() {
                    out.write_all(b" ")?;
                }
            }
            out.write_all(b"}")?;
        }
    }

    Ok(())
}

fn main() {
    let items = serde_json::Deserializer::from_reader(io::stdin()).into_iter::<serde_json::Value>();

    for src in items {
        let value = src.unwrap();
        let mut out = Vec::new();
        format(&value, &mut out, 0).unwrap();

        let out = String::from_utf8(out).unwrap();

        println!("{out}");
    }
}

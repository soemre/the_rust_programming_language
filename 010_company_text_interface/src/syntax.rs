pub fn interpret_line(line: &str) -> Result<Request, String> {
    let line = line.trim();
    if line.len() == 0 {
        return Err("no input found".to_string());
    }

    let (kw, line) = get_keyword(line);
    match kw.as_str() {
        "ADD" => {
            let (t1, line) = match Target::interpret(line) {
                Err(err) => return Err(err),
                Ok(v) => v,
            };

            let (t2, _) = match Target::interpret(line) {
                Err(err) => return Err(err),
                Ok(v) => v,
            };

            Ok(Request::Add {
                subject: t1,
                to: t2,
            })
        }
        "REMOVE" => {
            let (t1, line) = match Target::interpret(line) {
                Err(err) => return Err(err),
                Ok(v) => v,
            };

            let (t2, _) = match Target::interpret(line) {
                Err(err) => return Err(err),
                Ok(v) => v,
            };

            Ok(Request::Remove {
                subject: t1,
                from: t2,
            })
        }
        "SHOW" => match Target::interpret(line) {
            Ok((t, _)) => Ok(Request::Show { subject: t }),
            Err(err) => Err(err),
        },
        kw => Err(format!("not a valid keyword: {kw}")),
    }
}

/// Looks for the first piece of text written in double quotes.
///
/// Return previous, un-quoted found string, rest of the text.
fn get_quoted(text: &str) -> Option<(&str, &str, &str)> {
    const DELIMITER: char = '\"';
    if let Some((p, q)) = text.split_once(DELIMITER) {
        if let Some((q, s)) = q.split_once(DELIMITER) {
            return Some((p, q, s));
        }
    }
    None
}

fn get_keyword(text: &str) -> (String, &str) {
    let (key, rest) = text.split_once(' ').unwrap_or((text, ""));
    (key.to_uppercase(), rest)
}

pub enum Request {
    Add { subject: Target, to: Target },
    Remove { subject: Target, from: Target },
    Show { subject: Target },
}

pub enum Target {
    As(String),
    All,
}

impl Target {
    /// Checks if the given text have a target as its next item.
    ///
    /// Returns a target instance and the rest of the text if any found.
    fn interpret(text: &str) -> Result<(Self, &str), String> {
        let text = text.trim();

        if let Some(i) = text.find('\"') {
            if i == 0 {
                return match get_quoted(text) {
                    Some((_, quoted, suf)) => Ok((Target::As(quoted.to_string()), suf)),
                    None => Err("missing \" character".to_string()),
                };
            }
        }

        let (key, suf) = text.split_once(' ').unwrap_or((text, ""));
        if key.to_uppercase() == "ALL" {
            Ok((Target::All, suf))
        } else {
            Err(format!("not a valid target {key}"))
        }
    }
}

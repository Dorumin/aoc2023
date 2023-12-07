fn main() {
    let text = "amongus";
    let bold = true;
    let italic = true;
    let strike = true;
    let formatted = {
        let mut tags = Vec::new();
        bold.then(|| tags.push("b"));
        italic.then(|| tags.push("i"));
        strike.then(|| tags.push("s"));

        let prefix: String = tags.iter().flat_map(|tag| ["<", tag, ">"]).collect();
        let suffix: String = tags.iter().rev().flat_map(|tag| ["</", tag, ">"]).collect();

        format!("{prefix}{text}{suffix}")
    };

    dbg!(formatted);
}
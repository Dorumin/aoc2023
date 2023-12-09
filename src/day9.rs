#[derive(Debug)]
pub struct Oasis {
    histories: Vec<History>
}

#[derive(Debug)]
pub struct History {
    values: Vec<i32>
}

impl Oasis {
    pub fn parse(input: &str) -> Option<Oasis> {
        let histories: Option<Vec<History>> = input.lines().map(|line| History::parse_line(line)).collect();

        Some(Oasis {
            histories: histories?
        })
    }

    pub fn histories(&self) -> &[History] {
        &self.histories
    }
}

impl History {
    pub fn parse_line(line: &str) -> Option<History> {
        // Fancy magic to turn a vector of options into an option of a vector. Don't ask me how it works
        let values: Option<Vec<_>> = line.split_ascii_whitespace().map(|n| n.parse().ok()).collect();

        Some(History {
            values: values?
        })
    }

    pub fn predict_next_value(&self) -> (i32, i32) {
        // cloned, owned vecs. idgaf. I don't want to special case the first value
        let mut layers = vec![self.values.clone()];

        // do this asinine process to get the next value in the values chain
        while let Some(last_layer) = layers.last() {
            if last_layer.iter().all(|&n| n == 0) {
                break;
            }

            assert!(last_layer.len() >= 2);

            let mut next_layer = Vec::with_capacity(last_layer.len());

            for pair in last_layer.windows(2) {

                let [a, b] = pair else {
                    unreachable!();
                };

                next_layer.push(b - a);
            }

            layers.push(next_layer);
        }

        // reconstruct new final values for each layer
        // no need to actually "add" it at the end of each layer, we can just keep a running cell
        let first_predicted_cell = layers.iter()
            .rev()
            .skip(1)
            .map(|layer| layer.first().unwrap())
            .fold(0, |sum, n| n - sum);
        let last_predicted_cell = layers.iter()
            .rev()
            .skip(1)
            .map(|layer| layer.last().unwrap())
            .fold(0, |sum, n| sum + n);

        (first_predicted_cell, last_predicted_cell)
    }
}
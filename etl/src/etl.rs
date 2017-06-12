use std::collections::HashMap;

pub fn transform(data: &HashMap<i32, Vec<&str>>) -> HashMap<String, i32> {
    data.iter()
        .fold(HashMap::new(), |mut acc, (key, values)| {
            let tuples: HashMap<String, i32> = values.iter()
                .map(|value| (value.to_lowercase(), *key))
                .collect();
            acc.extend(tuples);
            acc
        })
}

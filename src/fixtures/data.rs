use serde_json::value::Value;

pub fn make_data() -> Value {
    json!({
            "team": "Jiangsu Suning",
            "players": [{
                "name": "Teixeira",
                "apps": 7,
                "goals": 2,
                "assists": 1
            }, {
                "name": "Ramires",
                "apps": 6,
                "goals": 2,
                "assists": 3
            }, {
                "name": "Wu Xi",
                "apps": 7,
                "goals": 1,
                "assists": 2
            }]
        })
}

use crate::prelude::*;

pub fn get() -> Vec<Vec<LineGroup>> {
    vec![
        // #1
        // 1 1 1 1 2 1 1 1 2 1 1 1 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
        ],
        // #2
        // 6 5 5
        vec![
            LineGroup { size: 6 },
            LineGroup { size: 5 },
            LineGroup { size: 5 },
        ],
        // #3
        // 1 1 1 1 2 1 1 1 1 1 1 1 1 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
        ],
        // #4
        // 1 1 2 1 1 2 2 1 1 2 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
        ],
        // #5
        // 2 2 2 2 2 2 2 2
        vec![
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
        ],
        // #6
        // 1 1 1 1 2 1 1 2 1 1 1 1 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
        ],
        // #7
        // 3 3 3 3 4
        vec![
            LineGroup { size: 3 },
            LineGroup { size: 3 },
            LineGroup { size: 3 },
            LineGroup { size: 3 },
            LineGroup { size: 4 },
        ],
        // #8
        // 1 2 1 1 2 1 2 1 1 1 1 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
        ],
        // #9
        // 2 3 2 2 3 2 2
        vec![
            LineGroup { size: 2 },
            LineGroup { size: 3 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 3 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
        ],
        // #10
        // 1 2 2 2 1 2 2 2 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
        ],
        // #11
        // 1 1 1 2 2 2 1 1 1 2 2
        vec![
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 1 },
            LineGroup { size: 2 },
            LineGroup { size: 2 },
        ],
        // #12
        // 8 8
        vec![LineGroup { size: 8 }, LineGroup { size: 8 }],
    ]
}

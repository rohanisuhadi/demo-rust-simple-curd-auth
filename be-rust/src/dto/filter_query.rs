#[derive(Debug, Clone)] // Tambahkan derive untuk debug dan cloning jika perlu
pub enum FilterValue {
    Like(String),
    String(String),
    Bool(bool),
    // StrList(Vec<String>),
    // UuidList(Vec<uuid::Uuid>),
}

#[derive(Debug, Clone)]
pub enum FilterOperator {
    Eq, // =
    // In,   // IN
    Like, // LIKE
}

impl FilterOperator {
    pub fn to_sql(&self) -> &'static str {
        match self {
            FilterOperator::Eq => "=",
            // FilterOperator::In => "IN",
            FilterOperator::Like => "LIKE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterParam {
    pub column: String,
    pub op: FilterOperator,
    pub value: FilterValue,
}

impl FilterParam {
    pub fn to_sql_and_params(&self, start_index: usize) -> (String, Vec<FilterValue>) {
        match &self.op {
            // FilterOperator::In => match &self.value {
            //     FilterValue::StrList(list) => {
            //         let placeholders: Vec<String> = (0..list.len())
            //             .map(|i| format!("${}", start_index + i))
            //             .collect();

            //         (
            //             format!(
            //                 "{} {} ({})",
            //                 self.column,
            //                 self.op.to_sql(),
            //                 placeholders.join(",")
            //             ),
            //             vec![self.value.clone()],
            //         )
            //     }

            //     FilterValue::UuidList(list) => {
            //         let placeholders: Vec<String> = (0..list.len())
            //             .map(|i| format!("${}", start_index + i))
            //             .collect();

            //         (
            //             format!("{} IN ({})", self.column, placeholders.join(",")),
            //             vec![self.value.clone()],
            //         )
            //     }

            //     _ => panic!("IN operator requires list"),
            // },
            FilterOperator::Eq => (
                format!("{} {} ${}", self.column, self.op.to_sql(), start_index),
                vec![self.value.clone()],
            ),
            FilterOperator::Like => (
                format!("{} {} ${}", self.column, self.op.to_sql(), start_index),
                vec![self.value.clone()],
            ),
        }
    }
}

pub fn build_filters(filters: &[FilterParam]) -> (String, Vec<FilterValue>) {
    let mut sql = vec![];
    let mut params = vec![];
    let mut index = 1;

    for f in filters {
        let (part_sql, mut part_params) = f.to_sql_and_params(index);
        index += part_params.len();

        sql.push(part_sql);
        params.append(&mut part_params);
    }

    let where_sql = if sql.is_empty() {
        "".into()
    } else {
        format!("WHERE {}", sql.join(" AND "))
    };

    (where_sql, params)
}

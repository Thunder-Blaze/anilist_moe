use serde_json::{json, Value};
use std::collections::HashMap;

/// Utility functions for working with GraphQL queries and variables
pub struct QueryUtils;

impl QueryUtils {
    /// Create a GraphQL request body with query and variables
    pub fn create_request_body(query: &str, variables: Value) -> Value {
        json!({
            "query": query,
            "variables": variables
        })
    }

    /// Create a GraphQL request body with just a query (no variables)
    pub fn create_simple_request(query: &str) -> Value {
        json!({
            "query": query
        })
    }

    /// Validate that required variables are present
    pub fn validate_required_vars(variables: &Value, required: &[&str]) -> Result<(), String> {
        let vars_obj = variables.as_object()
            .ok_or("Variables must be an object")?;

        for &required_var in required {
            if !vars_obj.contains_key(required_var) {
                return Err(format!("Missing required variable: {}", required_var));
            }
        }

        Ok(())
    }

    /// Clean up variables by removing null/empty values recursively
    pub fn clean_variables(value: Value) -> Value {
        match value {
            Value::Object(mut map) => {
                map.retain(|_key, val| !Self::is_empty_value(val));

                for (_, val) in map.iter_mut() {
                    *val = Self::clean_variables(val.clone());
                }

                Value::Object(map)
            }
            Value::Array(vec) => {
                let cleaned: Vec<Value> = vec.into_iter()
                    .filter(|v| !Self::is_empty_value(v))
                    .map(Self::clean_variables)
                    .collect();

                Value::Array(cleaned)
            }
            other => other,
        }
    }

    /// Check if a value should be considered "empty" and filtered out
    fn is_empty_value(value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::String(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }

    /// Convert a pagination result to include helpful metadata
    pub fn parse_pagination_info(data: &Value) -> Option<PaginationInfo> {
        data.get("Page")?
            .get("pageInfo")
            .and_then(|page_info| {
                Some(PaginationInfo {
                    current_page: page_info.get("currentPage")?.as_i64()? as i32,
                    last_page: page_info.get("lastPage")?.as_i64()? as i32,
                    total: page_info.get("total")?.as_i64()? as i32,
                    per_page: page_info.get("perPage")?.as_i64()? as i32,
                    has_next_page: page_info.get("hasNextPage")?.as_bool()?,
                })
            })
    }

    /// Create pagination variables for common queries
    pub fn create_pagination_vars(page: Option<i32>, per_page: Option<i32>) -> Value {
        let mut vars = HashMap::new();

        if let Some(page) = page {
            vars.insert("page".to_string(), json!(page));
        }

        if let Some(per_page) = per_page {
            vars.insert("perPage".to_string(), json!(per_page));
        }

        json!(vars)
    }

    /// Create date range variables for queries
    pub fn create_date_range_vars(
        start_year: Option<i32>,
        end_year: Option<i32>
    ) -> Value {
        let mut vars = HashMap::new();

        if let Some(start) = start_year {
            vars.insert("startDate_greater".to_string(), json!(start * 10000));
        }

        if let Some(end) = end_year {
            vars.insert("endDate_lesser".to_string(), json!(end * 10000 + 1231));
        }

        json!(vars)
    }

    /// Create score range variables
    pub fn create_score_range_vars(
        min_score: Option<i32>,
        max_score: Option<i32>
    ) -> Value {
        let mut vars = HashMap::new();

        if let Some(min) = min_score {
            vars.insert("averageScore_greater".to_string(), json!(min));
        }

        if let Some(max) = max_score {
            vars.insert("averageScore_lesser".to_string(), json!(max));
        }

        json!(vars)
    }

    /// Merge multiple variable objects together
    pub fn merge_variables(vars: Vec<Value>) -> Value {
        let mut merged = HashMap::new();

        for var_obj in vars {
            if let Value::Object(map) = var_obj {
                for (key, value) in map {
                    merged.insert(key, value);
                }
            }
        }

        json!(merged)
    }

    /// Convert enum to GraphQL string format
    pub fn enum_to_graphql_string<T: std::fmt::Debug>(value: T) -> String {
        format!("{:?}", value).to_uppercase()
    }

    /// Convert vector of enums to GraphQL array
    pub fn enums_to_graphql_array<T: std::fmt::Debug>(values: Vec<T>) -> Value {
        let strings: Vec<String> = values.into_iter()
            .map(|v| Self::enum_to_graphql_string(v))
            .collect();
        json!(strings)
    }
}

/// Pagination information extracted from GraphQL responses
#[derive(Debug, Clone)]
pub struct PaginationInfo {
    pub current_page: i32,
    pub last_page: i32,
    pub total: i32,
    pub per_page: i32,
    pub has_next_page: bool,
}

impl PaginationInfo {
    pub fn has_previous_page(&self) -> bool {
        self.current_page > 1
    }

    pub fn next_page(&self) -> Option<i32> {
        if self.has_next_page {
            Some(self.current_page + 1)
        } else {
            None
        }
    }

    pub fn previous_page(&self) -> Option<i32> {
        if self.has_previous_page() {
            Some(self.current_page - 1)
        } else {
            None
        }
    }

    pub fn total_pages(&self) -> i32 {
        self.last_page
    }
}

/// Common preset variable builders for frequently used queries
pub struct QueryPresets;

impl QueryPresets {
    /// Popular anime from current season
    pub fn popular_current_season() -> Value {
        json!({
            "type": "ANIME",
            "season": "SPRING", // Would be dynamically set based on current date
            "seasonYear": 2024,
            "sort": ["POPULARITY_DESC"],
            "page": 1,
            "perPage": 20
        })
    }

    /// Top rated anime of all time
    pub fn top_rated_anime() -> Value {
        json!({
            "type": "ANIME",
            "sort": ["SCORE_DESC"],
            "averageScore_greater": 80,
            "page": 1,
            "perPage": 50
        })
    }

    /// Trending anime right now
    pub fn trending_anime() -> Value {
        json!({
            "type": "ANIME",
            "sort": ["TRENDING_DESC"],
            "page": 1,
            "perPage": 20
        })
    }

    /// Popular manga
    pub fn popular_manga() -> Value {
        json!({
            "type": "MANGA",
            "sort": ["POPULARITY_DESC"],
            "page": 1,
            "perPage": 20
        })
    }

    /// Recently updated manga
    pub fn recently_updated_manga() -> Value {
        json!({
            "type": "MANGA",
            "status": "RELEASING",
            "sort": ["UPDATED_AT_DESC"],
            "page": 1,
            "perPage": 25
        })
    }

    /// Upcoming anime
    pub fn upcoming_anime() -> Value {
        json!({
            "type": "ANIME",
            "status": "NOT_YET_RELEASED",
            "sort": ["START_DATE"],
            "page": 1,
            "perPage": 25
        })
    }

    /// User's anime list (requires authentication)
    pub fn user_anime_list(user_id: i32, status: Option<&str>) -> Value {
        let mut vars = json!({
            "userId": user_id,
            "type": "ANIME",
            "sort": ["UPDATED_TIME_DESC"]
        });

        if let Some(status) = status {
            vars.as_object_mut().unwrap().insert("status".to_string(), json!(status));
        }

        vars
    }

    /// User's manga list (requires authentication)
    pub fn user_manga_list(user_id: i32, status: Option<&str>) -> Value {
        let mut vars = json!({
            "userId": user_id,
            "type": "MANGA",
            "sort": ["UPDATED_TIME_DESC"]
        });

        if let Some(status) = status {
            vars.as_object_mut().unwrap().insert("status".to_string(), json!(status));
        }

        vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_request_body() {
        let query = "query { Media { id title } }";
        let variables = json!({"id": 123});
        let body = QueryUtils::create_request_body(query, variables);

        assert!(body.get("query").is_some());
        assert!(body.get("variables").is_some());
    }

    #[test]
    fn test_clean_variables() {
        let dirty_vars = json!({
            "validString": "hello",
            "emptyString": "",
            "nullValue": null,
            "validNumber": 42,
            "emptyArray": [],
            "validArray": [1, 2, 3],
            "emptyObject": {},
            "validObject": {"key": "value"}
        });

        let cleaned = QueryUtils::clean_variables(dirty_vars);
        println!("{}", serde_json::to_string_pretty(&cleaned).unwrap());

        // Should only contain validString, validNumber, validArray, validObject
        assert!(cleaned.get("validString").is_some());
        assert!(cleaned.get("emptyString").is_none());
        assert!(cleaned.get("nullValue").is_none());
        assert!(cleaned.get("validNumber").is_some());
        assert!(cleaned.get("emptyArray").is_none());
        assert!(cleaned.get("validArray").is_some());
        assert!(cleaned.get("emptyObject").is_none());
        assert!(cleaned.get("validObject").is_some());
    }

    #[test]
    fn test_merge_variables() {
        let vars1 = json!({"page": 1, "perPage": 25});
        let vars2 = json!({"type": "ANIME", "sort": ["POPULARITY_DESC"]});
        let vars3 = json!({"averageScore_greater": 80});

        let merged = QueryUtils::merge_variables(vec![vars1, vars2, vars3]);
        println!("{}", serde_json::to_string_pretty(&merged).unwrap());

        assert_eq!(merged.get("page").unwrap(), &json!(1));
        assert_eq!(merged.get("type").unwrap(), &json!("ANIME"));
        assert_eq!(merged.get("averageScore_greater").unwrap(), &json!(80));
    }
}

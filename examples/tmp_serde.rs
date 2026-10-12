use serde::Serialize;
use serde_with::skip_serializing_none;
use std::borrow::Cow;

fn ser<S>(value: &Option<Cow<'_, [u8]>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let list: &[u8] = value.as_deref().unwrap_or(&[]);
    list.serialize(serializer)
}

/// Never skip, even when the value is `None`.
fn never<T>(_: &T) -> bool {
    false
}

#[skip_serializing_none]
#[derive(Serialize)]
struct Opts<'a> {
    #[serde(serialize_with = "ser", skip_serializing_if = "never")]
    sort: Option<Cow<'a, [u8]>>,
    other: Option<u8>,
}

fn main() {
    println!(
        "none: {}",
        serde_json::to_string(&Opts {
            sort: None,
            other: None
        })
        .unwrap()
    );
    println!(
        "empty: {}",
        serde_json::to_string(&Opts {
            sort: Some(Cow::Borrowed(&[])),
            other: None
        })
        .unwrap()
    );
    println!(
        "some: {}",
        serde_json::to_string(&Opts {
            sort: Some(Cow::Borrowed(&[1, 2])),
            other: Some(3)
        })
        .unwrap()
    );
}

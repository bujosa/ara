#![allow(clippy::needless_lifetimes)]

use async_graphql::{
    connection::{query, Connection, Edge},
    Context, Enum, Error, Interface, Object, OutputType, Result,
};

use super::NarutoShippuden;
use crate::naruto::NarutoChar;

/// This enum represent the villages in Naruto.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Village {
    /// Konohagakure - Village Hidden in the Leaves
    Konoha,
    /// Sunagakure - Village Hidden in the Sand
    Suna,
    /// Kirigakure - Village Hidden in the Mist
    Kiri,
    /// Kumogakure - Village Hidden in the Clouds
    Kumo,
    /// Iwagakure - Village Hidden in the Stones
    Iwa,
    /// Amegakure - Village Hidden in the Rain
    Ame,
}

/// This enum represent the elements in Naruto.
/// The elements are: Fire, Wind, Lightning, Earth and Water.
/// The elements are used in jutsus.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Element {
    /// Fire element
    Fire,
    /// Wind element
    Wind,
    /// Lightning element
    Lightning,
    /// Earth element
    Earth,
    /// Water element
    Water,
}

pub struct Human<'a>(&'a NarutoChar);

/// A humanoid creature in the Star Wars universe.
#[Object]
impl<'a> Human<'a> {
    /// The id of the human.
    async fn id(&self) -> &str {
        self.0.id
    }

    /// The name of the human.
    async fn name(&self) -> &str {
        self.0.name
    }

    /// The friends of the human, or an empty list if they have none.
    async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        let naruto_shippuden = ctx.data_unchecked::<NarutoShippuden>();
        naruto_shippuden
            .friends(self.0)
            .into_iter()
            .map(|ch| Human(ch).into())
            .collect()
    }

    /// Which element the human is affiliated with.
    async fn elements(&self) -> &[Element] {
        &self.0.elements
    }

    /// The village this human is from, or null if unknown.
    /// This is the village this human was born in or primarily works in.
    async fn village(&self) -> &Village {
        &self.0.village
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "If omitted, returns the the hero of the whole saga. If provided, returns the hero of that particular village."
        )]
        village: Option<Village>,
    ) -> Character<'a> {
        let naruto_shippuden = ctx.data_unchecked::<NarutoShippuden>();
        match village {
            Some(village) => match village {
                Village::Konoha => {
                    Human(naruto_shippuden.chars.get(naruto_shippuden.naruto).unwrap()).into()
                }
                Village::Iwa => {
                    Human(naruto_shippuden.chars.get(naruto_shippuden.gara).unwrap()).into()
                }
                Village::Ame => {
                    Human(naruto_shippuden.chars.get(naruto_shippuden.pain).unwrap()).into()
                }
                _ => Human(naruto_shippuden.chars.get(naruto_shippuden.naruto).unwrap()).into(),
            },
            None => Human(naruto_shippuden.chars.get(naruto_shippuden.naruto).unwrap()).into(),
        }
    }

    async fn human<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the human")] id: String,
    ) -> Option<Human<'a>> {
        ctx.data_unchecked::<NarutoShippuden>()
            .human(&id)
            .map(Human)
    }

    async fn humans<'a>(
        &self,
        ctx: &Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Human<'a>>> {
        let humans = ctx.data_unchecked::<NarutoShippuden>().humans().to_vec();
        query_characters(after, before, first, last, &humans, Human).await
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&str"),
    field(name = "name", type = "&str"),
    field(name = "elements", type = "&[Element]"),
    field(name = "village", type = "&Village"),
    field(name = "friends", type = "Vec<Character<'ctx>>")
)]
pub enum Character<'a> {
    Human(Human<'a>),
}

async fn query_characters<'a, F, T>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: &[&'a NarutoChar],
    map_to: F,
) -> Result<Connection<usize, T>>
where
    F: Fn(&'a NarutoChar) -> T,
    T: OutputType,
{
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();

            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &characters[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < characters.len());
            connection.edges.extend(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, (map_to)(item))),
            );
            Ok::<_, Error>(connection)
        },
    )
    .await
}

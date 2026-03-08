use spacetimedb::{reducer, table, ReducerContext, Table};

#[table(accessor = links, public)]
pub struct Link {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub source: u64,
    #[index(btree)]
    pub target: u64,
}

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    log::info!("SpacetimeDB links module initialized");
}

#[reducer]
pub fn create_link(ctx: &ReducerContext, source: u64, target: u64) {
    ctx.db.links().insert(Link {
        id: 0,
        source,
        target,
    });
}

#[reducer]
pub fn update_link(ctx: &ReducerContext, id: u64, source: u64, target: u64) {
    if let Some(link) = ctx.db.links().id().find(&id) {
        ctx.db.links().id().update(Link {
            id: link.id,
            source,
            target,
        });
    }
}

#[reducer]
pub fn delete_link(ctx: &ReducerContext, id: u64) {
    ctx.db.links().id().delete(&id);
}

#[reducer]
pub fn delete_all_links(ctx: &ReducerContext) {
    let ids: Vec<u64> = ctx.db.links().iter().map(|l| l.id).collect();
    for id in ids {
        ctx.db.links().id().delete(&id);
    }
}

//! Differential checks for every benchmark backend. Server tests run explicitly in CI.
use spacetimedb_vs_doublets::{doublets_impl::*, spacetimedb_impl::SpacetimeDbLinks, Link, Links};

fn sorted(mut links: Vec<Link>) -> Vec<Link> {
    links.sort_by_key(|link| (link.id, link.source, link.target));
    links
}

// IDs are auto-incremented by SpacetimeDB and reused by Doublets. Compare identities
// by creation order, while preserving source/target relationships between links.
fn exercise(db: &mut dyn Links) -> Vec<Vec<Link>> {
    db.delete_all();
    let ids: Vec<_> = (0..12).map(|_| db.create_point()).collect();
    let normalize = |links: Vec<Link>| {
        let identity = |id| {
            ids.iter()
                .position(|&candidate| candidate == id)
                .map_or(id, |i| i as u64 + 1)
        };
        sorted(
            links
                .into_iter()
                .map(|link| {
                    Link::new(
                        identity(link.id),
                        identity(link.source),
                        identity(link.target),
                    )
                })
                .collect(),
        )
    };
    assert_eq!(db.count(), 12);
    let mut trace = vec![normalize(db.query_all())];
    for &id in &ids {
        let point = Link::new(id, id, id);
        assert_eq!(db.query_by_id(id), Some(point));
        assert_eq!(db.query_by_source(id), vec![point]);
        assert_eq!(db.query_by_target(id), vec![point]);
    }
    for (i, &id) in ids.iter().enumerate() {
        if std::env::var_os("BENCHMARK_TRACE").is_some() {
            eprintln!("distributed update {id}");
        }
        db.update(id, ids[i % 3], ids[i % 5]);
    }
    trace.push(normalize(db.query_all()));
    for &id in &ids {
        trace.push(normalize(db.query_by_id(id).into_iter().collect()));
        trace.push(normalize(db.query_by_source(id)));
        trace.push(normalize(db.query_by_target(id)));
        trace.push(normalize(db.query_by_source_target(id, ids[0])));
    }
    assert_eq!(db.query_by_id(u64::MAX - 1), None);
    assert!(db.query_by_source(99999).is_empty());
    assert!(db.query_by_target(99999).is_empty());
    // The benchmark updates each point to (0, 0), then restores it.
    for &id in &ids {
        if std::env::var_os("BENCHMARK_TRACE").is_some() {
            eprintln!("reset update {id}");
        }
        db.update(id, 0, 0);
    }
    trace.push(normalize(db.query_all()));
    for &id in &ids {
        db.update(id, id, id);
    }
    for &id in ids.iter().rev().take(6) {
        db.delete(id);
    }
    assert_eq!(db.count(), 6);
    trace.push(normalize(db.query_all()));
    db.delete_all();
    assert_eq!(db.count(), 0);
    assert!(db.query_all().is_empty());
    trace
}

fn distributed_queries(db: &mut dyn Links) -> Vec<Vec<Link>> {
    let mut trace = Vec::new();
    // Two repetitions exercise ID sequences that do not restart at one.
    for _ in 0..2 {
        for by_source in [true, false] {
            db.delete_all();
            let mut ids: Vec<u64> = (0..30).map(|_| db.create_point()).collect();
            let background = ids.clone();
            for i in 0..100 {
                let (source, target) =
                    spacetimedb_vs_doublets::workload::distributed_link(i, &background, by_source);
                ids.push(db.create(source, target));
            }
            assert_eq!(db.count(), 130);
            for &key in &background[..10] {
                let rows = if by_source {
                    db.query_by_source(key)
                } else {
                    db.query_by_target(key)
                };
                assert_eq!(rows.len(), 11); // one point plus ten distributed links
                let identity = |id| ids.iter().position(|&value| value == id).unwrap() as u64 + 1;
                trace.push(sorted(
                    rows.into_iter()
                        .map(|row| {
                            Link::new(identity(row.id), identity(row.source), identity(row.target))
                        })
                        .collect(),
                ));
            }
        }
    }
    db.delete_all();
    trace
}

fn full_trace(db: &mut dyn Links) -> Vec<Vec<Link>> {
    let mut trace = exercise(db);
    trace.extend(distributed_queries(db));
    trace
}

fn doublets_traces() -> Vec<Vec<Vec<Link>>> {
    let directory = std::env::temp_dir().join(format!("same-behavior-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let path = |name| directory.join(name).to_str().unwrap().to_string();
    let traces = vec![
        full_trace(&mut create_united_volatile()),
        full_trace(&mut create_split_volatile()),
        full_trace(&mut create_united_non_volatile(&path("united"))),
        full_trace(&mut create_split_non_volatile(
            &path("data"),
            &path("index"),
        )),
    ];
    std::fs::remove_dir_all(directory).unwrap();
    traces
}

#[test]
fn all_doublets_variants_return_same_results() {
    let traces = doublets_traces();
    for trace in &traces[1..] {
        assert_eq!(trace, &traces[0]);
    }
}

#[test]
#[ignore = "requires a published SpacetimeDB module; CI runs with --include-ignored"]
fn spacetimedb_and_all_doublets_variants_return_same_results() {
    let actual = full_trace(&mut SpacetimeDbLinks::connect());
    for trace in doublets_traces() {
        assert_eq!(trace, actual);
    }
}

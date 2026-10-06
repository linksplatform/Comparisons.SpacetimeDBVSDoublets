using Comparisons.SpacetimeDBVSDoublets;
using Xunit;

namespace Comparisons.SpacetimeDBVSDoublets.Tests;

public sealed class SameBehaviorTests
{
    private static List<Link[]> Exercise(ILinks store)
    {
        store.DeleteAll();
        var ids = Enumerable.Range(0, 12).Select(_ => store.CreatePoint()).ToArray();
        ulong Normalize(ulong id) => Array.IndexOf(ids, id) is var index && index >= 0 ? (ulong)index + 1 : id;
        Link[] Snapshot(IEnumerable<Link> rows) => rows.Select(row => new Link(Normalize(row.Id), Normalize(row.Source), Normalize(row.Target)))
            .OrderBy(row => row.Id).ToArray();
        Assert.Equal(12UL, store.Count());
        var trace = new List<Link[]> { Snapshot(store.QueryAll()) };
        foreach (var id in ids)
        {
            var point = new Link(id, id, id);
            Assert.Equal(point, store.QueryById(id));
            Assert.Equal([point], store.QueryBySource(id));
            Assert.Equal([point], store.QueryByTarget(id));
        }
        for (var i = 0; i < ids.Length; i++) store.Update(ids[i], ids[i % 3], ids[i % 5]);
        trace.Add(Snapshot(store.QueryAll()));
        foreach (var id in ids)
        {
            trace.Add(Snapshot([store.QueryById(id)!.Value]));
            trace.Add(Snapshot(store.QueryBySource(id)));
            trace.Add(Snapshot(store.QueryByTarget(id)));
            trace.Add(Snapshot(store.QueryBySourceTarget(id, ids[0])));
        }
        Assert.Null(store.QueryById(ulong.MaxValue - 1));
        Assert.Empty(store.QueryBySource(99999));
        Assert.Empty(store.QueryByTarget(99999));
        foreach (var id in ids) store.Update(id, 0, 0);
        trace.Add(Snapshot(store.QueryAll()));
        foreach (var id in ids) store.Update(id, id, id);
        foreach (var id in ids.Reverse().Take(6)) store.Delete(id);
        Assert.Equal(6UL, store.Count());
        trace.Add(Snapshot(store.QueryAll()));
        store.DeleteAll();
        Assert.Empty(store.QueryAll());
        Assert.Equal(0UL, store.Count());
        return trace;
    }

    private static List<Link[]> DistributedQueries(ILinks store)
    {
        var trace = new List<Link[]>();
        for (var repetition = 0; repetition < 2; repetition++)
            foreach (var bySource in new[] { true, false })
            {
                store.DeleteAll();
                var ids = Enumerable.Range(0, 30).Select(_ => store.CreatePoint()).ToList();
                var background = ids.ToArray();
                for (var i = 0; i < 100; i++)
                {
                    var pair = Benchmarks.DistributedLink(i, background, bySource);
                    ids.Add(store.Create(pair.Source, pair.Target));
                }
                Assert.Equal(130UL, store.Count());
                foreach (var key in background.Take(10))
                {
                    var rows = bySource ? store.QueryBySource(key) : store.QueryByTarget(key);
                    Assert.Equal(11, rows.Length);
                    ulong Identity(ulong id) => (ulong)ids.IndexOf(id) + 1;
                    trace.Add(rows.Select(row => new Link(Identity(row.Id), Identity(row.Source), Identity(row.Target))).OrderBy(row => row.Id).ToArray());
                }
            }
        store.DeleteAll();
        return trace;
    }

    private static List<Link[]> FullTrace(ILinks store)
    {
        var trace = Exercise(store);
        trace.AddRange(DistributedQueries(store));
        return trace;
    }

    [Fact]
    public void AllBackendsReturnSameResults()
    {
        var directory = Path.Combine(Path.GetTempPath(), $"same-behavior-{Guid.NewGuid()}");
        Directory.CreateDirectory(directory);
        try
        {
            List<Link[]>? expected = null;
            foreach (var variant in DoubletsLinks.Variants)
            {
                using var store = DoubletsLinks.Open(variant, directory);
                var actual = FullTrace(store);
                expected ??= actual;
                for (var i = 0; i < expected.Count; i++) Assert.Equal(expected[i], actual[i]);
            }
            if (Environment.GetEnvironmentVariable("SPACETIMEDB_URI") is not null)
            {
                using var store = new SpacetimeLinks();
                var actual = FullTrace(store);
                for (var i = 0; i < expected!.Count; i++) Assert.Equal(expected[i], actual[i]);
            }
        }
        finally { Directory.Delete(directory, true); }
    }

    [Fact]
    public void EveryBenchmarkIterationResetsTheStore()
    {
        var directory = Path.Combine(Path.GetTempPath(), $"iterations-{Guid.NewGuid()}");
        Directory.CreateDirectory(directory);
        try
        {
            var variants = DoubletsLinks.Variants.Concat(Environment.GetEnvironmentVariable("SPACETIMEDB_URI") is null
                ? Array.Empty<string>() : ["SpacetimeDB"]);
            foreach (var variant in variants)
            {
                using ILinks store = variant == "SpacetimeDB" ? new SpacetimeLinks() : DoubletsLinks.Open(variant, directory);
                store.DeleteAll();
                foreach (var operation in Benchmarks.Operations)
                    for (var i = 0; i < 2; i++)
                    {
                        Benchmarks.Iteration(store, operation, 10, 30);
                        Assert.Equal(0UL, store.Count());
                    }
            }
        }
        finally { Directory.Delete(directory, true); }
    }
}

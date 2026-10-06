using System.Diagnostics;

namespace Comparisons.SpacetimeDBVSDoublets;

/// <summary>Mirrors rust/benches/bench.rs, including the setup and reset outside measurement.</summary>
public static class Benchmarks
{
    public static readonly string[] Operations =
        ["create", "update", "delete", "query_all", "query_by_id", "query_by_source", "query_by_target"];

    /// <summary>Unique pairs referencing actual background IDs; matches rust/src/workload.rs.</summary>
    public static (ulong Source, ulong Target) DistributedLink(int index, ulong[] background, bool bySource)
    {
        var key = background[index % 10];
        var other = background[index / 10 + 10];
        return bySource ? (key, other) : (other, key);
    }

    public static TimeSpan Iteration(ILinks store, string operation, int count, int background)
    {
        var backgroundIds = Enumerable.Range(0, background).Select(_ => store.CreatePoint()).ToArray();
        if (operation is "query_by_source" or "query_by_target" && background < 10 + (count + 9) / 10)
            throw new ArgumentException("Query workloads require at least 10 + ceil(N/10) background links");
        var ids = new List<ulong>();
        if (operation is "update" or "delete" or "query_all" or "query_by_id")
            for (var i = 0; i < count; i++) ids.Add(store.CreatePoint());
        if (operation == "query_by_source")
            for (ulong i = 1; i <= (ulong)count; i++) { var pair = DistributedLink((int)i - 1, backgroundIds, true); store.Create(pair.Source, pair.Target); }
        if (operation == "query_by_target")
            for (ulong i = 1; i <= (ulong)count; i++) { var pair = DistributedLink((int)i - 1, backgroundIds, false); store.Create(pair.Source, pair.Target); }
        try
        {
            var started = Stopwatch.GetTimestamp();
            switch (operation)
            {
                case "create":
                    for (var i = 0; i < count; i++) store.CreatePoint();
                    break;
                case "update":
                    foreach (var id in ids) store.Update(id, 0, 0);
                    foreach (var id in ids) store.Update(id, id, id);
                    break;
                case "delete":
                    foreach (var id in ids) store.Delete(id);
                    break;
                case "query_all":
                    GC.KeepAlive(store.QueryAll());
                    break;
                case "query_by_id":
                    foreach (var id in ids) GC.KeepAlive(store.QueryById(id));
                    break;
                case "query_by_source":
                    foreach (var source in backgroundIds.Take(Math.Min(count, 10))) GC.KeepAlive(store.QueryBySource(source));
                    break;
                case "query_by_target":
                    foreach (var target in backgroundIds.Take(Math.Min(count, 10))) GC.KeepAlive(store.QueryByTarget(target));
                    break;
                default: throw new ArgumentException($"Unknown operation {operation}");
            }
            return Stopwatch.GetElapsedTime(started);
        }
        finally { store.DeleteAll(); }
    }
}
